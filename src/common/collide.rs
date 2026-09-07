//! 通用 AABB 地形解算：脱困 + 分轴推进。
//!
//! 纯函数、零 ECS 依赖，所以能 headless 单测 —— 这是把「撞墙 / 卡住 / 横向瞬移」
//! 这类问题挡在回归测试外面的唯一办法。
//!
//! # 为什么要先脱困
//!
//! 分轴解算（先动 X 解 X、再动 Y 解 Y）只有在**帧初该轴未嵌入**时才正确。
//! 一旦角色 Y 方向嵌进地面哪怕 1 单位，X 轴的穿透深度就是「半宽之和 − |dx|」，
//! 对一块比角色宽得多的地砖来说是几十上百单位 —— 角色会被横向弹飞。
//!
//! 与其指望调用方永远不把角色塞进地形里（变身长高、移动平台上升、地形接缝错位
//! 都会破坏它），不如每帧先跑一遍 [`depenetrate`] 把不变式重新建立起来。

use bevy::math::Vec2;

/// 位置比较容差（世界单位）。恰好接触不算重叠，与各游戏原有的 `aabb_overlap` 一致。
const EPS: f32 = 1e-3;

/// 脱困最多迭代几轮：推出一块可能撞进相邻块，而地砖是逐格 spawn 的。
const DEPEN_PASSES: usize = 4;

/// 近似平局时优先竖直顶出。角色几乎总是「踩进地面」而不是「插进墙里」，
/// 而地形块往往比角色宽，走 X 轴会把几单位的下沉解成几十单位的横移。
const VERTICAL_BIAS: f32 = 1.0;

/// 一块地形碰撞体（中心锚点，与全项目现有的 `(pos, size)` 约定一致）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Solid {
    pub center: Vec2,
    pub size: Vec2,
    /// 本帧这块 solid 自身的位移。静止地形填 `Vec2::ZERO`；移动平台填它这一帧
    /// 走过的距离，调用方拿到 [`Resolved::ground`] 后据此把站在上面的角色带走。
    pub delta: Vec2,
}

impl Solid {
    /// 静止地形。
    pub const fn fixed(center: Vec2, size: Vec2) -> Self {
        Self { center, size, delta: Vec2::ZERO }
    }

    /// 移动平台：`delta` 是它本帧已经走过的位移。
    pub const fn moving(center: Vec2, size: Vec2, delta: Vec2) -> Self {
        Self { center, size, delta }
    }
}

/// 一次解算的结果。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Resolved {
    pub pos: Vec2,
    pub vel: Vec2,
    pub on_ground: bool,
    pub hit_wall: bool,
    /// 落脚的那块 solid 下标；调用方据此取 `delta` 做移动平台承载。
    pub ground: Option<usize>,
    /// 顶到的那块 solid 下标；马里奥顶砖块靠它。
    pub ceiling: Option<usize>,
}

/// 严格小于：恰好接触不算重叠。
pub fn overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

/// 把已经嵌进地形的盒子沿**最小穿透轴**推出；没有嵌入时原样返回，
/// 所以可以无条件每帧调用。
pub fn depenetrate(pos: Vec2, size: Vec2, solids: &[Solid]) -> Vec2 {
    let mut p = pos;
    for _ in 0..DEPEN_PASSES {
        let mut moved = false;
        for s in solids {
            let d = p - s.center;
            let px = (size.x + s.size.x) * 0.5 - d.x.abs();
            let py = (size.y + s.size.y) * 0.5 - d.y.abs();
            if px <= 0.0 || py <= 0.0 {
                continue;
            }
            if py <= px + VERTICAL_BIAS {
                p.y += if d.y >= 0.0 { py } else { -py };
            } else {
                p.x += if d.x >= 0.0 { px } else { -px };
            }
            moved = true;
        }
        if !moved {
            break;
        }
    }
    p
}

/// 推进一步并解算地形碰撞。
///
/// 顺序固定为：脱困 → 动 X 解 X → 动 Y 解 Y。
///
/// 推出方向由**帧初在哪一侧**决定，而不是由「当前中心在哪一侧」决定 —— 后者会让
/// 角色在接近跳跃顶点、身体中心刚越过薄平台中心时被瞬间弹到平台上面去。
/// 因为脱困已经保证帧初无重叠，某块 solid 若在移动后才重叠，它在该轴上帧初
/// 必然是分离的，所以两个方向判定里必有一个成立；`d` 分支只是兜底。
pub fn resolve(prev: Vec2, size: Vec2, vel: Vec2, dt: f32, solids: &[Solid]) -> Resolved {
    let half = size * 0.5;
    let mut out = Resolved {
        pos: depenetrate(prev, size, solids),
        vel,
        ..Default::default()
    };

    // ---- X 轴 ----
    let x0 = out.pos.x;
    out.pos.x += out.vel.x * dt;
    for s in solids {
        if !overlap(out.pos, size, s.center, s.size) {
            continue;
        }
        let s_half = s.size * 0.5;
        let from_left = x0 + half.x <= s.center.x - s_half.x + EPS;
        let from_right = x0 - half.x >= s.center.x + s_half.x - EPS;
        let push_left = from_left || (!from_right && out.pos.x < s.center.x);
        out.pos.x = if push_left {
            s.center.x - s_half.x - half.x
        } else {
            s.center.x + s_half.x + half.x
        };
        out.vel.x = 0.0;
        out.hit_wall = true;
    }

    // ---- Y 轴 ----
    let y0 = out.pos.y;
    out.pos.y += out.vel.y * dt;
    for (i, s) in solids.iter().enumerate() {
        if !overlap(out.pos, size, s.center, s.size) {
            continue;
        }
        let s_half = s.size * 0.5;
        let from_above = y0 - half.y >= s.center.y + s_half.y - EPS;
        let from_below = y0 + half.y <= s.center.y - s_half.y + EPS;
        if from_above || (!from_below && out.pos.y > s.center.y) {
            out.pos.y = s.center.y + s_half.y + half.y;
            // 只有向下运动才算落地：上升途中蹭到砖块顶角不该重置跳跃与土狼时间
            if out.vel.y <= 0.0 {
                out.on_ground = true;
                out.ground = Some(i);
            }
            if out.vel.y < 0.0 {
                out.vel.y = 0.0;
            }
        } else {
            out.pos.y = s.center.y - s_half.y - half.y;
            out.ceiling = Some(i);
            if out.vel.y > 0.0 {
                out.vel.y = 0.0;
            }
        }
    }

    out
}

/// 半隐式欧拉（先加重力再位移，与 `FixedUpdate` 里的物理系统一致）下的实际跳跃升幅。
///
/// 解析解 `v²/2g` 会比离散结果高几个单位，而「这块平台够不够得着」正好卡在那几个
/// 单位上，所以关卡可达性断言必须用这个函数而不是解析解。
pub fn apex_height(jump_vel: f32, gravity: f32, dt: f32) -> f32 {
    debug_assert!(gravity > 0.0 && dt > 0.0);
    let mut v = jump_vel;
    let mut y = 0.0f32;
    let mut peak = 0.0f32;
    while v > 0.0 {
        v -= gravity * dt;
        y += v * dt;
        peak = peak.max(y);
    }
    peak
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;

    /// 马里奥实际数值：地砖 36、小马里奥 30×42、大马里奥 32×66。
    fn floor_row(cols: i32) -> Vec<Solid> {
        (-cols..=cols)
            .map(|c| Solid::fixed(Vec2::new(c as f32 * 36.0, 0.0), Vec2::splat(36.0)))
            .collect()
    }

    /// 地砖顶面 = 18，角色恰好站在上面时的中心 y。
    fn standing_y(height: f32) -> f32 {
        18.0 + height * 0.5
    }

    #[test]
    fn walking_never_moves_further_than_velocity_allows() {
        // 「横向瞬移」整类 bug 的总闸：帧初不嵌入时，单帧位移不可能超过 |v·dt|。
        let size = Vec2::new(30.0, 42.0);
        let solids = floor_row(6);
        let mut pos = Vec2::new(-100.0, standing_y(size.y));
        let vel = Vec2::new(150.0, -10.0);
        for _ in 0..120 {
            let prev = pos;
            let r = resolve(pos, size, vel, DT, &solids);
            let step = (r.pos.x - prev.x).abs();
            assert!(
                step <= vel.x.abs() * DT + EPS,
                "单帧横移 {step} 超过了 {}",
                vel.x.abs() * DT
            );
            assert!(r.on_ground, "沿平地行走应始终判定为落地");
            pos = r.pos;
        }
    }

    #[test]
    fn growing_into_the_floor_lifts_instead_of_shoving_sideways() {
        // T1：马里奥吃蘑菇长高 42 → 66，中心不动 ⇒ 脚底下沉 12 埋进地砖。
        // 旧解算会按 X 穿透 (32+36)/2 = 34 把人横推 34 单位。
        let big = Vec2::new(32.0, 66.0);
        let solids = floor_row(6);
        let sunken = Vec2::new(0.0, standing_y(42.0)); // 仍是小马里奥的站立中心
        let fixed = depenetrate(sunken, big, &solids);
        assert!((fixed.x - sunken.x).abs() < EPS, "脱困不应产生任何横移");
        assert!(
            (fixed.y - standing_y(big.y)).abs() < EPS,
            "应向上顶出 12 单位回到地面，实际 y = {}",
            fixed.y
        );
    }

    #[test]
    fn a_one_unit_seam_lifts_the_body_instead_of_pinning_it() {
        // T3：魂斗罗木桥顶面比陆地低 1 单位，站在桥上走向岸边时与陆地重叠 1 单位。
        // 旧解算会按 X 穿透把人每帧推回岸边，形成隐形墙。
        let size = Vec2::new(18.0, 32.0); // 魂斗罗玩家
        let plank = Solid::fixed(Vec2::new(1190.0, -15.0), Vec2::new(180.0, 12.0));
        let ground = Solid::fixed(Vec2::new(1780.0, -108.0), Vec2::new(1000.0, 200.0));
        let solids = vec![plank, ground];

        let on_plank = Vec2::new(1275.0, -15.0 + (32.0 + 12.0) * 0.5);
        let fixed = depenetrate(on_plank, size, &solids);
        assert!((fixed.x - on_plank.x).abs() < EPS, "脱困不应产生横移");
        assert!((fixed.y - on_plank.y - 1.0).abs() < EPS, "应向上顶出 1 单位");

        // 顶面齐平后，继续朝岸上走应当单调推进。
        let mut pos = fixed;
        for _ in 0..40 {
            let prev_x = pos.x;
            let r = resolve(pos, size, Vec2::new(200.0, -10.0), DT, &solids);
            assert!(r.pos.x > prev_x, "走过接缝时不应被推回");
            pos = r.pos;
        }
    }

    #[test]
    fn a_rising_platform_carries_instead_of_ejecting() {
        // T2：平台先于物理上移 80/60 单位，玩家次帧嵌入。
        let size = Vec2::new(30.0, 42.0);
        let rise = 80.0 * DT;
        let platform = Solid::moving(Vec2::ZERO, Vec2::new(108.0, 12.0), Vec2::new(0.0, rise));
        let solids = vec![platform];

        // 玩家还站在平台上移之前的位置
        let stale = Vec2::new(0.0, 6.0 - rise + 21.0);
        let r = resolve(stale, size, Vec2::new(0.0, -700.0), DT, &solids);
        assert!((r.pos.x - stale.x).abs() < EPS, "站在上升平台上不应被横向弹开");
        assert_eq!(r.ground, Some(0), "应识别出落脚的平台，供调用方做承载");
        assert!(r.on_ground);
    }

    #[test]
    fn a_one_tile_corridor_is_passable() {
        // 通道 36 宽、角色 30 宽，单侧余量 3：必须能走通。
        let size = Vec2::new(30.0, 42.0);
        let solids = vec![
            Solid::fixed(Vec2::new(-36.0, 0.0), Vec2::splat(36.0)),
            Solid::fixed(Vec2::new(36.0, 0.0), Vec2::splat(36.0)),
        ];
        let mut pos = Vec2::new(0.0, -200.0);
        for _ in 0..200 {
            let r = resolve(pos, size, Vec2::new(0.0, 260.0), DT, &solids);
            pos = r.pos;
        }
        assert!(pos.y > 200.0, "角色应当穿过通道，实际停在 y = {}", pos.y);
    }

    #[test]
    fn grazing_a_ceiling_while_rising_is_not_a_landing() {
        // 旧解算里 `dy > 0` 就无条件置 on_ground，上升途中蹭砖块会白送二段跳。
        let size = Vec2::new(30.0, 42.0);
        let solids = vec![Solid::fixed(Vec2::ZERO, Vec2::splat(36.0))];
        let below = Vec2::new(0.0, -18.0 - 21.0);
        let r = resolve(below, size, Vec2::new(0.0, 520.0), DT, &solids);
        assert!(!r.on_ground, "上升撞到砖块底面不是落地");
        assert_eq!(r.ceiling, Some(0), "应报告被顶到的砖块，供顶砖逻辑使用");
        assert_eq!(r.vel.y, 0.0);
    }

    #[test]
    fn rising_into_a_thin_platform_bonks_instead_of_popping_on_top() {
        // 魂斗罗的关卡当初就是照着「穿透弹射」摆的：身体中心越过薄平台中心时，
        // 旧解算判 dy>0 把人向上瞬移到平台上面。修好后应当是撞头。
        let size = Vec2::new(18.0, 32.0);
        let ground_top = -8.0;
        // spawn_platform 传入的 center 就是顶面，solid 放在 center.y - h/2
        let top = ground_top + 24.0 * 3.5;
        let solids = vec![Solid::fixed(Vec2::new(0.0, top - 8.0), Vec2::new(192.0, 16.0))];

        let mut pos = Vec2::new(0.0, ground_top + 16.0);
        let mut vel = Vec2::new(0.0, 480.0);
        for _ in 0..60 {
            vel.y -= 1500.0 * DT;
            let r = resolve(pos, size, vel, DT, &solids);
            pos = r.pos;
            vel = r.vel;
            assert!(
                pos.y - 16.0 < top - EPS,
                "脚底 {} 不该越过平台顶面 {top}（穿透弹射复发）",
                pos.y - 16.0
            );
        }
    }

    #[test]
    fn discrete_apex_is_lower_than_the_analytic_one() {
        // 关卡可达性必须用离散升幅判定：解析解 480²/(2·1500) = 76.8，
        // 而半隐式欧拉实际只有 72.83 —— 魂斗罗最低平台正好卡在这 4 个单位里。
        let apex = apex_height(480.0, 1500.0, DT);
        assert!((apex - 72.83).abs() < 0.05, "实际升幅 {apex}");
        assert!(apex < 480.0 * 480.0 / (2.0 * 1500.0));
    }

    #[test]
    fn depenetrate_is_a_no_op_when_resting_exactly_on_top() {
        let size = Vec2::new(30.0, 42.0);
        let solids = floor_row(3);
        let resting = Vec2::new(0.0, standing_y(size.y));
        assert_eq!(depenetrate(resting, size, &solids), resting);
    }
}
