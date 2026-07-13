use bevy::math::Vec2;

use super::components::AimDir;
use super::constants::{BOSS_CORE_SIZE, PLAYER_H, PLAYER_W, PRONE_H};

pub fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

pub fn player_size(prone: bool) -> Vec2 {
    Vec2::new(PLAYER_W, if prone { PRONE_H } else { PLAYER_H })
}

pub fn muzzle_offset(prone: bool, aim: AimDir, facing: f32) -> Vec2 {
    let body_h = if prone { PRONE_H } else { PLAYER_H };
    let mid_y = if prone { 0.0 } else { 4.0 };
    let dir = aim.vec();
    let half = body_h * 0.5;
    Vec2::new(
        dir.x * (PLAYER_W * 0.5 + 8.0) + if dir.x.abs() < 0.01 { facing * 2.0 } else { 0.0 },
        dir.y * (half + 6.0) + mid_y,
    )
}

pub fn resolve_player_aim(movement: Vec2, on_ground: bool, facing: f32) -> AimDir {
    let left = movement.x < 0.0;
    let right = movement.x > 0.0;
    let up = movement.y > 0.0;
    let down = movement.y < 0.0;
    if up && left {
        AimDir::UpLeft
    } else if up && right {
        AimDir::UpRight
    } else if up {
        AimDir::Up
    } else if down && !on_ground && left {
        AimDir::DownLeft
    } else if down && !on_ground && right {
        AimDir::DownRight
    } else if down && !on_ground {
        AimDir::Down
    } else if left {
        AimDir::Left
    } else if right || facing >= 0.0 {
        AimDir::Right
    } else {
        AimDir::Left
    }
}

/// 把任意方向量化为 FC 风格的八方向单位向量，避免敌弹像素角度过细。
pub fn quantize_direction_8(dir: Vec2) -> Vec2 {
    if dir.length_squared() <= f32::EPSILON {
        return Vec2::NEG_X;
    }
    let ax = dir.x.abs();
    let ay = dir.y.abs();
    const AXIS_RATIO: f32 = 2.414_213_7;
    if ax > ay * AXIS_RATIO {
        Vec2::new(dir.x.signum(), 0.0)
    } else if ay > ax * AXIS_RATIO {
        Vec2::new(0.0, dir.y.signum())
    } else {
        Vec2::new(dir.x.signum(), dir.y.signum()).normalize()
    }
}

pub fn boss_core_overlap(projectile_pos: Vec2, projectile_size: Vec2, boss_pos: Vec2) -> bool {
    aabb_overlap(
        projectile_pos,
        projectile_size,
        boss_pos,
        Vec2::splat(BOSS_CORE_SIZE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_basic() {
        let s = Vec2::splat(10.0);
        assert!(aabb_overlap(Vec2::ZERO, s, Vec2::new(5.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(20.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(10.0, 0.0), s));
    }

    #[test]
    fn player_size_shrinks_when_prone() {
        let stand = player_size(false);
        let prone = player_size(true);
        assert_eq!(stand.x, prone.x);
        assert!(prone.y < stand.y);
    }

    #[test]
    fn muzzle_extends_along_aim() {
        // 朝右站立：muzzle 应在身体右侧外
        let m = muzzle_offset(false, AimDir::Right, 1.0);
        assert!(m.x > PLAYER_W * 0.5);
        // 朝上：muzzle x 应几乎为 0，y 在身体上方
        let m = muzzle_offset(false, AimDir::Up, 1.0);
        assert!(m.x.abs() < 5.0);
        assert!(m.y > PLAYER_H * 0.5);
    }

    #[test]
    fn grounded_down_keeps_horizontal_aim_but_airborne_down_aims_down() {
        assert_eq!(
            resolve_player_aim(Vec2::NEG_Y, true, -1.0),
            AimDir::Left
        );
        assert_eq!(
            resolve_player_aim(Vec2::NEG_Y, false, -1.0),
            AimDir::Down
        );
        assert_eq!(
            resolve_player_aim(Vec2::new(1.0, -1.0), false, 1.0),
            AimDir::DownRight
        );
    }

    #[test]
    fn enemy_aim_is_quantized_to_eight_directions() {
        assert_eq!(quantize_direction_8(Vec2::new(10.0, 1.0)), Vec2::X);
        assert_eq!(quantize_direction_8(Vec2::new(-1.0, 10.0)), Vec2::Y);
        let diagonal = quantize_direction_8(Vec2::new(-5.0, -4.0));
        assert!((diagonal.length() - 1.0).abs() < 1e-5);
        assert!(diagonal.x < 0.0 && diagonal.y < 0.0);
    }

    #[test]
    fn only_the_boss_core_is_a_weak_point() {
        let boss = Vec2::new(100.0, 100.0);
        assert!(boss_core_overlap(boss, Vec2::splat(6.0), boss));
        assert!(!boss_core_overlap(
            boss + Vec2::new(80.0, 0.0),
            Vec2::splat(6.0),
            boss
        ));
    }
}
