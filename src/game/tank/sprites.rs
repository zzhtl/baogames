//! 坦克大战精灵的共享定义（游戏渲染 + 离线预览同源）。
//!
//! 坦克一律**按朝上**画，游戏里靠父实体 `Transform` 旋转转向（子块跟着转）。
//! `cargo run --bin preview -- tank p1` 出预览图。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::constants::{POWERUP_SIZE, SUBTILE, TANK_SIZE};
use super::palette::*;

/// 坦克几何：左右履带 + 车体（带右侧阴影）+ 炮塔 + 朝上炮管 + 黑描边。
/// `$body` 车体主色、`$body_dk` 阴影/炮塔色。
macro_rules! tank_def {
    ($body:expr, $hi:expr, $dark:expr) => {
        SpriteDef {
            size: Vec2::new(TANK_SIZE, TANK_SIZE),
            parts: parts![
                // 左右履带 + 横纹高光
                (-11.0, 0.0, 7.0, 28.0, COLOR_TANK_TREAD, 0.02),
                (11.0, 0.0, 7.0, 28.0, COLOR_TANK_TREAD, 0.02),
                (-11.0, 9.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (-11.0, 3.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (-11.0, -3.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (-11.0, -9.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (11.0, 9.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (11.0, 3.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (11.0, -3.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                (11.0, -9.0, 7.0, 1.5, COLOR_TANK_TREAD_HI, 0.025),
                // 车体（4 阶：主体 / 左上受光 / 右阴影 / 底阴影）
                (0.0, 0.0, 15.0, 24.0, $body, 0.04),
                (-4.0, 3.0, 6.0, 18.0, $hi, 0.045),
                (4.5, -1.0, 5.0, 22.0, $dark, 0.046),
                (0.0, -10.0, 15.0, 4.0, $dark, 0.047),
                // 炮塔（环 + 主体 + 高光 + 暗角）
                (0.0, 0.0, 13.0, 13.0, $dark, 0.06),
                (0.0, 0.0, 9.0, 9.0, $body, 0.065),
                (-1.6, 1.6, 3.5, 3.5, $hi, 0.067),
                (1.8, -1.8, 3.0, 3.0, $dark, 0.067),
                // 炮管（朝上，高光 + 暗边）
                (0.0, 13.0, 4.5, 16.0, COLOR_TANK_BARREL, 0.07),
                (-0.9, 13.0, 1.2, 16.0, COLOR_TANK_TREAD_HI, 0.072),
                (1.6, 13.0, 0.8, 16.0, COLOR_TANK_OUTLINE, 0.072),
                // 车体描边
                (0.0, 12.5, 15.0, 1.0, COLOR_TANK_OUTLINE, 0.035),
                (0.0, -12.5, 15.0, 1.0, COLOR_TANK_OUTLINE, 0.035),
                (-7.5, 0.0, 1.0, 25.0, COLOR_TANK_OUTLINE, 0.035),
                (7.5, 0.0, 1.0, 25.0, COLOR_TANK_OUTLINE, 0.035),
            ],
        }
    };
}

pub const TANK_P1: SpriteDef = tank_def!(COLOR_TANK_P1, COLOR_TANK_P1_HI, COLOR_TANK_P1_DK);
pub const TANK_P2: SpriteDef = tank_def!(COLOR_TANK_P2, COLOR_TANK_P2_HI, COLOR_TANK_P2_DK);
pub const TANK_ENEMY_BASIC: SpriteDef =
    tank_def!(COLOR_TANK_E_BASIC, COLOR_TANK_E_BASIC_HI, COLOR_TANK_E_BASIC_DK);
pub const TANK_ENEMY_FAST: SpriteDef =
    tank_def!(COLOR_TANK_E_FAST, COLOR_TANK_E_FAST_HI, COLOR_TANK_E_FAST_DK);
pub const TANK_ENEMY_POWER: SpriteDef =
    tank_def!(COLOR_TANK_E_POWER, COLOR_TANK_E_POWER_HI, COLOR_TANK_E_POWER_DK);
pub const TANK_ENEMY_ARMOR: SpriteDef =
    tank_def!(COLOR_TANK_E_ARMOR, COLOR_TANK_E_ARMOR_HI, COLOR_TANK_E_ARMOR_DK);

// ===== 地形瓦片 =====

/// 砖块子格（16×16，可逐格炸毁）：砖面 + 错位砖缝 + 顶高光。
pub const BRICK_SUBTILE: SpriteDef = SpriteDef {
    size: Vec2::splat(SUBTILE),
    parts: parts![
        (0.0, 0.0, 15.5, 15.5, COLOR_BRICK, 0.0),
        (0.0, 6.6, 15.5, 1.2, COLOR_BRICK_HI, 0.02),
        (0.0, 0.4, 15.5, 1.2, COLOR_BRICK_DK, 0.01),
        (0.0, -5.8, 15.5, 1.2, COLOR_BRICK_DK, 0.01),
        (-4.0, 3.4, 1.2, 6.0, COLOR_BRICK_DK, 0.01),
        (4.0, -2.8, 1.2, 6.0, COLOR_BRICK_DK, 0.01),
    ],
};

/// 钢块（32×32，不可炸）：四边斜面高光/阴影 + 中央十字 + 四角铆钉。
pub const STEEL_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(32.0),
    parts: parts![
        (0.0, 0.0, 32.0, 32.0, COLOR_STEEL, 0.0),
        (0.0, 14.0, 32.0, 4.0, COLOR_STEEL_HI, 0.02),
        (-14.0, 0.0, 4.0, 32.0, COLOR_STEEL_HI, 0.02),
        (0.0, -14.0, 32.0, 4.0, COLOR_STEEL_DK, 0.02),
        (14.0, 0.0, 4.0, 32.0, COLOR_STEEL_DK, 0.02),
        (0.0, 0.0, 3.0, 26.0, COLOR_STEEL_DK, 0.01),
        (0.0, 0.0, 26.0, 3.0, COLOR_STEEL_DK, 0.01),
        (-9.0, 9.0, 3.0, 3.0, COLOR_STEEL_HI, 0.03),
        (9.0, 9.0, 3.0, 3.0, COLOR_STEEL_HI, 0.03),
        (-9.0, -9.0, 3.0, 3.0, COLOR_STEEL_HI, 0.03),
        (9.0, -9.0, 3.0, 3.0, COLOR_STEEL_HI, 0.03),
    ],
};

/// 水（32×32，挡坦克不挡子弹）：深底 + 上半亮水 + 错落波纹。
pub const WATER_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(32.0),
    parts: parts![
        (0.0, 0.0, 32.0, 32.0, COLOR_WATER_DK, 0.0),
        (0.0, 5.0, 32.0, 18.0, COLOR_WATER, 0.01),
        (-6.0, 8.0, 14.0, 2.5, COLOR_WATER_HI, 0.02),
        (6.0, 1.0, 10.0, 2.5, COLOR_WATER_HI, 0.02),
        (-3.0, -7.0, 12.0, 2.5, COLOR_WATER_HI, 0.02),
    ],
};

/// 草丛（32×32，盖在坦克上方）：底绿 + 高光/暗叶簇。
pub const BUSH_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(32.0),
    parts: parts![
        (0.0, 0.0, 32.0, 32.0, COLOR_BUSH, 0.0),
        (-8.0, 8.0, 10.0, 10.0, COLOR_BUSH_HI, 0.02),
        (8.0, -6.0, 10.0, 10.0, COLOR_BUSH_HI, 0.02),
        (7.0, 9.0, 7.0, 7.0, COLOR_BUSH_HI, 0.02),
        (-9.0, -8.0, 8.0, 8.0, COLOR_BUSH_DK, 0.015),
        (2.0, 0.0, 6.0, 6.0, COLOR_BUSH_DK, 0.015),
    ],
};

/// 冰面（32×32，打滑）：浅蓝底 + 反光斜线 + 裂纹。
pub const ICE_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(32.0),
    parts: parts![
        (0.0, 0.0, 32.0, 32.0, COLOR_ICE, 0.0),
        (-7.0, 8.0, 13.0, 2.0, COLOR_ICE_HI, 0.02),
        (5.0, -4.0, 11.0, 2.0, COLOR_ICE_HI, 0.02),
        (10.0, 10.0, 5.0, 2.0, COLOR_ICE_DK, 0.015),
        (-6.0, -10.0, 7.0, 2.0, COLOR_ICE_DK, 0.015),
    ],
};

/// 基地（28×28）：深底座 + 黄色鹰徽。
pub const BASE_EAGLE: SpriteDef = SpriteDef {
    size: Vec2::splat(28.0),
    parts: parts![
        (0.0, 0.0, 28.0, 28.0, COLOR_BASE, 0.0),
        (0.0, 1.0, 16.0, 11.0, COLOR_BASE_EAGLE, 0.02),
        (0.0, 9.0, 5.0, 5.0, COLOR_BASE_EAGLE, 0.02),
        (0.0, -8.0, 9.0, 6.0, COLOR_BASE_EAGLE, 0.02),
        (0.0, 1.0, 16.0, 1.2, COLOR_BASE_EAGLE_DK, 0.025),
        (-5.0, 1.0, 1.2, 9.0, COLOR_BASE_EAGLE_DK, 0.025),
        (5.0, 1.0, 1.2, 9.0, COLOR_BASE_EAGLE_DK, 0.025),
        (0.0, 11.0, 2.0, 2.0, COLOR_BASE, 0.03),
    ],
};

// ===== 道具徽章 =====

/// 道具徽章外壳：金边 + 暗底。`$sym` 是符号 parts 块。
macro_rules! powerup_def {
    ($($p:tt),* $(,)?) => {
        SpriteDef {
            size: Vec2::splat(POWERUP_SIZE),
            parts: parts![
                (0.0, 0.0, 28.0, 28.0, COLOR_PU_BORDER, 0.0),
                (0.0, 0.0, 23.0, 23.0, COLOR_PU_BG, 0.01),
                $($p),*
            ],
        }
    };
}

/// 星（升级）：黄色四角闪光。
pub const PU_STAR: SpriteDef = powerup_def!(
    (0.0, 0.0, 4.0, 18.0, COLOR_PU_STAR, 0.02),
    (0.0, 0.0, 18.0, 4.0, COLOR_PU_STAR, 0.02),
    (-5.0, 5.0, 3.0, 3.0, COLOR_PU_STAR, 0.02),
    (5.0, 5.0, 3.0, 3.0, COLOR_PU_STAR, 0.02),
    (-5.0, -5.0, 3.0, 3.0, COLOR_PU_STAR, 0.02),
    (5.0, -5.0, 3.0, 3.0, COLOR_PU_STAR, 0.02),
);

/// 手雷（清屏）：绿色圆弹 + 引线火星。
pub const PU_GRENADE: SpriteDef = powerup_def!(
    (0.0, -2.0, 12.0, 12.0, COLOR_PU_GRENADE, 0.02),
    (0.0, -2.0, 14.0, 9.0, COLOR_PU_GRENADE, 0.02),
    (0.0, -2.0, 9.0, 14.0, COLOR_PU_GRENADE, 0.02),
    (4.0, 7.0, 2.0, 4.0, COLOR_PU_GRENADE, 0.02),
    (5.0, 9.0, 3.0, 2.0, COLOR_PU_STAR, 0.025),
);

/// 头盔（护盾）：蓝色穹顶。
pub const PU_HELMET: SpriteDef = powerup_def!(
    (0.0, 0.0, 16.0, 7.0, COLOR_PU_HELMET, 0.02),
    (0.0, 5.0, 11.0, 4.0, COLOR_PU_HELMET, 0.02),
    (0.0, -4.0, 18.0, 3.0, COLOR_PU_HELMET, 0.02),
);

/// 坦克（加命）：橙色小坦克。
pub const PU_TANK: SpriteDef = powerup_def!(
    (0.0, -1.0, 11.0, 12.0, COLOR_PU_TANK, 0.02),
    (-6.0, -1.0, 3.0, 13.0, COLOR_PU_TANK, 0.02),
    (6.0, -1.0, 3.0, 13.0, COLOR_PU_TANK, 0.02),
    (0.0, 8.0, 3.0, 7.0, COLOR_PU_TANK, 0.02),
);

/// 时钟（冻结）：紫色表盘 + 指针。
pub const PU_CLOCK: SpriteDef = powerup_def!(
    (0.0, 7.0, 15.0, 2.5, COLOR_PU_CLOCK, 0.02),
    (0.0, -7.0, 15.0, 2.5, COLOR_PU_CLOCK, 0.02),
    (-7.0, 0.0, 2.5, 15.0, COLOR_PU_CLOCK, 0.02),
    (7.0, 0.0, 2.5, 15.0, COLOR_PU_CLOCK, 0.02),
    (0.0, 2.0, 2.0, 6.0, COLOR_PU_CLOCK, 0.025),
    (3.0, 0.0, 6.0, 2.0, COLOR_PU_CLOCK, 0.025),
);

/// 铲子（筑墙）：棕色铲。
pub const PU_SHOVEL: SpriteDef = powerup_def!(
    (0.0, 4.0, 3.0, 12.0, COLOR_PU_SHOVEL, 0.02),
    (0.0, 10.0, 7.0, 3.0, COLOR_PU_SHOVEL, 0.02),
    (0.0, -6.0, 9.0, 8.0, COLOR_PU_SHOVEL, 0.02),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powerup_sprite_defs_well_formed() {
        PU_STAR.check("PU_STAR");
        PU_GRENADE.check("PU_GRENADE");
        PU_HELMET.check("PU_HELMET");
        PU_TANK.check("PU_TANK");
        PU_CLOCK.check("PU_CLOCK");
        PU_SHOVEL.check("PU_SHOVEL");
    }

    #[test]
    fn terrain_sprite_defs_well_formed() {
        BRICK_SUBTILE.check("BRICK_SUBTILE");
        STEEL_TILE.check("STEEL_TILE");
        WATER_TILE.check("WATER_TILE");
        BUSH_TILE.check("BUSH_TILE");
        ICE_TILE.check("ICE_TILE");
        BASE_EAGLE.check("BASE_EAGLE");
    }

    #[test]
    fn tank_sprite_defs_well_formed() {
        for (name, def) in [
            ("P1", &TANK_P1),
            ("P2", &TANK_P2),
            ("BASIC", &TANK_ENEMY_BASIC),
            ("FAST", &TANK_ENEMY_FAST),
            ("POWER", &TANK_ENEMY_POWER),
            ("ARMOR", &TANK_ENEMY_ARMOR),
        ] {
            def.check(name);
            assert_eq!(def.size, Vec2::splat(TANK_SIZE));
        }
    }
}
