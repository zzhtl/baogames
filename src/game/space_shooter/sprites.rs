//! 太空射击精灵的共享定义（游戏渲染 + 离线预览同源）。
//!
//! 玩家战机**朝上**（机首 +y），敌机**朝下**（机首 -y，扑向下方的玩家）。
//! 方向固定、无旋转。`cargo run --bin preview -- space player` 出预览图。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::palette::*;

/// 玩家蓝色喷气战斗机（朝上）。包络 18×30。
pub const SHIP_PLAYER: SpriteDef = SpriteDef {
    size: Vec2::new(18.0, 30.0),
    parts: parts![
        // 后掠翼（靠下更宽）
        (0.0, -5.0, 30.0, 6.0, P_WING, 0.04),
        (0.0, -3.2, 26.0, 1.6, P_BODY_HI, 0.043), // 翼前缘受光
        (0.0, -8.5, 22.0, 4.0, P_WING_DK, 0.045),
        (-13.0, -6.0, 5.0, 5.0, P_WING, 0.04),
        (13.0, -6.0, 5.0, 5.0, P_WING, 0.04),
        // 机身
        (0.0, 1.0, 9.0, 24.0, P_BODY, 0.05),
        (0.0, 12.0, 5.0, 8.0, P_BODY, 0.05),
        (0.0, 15.0, 3.0, 5.0, P_NOSE, 0.07),
        (-2.5, 2.0, 2.0, 18.0, P_BODY_HI, 0.055),
        // 座舱
        (0.0, 5.0, 5.0, 7.0, P_CANOPY, 0.07),
        (0.0, 6.5, 3.0, 3.0, P_CANOPY_HI, 0.075),
        // 尾喷
        (-3.0, -13.0, 4.0, 5.0, P_THRUST, 0.06),
        (3.0, -13.0, 4.0, 5.0, P_THRUST, 0.06),
        (-3.0, -15.5, 3.0, 3.0, P_THRUST_HI, 0.065),
        (3.0, -15.5, 3.0, 3.0, P_THRUST_HI, 0.065),
        // 描边
        (-5.0, 2.0, 1.0, 24.0, OUTLINE, 0.03),
        (5.0, 2.0, 1.0, 24.0, OUTLINE, 0.03),
        (0.0, -10.8, 30.0, 1.0, OUTLINE, 0.03),
    ],
};

/// 侦察机 Scout（朝下，红，小巧）。28×28。
pub const ENEMY_SCOUT: SpriteDef = SpriteDef {
    size: Vec2::new(28.0, 28.0),
    parts: parts![
        (0.0, 4.0, 24.0, 6.0, COLOR_E_SCOUT_DK, 0.04),
        (-10.0, 5.0, 5.0, 5.0, COLOR_E_SCOUT, 0.04),
        (10.0, 5.0, 5.0, 5.0, COLOR_E_SCOUT, 0.04),
        (0.0, -1.0, 9.0, 18.0, COLOR_E_SCOUT, 0.05),
        (-2.5, 0.0, 3.0, 14.0, COLOR_E_SCOUT_HI, 0.053),
        (0.0, -10.0, 3.0, 5.0, COLOR_E_SCOUT_DK, 0.05),
        (0.0, 1.0, 5.0, 5.0, E_CORE, 0.06),
        (-5.0, -1.0, 1.0, 18.0, OUTLINE, 0.03),
        (5.0, -1.0, 1.0, 18.0, OUTLINE, 0.03),
    ],
};

/// 狙击机 Sniper（朝下，橙，带双侧炮）。34×24。
pub const ENEMY_SNIPER: SpriteDef = SpriteDef {
    size: Vec2::new(34.0, 24.0),
    parts: parts![
        (0.0, 3.0, 34.0, 6.0, COLOR_E_SNIPER_DK, 0.04),
        (0.0, 0.0, 14.0, 16.0, COLOR_E_SNIPER, 0.05),
        (-4.0, 1.0, 3.0, 12.0, COLOR_E_SNIPER_HI, 0.053),
        (0.0, -9.0, 4.0, 4.0, COLOR_E_SNIPER_DK, 0.05),
        (-12.0, -7.0, 4.0, 10.0, COLOR_E_SNIPER_DK, 0.05), // 左炮
        (12.0, -7.0, 4.0, 10.0, COLOR_E_SNIPER_DK, 0.05),  // 右炮
        (0.0, 1.0, 7.0, 7.0, E_CORE, 0.06),
        (-7.0, 0.0, 1.0, 16.0, OUTLINE, 0.03),
        (7.0, 0.0, 1.0, 16.0, OUTLINE, 0.03),
    ],
};

/// 轰炸机 Bomber（朝下，绿，大翼 + 双引擎）。54×36。
pub const ENEMY_BOMBER: SpriteDef = SpriteDef {
    size: Vec2::new(54.0, 36.0),
    parts: parts![
        (0.0, 6.0, 54.0, 9.0, COLOR_E_BOMBER_DK, 0.04),
        (0.0, 0.0, 22.0, 24.0, COLOR_E_BOMBER, 0.05),
        (-6.5, 2.0, 4.0, 18.0, COLOR_E_BOMBER_HI, 0.053),
        (0.0, -13.0, 8.0, 6.0, COLOR_E_BOMBER_DK, 0.05),
        (-18.0, -10.0, 10.0, 8.0, COLOR_E_BOMBER_DK, 0.05), // 左引擎
        (18.0, -10.0, 10.0, 8.0, COLOR_E_BOMBER_DK, 0.05),  // 右引擎
        (-18.0, -13.0, 6.0, 3.0, P_THRUST, 0.055),
        (18.0, -13.0, 6.0, 3.0, P_THRUST, 0.055),
        (0.0, 3.0, 12.0, 12.0, E_CORE, 0.06),
        (0.0, 3.0, 6.0, 6.0, E_CORE_HI, 0.065),
        (-11.0, 0.0, 1.0, 24.0, OUTLINE, 0.03),
        (11.0, 0.0, 1.0, 24.0, OUTLINE, 0.03),
    ],
};

/// 补给机 Carrier（朝下，品红，方正带亮核）。32×32。
pub const ENEMY_CARRIER: SpriteDef = SpriteDef {
    size: Vec2::new(32.0, 32.0),
    parts: parts![
        (0.0, 0.0, 26.0, 24.0, COLOR_E_CARRIER, 0.05),
        (-8.0, 2.0, 4.0, 18.0, COLOR_E_CARRIER_HI, 0.053),
        (0.0, 9.0, 30.0, 6.0, COLOR_E_CARRIER_DK, 0.04),
        (0.0, -11.0, 18.0, 4.0, COLOR_E_CARRIER_DK, 0.05),
        (-13.0, -2.0, 4.0, 16.0, COLOR_E_CARRIER_DK, 0.045),
        (13.0, -2.0, 4.0, 16.0, COLOR_E_CARRIER_DK, 0.045),
        (0.0, 0.0, 10.0, 10.0, E_CORE, 0.06),
        (0.0, 0.0, 5.0, 5.0, E_CORE_HI, 0.065),
        (0.0, 12.5, 26.0, 1.0, OUTLINE, 0.03),
        (0.0, -12.5, 18.0, 1.0, OUTLINE, 0.03),
    ],
};

/// Boss 母舰（朝下，大型，双侧炮塔 + 中央核）。170×110。
pub const ENEMY_BOSS: SpriteDef = SpriteDef {
    size: Vec2::new(170.0, 110.0),
    parts: parts![
        // 主体舰身
        (0.0, 10.0, 150.0, 70.0, COLOR_E_BOSS, 0.05),
        (-52.0, 18.0, 28.0, 50.0, COLOR_E_BOSS_HI, 0.053),
        (0.0, 36.0, 170.0, 22.0, COLOR_E_BOSS_DK, 0.04), // 顶部装甲带
        (0.0, -30.0, 60.0, 24.0, COLOR_E_BOSS_DK, 0.05), // 下方舰首
        // 双侧炮塔
        (-66.0, -10.0, 30.0, 30.0, COLOR_E_BOSS_DK, 0.06),
        (66.0, -10.0, 30.0, 30.0, COLOR_E_BOSS_DK, 0.06),
        (-66.0, -28.0, 8.0, 14.0, OUTLINE, 0.055),
        (66.0, -28.0, 8.0, 14.0, OUTLINE, 0.055),
        // 中央能量核
        (0.0, 6.0, 44.0, 28.0, COLOR_E_BOSS_DK, 0.06),
        (0.0, 6.0, 30.0, 18.0, E_CORE, 0.07),
        (0.0, 6.0, 16.0, 10.0, E_CORE_HI, 0.075),
        // 舰桥灯带
        (-30.0, 26.0, 10.0, 6.0, METAL_HI, 0.06),
        (0.0, 26.0, 10.0, 6.0, METAL_HI, 0.06),
        (30.0, 26.0, 10.0, 6.0, METAL_HI, 0.06),
        // 描边
        (0.0, 45.0, 170.0, 2.0, OUTLINE, 0.03),
        (-83.0, 10.0, 2.0, 70.0, OUTLINE, 0.03),
        (83.0, 10.0, 2.0, 70.0, OUTLINE, 0.03),
    ],
};

/// 火力升级道具（黄盘 + 红芯 + 白十字）。22×22。
pub const POWERUP_P: SpriteDef = SpriteDef {
    size: Vec2::new(22.0, 22.0),
    parts: parts![
        (0.0, 0.0, 22.0, 22.0, PU_BG, 0.0),
        (0.0, 0.0, 14.0, 14.0, PU_CORE, 0.01),
        (0.0, 0.0, 4.0, 10.0, PU_MARK, 0.02),
        (0.0, 0.0, 10.0, 4.0, PU_MARK, 0.02),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::components::EnemyKind;

    #[test]
    fn ship_sprite_defs_well_formed() {
        for (name, def) in [
            ("PLAYER", &SHIP_PLAYER),
            ("SCOUT", &ENEMY_SCOUT),
            ("SNIPER", &ENEMY_SNIPER),
            ("BOMBER", &ENEMY_BOMBER),
            ("CARRIER", &ENEMY_CARRIER),
            ("BOSS", &ENEMY_BOSS),
            ("POWERUP", &POWERUP_P),
        ] {
            def.check(name);
        }
    }

    #[test]
    fn enemy_sprite_sizes_match_enum() {
        assert_eq!(ENEMY_SCOUT.size, EnemyKind::Scout.size());
        assert_eq!(ENEMY_SNIPER.size, EnemyKind::Sniper.size());
        assert_eq!(ENEMY_BOMBER.size, EnemyKind::Bomber.size());
        assert_eq!(ENEMY_CARRIER.size, EnemyKind::Carrier.size());
        assert_eq!(ENEMY_BOSS.size, EnemyKind::Boss.size());
    }
}
