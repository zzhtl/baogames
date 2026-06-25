//! 炸弹迷宫精灵的共享定义（游戏渲染 + 离线预览同源）。
//!
//! 俯视、方向不旋转。`cargo run --bin preview -- bomb player1` 出预览图。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::constants::{BM_BOMB_SIZE, BM_ENEMY_SIZE, BM_FLAME_SIZE, BM_PLAYER_SIZE, BM_TILE};
use super::palette::*;

/// 炸弹人：白甲 + 彩盔 + 护目 + 腰带 + 双脚。`$helm`/`$helm_dk` 头盔色。
macro_rules! player_def {
    ($helm:expr, $helm_dk:expr) => {
        SpriteDef {
            size: Vec2::splat(BM_PLAYER_SIZE),
            parts: parts![
                // 身体
                (0.0, -3.0, 15.0, 14.0, P_SUIT, 0.05),
                (0.0, -8.0, 15.0, 4.0, P_SUIT_DK, 0.055),
                (0.0, -3.5, 15.0, 2.5, $helm, 0.06), // 腰带
                // 头盔
                (0.0, 7.0, 17.0, 9.0, $helm, 0.06),
                (0.0, 11.0, 11.0, 3.0, $helm_dk, 0.065),
                (0.0, 13.5, 3.0, 4.0, $helm, 0.066), // 顶饰
                // 护目
                (0.0, 4.0, 11.0, 4.0, P_VISOR, 0.07),
                (3.0, 4.0, 3.0, 3.0, $helm_dk, 0.075),
                // 双脚
                (-4.0, -11.0, 6.0, 4.0, P_FOOT, 0.06),
                (4.0, -11.0, 6.0, 4.0, P_FOOT, 0.06),
                // 描边
                (-8.0, 0.0, 1.0, 18.0, OUTLINE, 0.03),
                (8.0, 0.0, 1.0, 18.0, OUTLINE, 0.03),
                (0.0, 12.5, 17.0, 1.0, OUTLINE, 0.03),
            ],
        }
    };
}

pub const BM_PLAYER1: SpriteDef = player_def!(P1_HELM, P1_HELM_DK);
pub const BM_PLAYER2: SpriteDef = player_def!(P2_HELM, P2_HELM_DK);

/// 球形敌人：圆身 + 球面 3 阶明暗 + 双眼 + 瞳孔。`$body`/受光 `$hi`/暗面 `$dk`。
macro_rules! enemy_def {
    ($body:expr, $hi:expr, $dk:expr) => {
        SpriteDef {
            size: Vec2::splat(BM_ENEMY_SIZE),
            parts: parts![
                (0.0, 0.0, 18.0, 18.0, $body, 0.05),
                (0.0, -4.0, 16.0, 7.0, $dk, 0.055),  // 下半暗
                (0.0, 7.0, 14.0, 5.0, $hi, 0.056),   // 顶部受光（球面）
                (-4.5, 6.0, 4.0, 3.0, $hi, 0.057),   // 左上高光斑
                // 双眼
                (-4.0, 2.0, 6.0, 6.0, E_EYE, 0.06),
                (4.0, 2.0, 6.0, 6.0, E_EYE, 0.06),
                (-4.0, 1.5, 2.5, 2.5, E_PUPIL, 0.065),
                (4.0, 1.5, 2.5, 2.5, E_PUPIL, 0.065),
                // 描边
                (-9.0, 0.0, 1.0, 18.0, OUTLINE, 0.03),
                (9.0, 0.0, 1.0, 18.0, OUTLINE, 0.03),
                (0.0, 9.0, 16.0, 1.0, OUTLINE, 0.03),
                (0.0, -9.0, 16.0, 1.0, OUTLINE, 0.03),
            ],
        }
    };
}

pub const ENEMY_BALLOOM: SpriteDef = enemy_def!(E_BALLOOM, E_BALLOOM_HI, E_BALLOOM_DK);
pub const ENEMY_ONEAL: SpriteDef = enemy_def!(E_ONEAL, E_ONEAL_HI, E_ONEAL_DK);
pub const ENEMY_DOLL: SpriteDef = enemy_def!(E_DOLL, E_DOLL_HI, E_DOLL_DK);
pub const ENEMY_KONDORIA: SpriteDef = enemy_def!(E_KONDORIA, E_KONDORIA_HI, E_KONDORIA_DK);

/// 炸弹：黑球 + 高光 + 引线 + 火花。
pub const BOMB_DEF: SpriteDef = SpriteDef {
    size: Vec2::splat(BM_BOMB_SIZE),
    parts: parts![
        (0.0, -1.0, 22.0, 22.0, BOMB, 0.05),
        (0.0, -1.0, 24.0, 16.0, BOMB, 0.05),
        (0.0, -1.0, 16.0, 24.0, BOMB, 0.05),
        (-5.0, 5.0, 7.0, 7.0, BOMB_HI, 0.06), // 高光
        (4.0, 12.0, 2.5, 6.0, BOMB_FUSE, 0.055), // 引线
        (5.0, 16.0, 4.0, 4.0, BOMB_SPARK, 0.06), // 火花
    ],
};

/// 硬墙（不可炸，36×36）：立体倒角钢块。
pub const HARD_WALL: SpriteDef = SpriteDef {
    size: Vec2::splat(BM_TILE),
    parts: parts![
        (0.0, 0.0, 36.0, 36.0, HARD, 0.0),
        (0.0, 15.0, 34.0, 5.0, HARD_HI, 0.02),
        (-15.0, 0.0, 5.0, 34.0, HARD_HI, 0.02),
        (0.0, -15.0, 34.0, 5.0, HARD_DK, 0.02),
        (15.0, 0.0, 5.0, 34.0, HARD_DK, 0.02),
        (0.0, 0.0, 18.0, 18.0, HARD_DK, 0.01),
        (0.0, 0.0, 14.0, 14.0, HARD, 0.015),
    ],
};

/// 软砖（可炸，35×35）：砖墙 + 横竖砖缝 + 顶高光。
pub const SOFT_WALL: SpriteDef = SpriteDef {
    size: Vec2::splat(BM_TILE - 1.0),
    parts: parts![
        (0.0, 0.0, 35.0, 35.0, SOFT, 0.0),
        (0.0, 15.0, 35.0, 3.0, SOFT_HI, 0.02),
        (0.0, 9.0, 35.0, 1.5, SOFT_SEAM, 0.01),
        (0.0, -9.0, 35.0, 1.5, SOFT_SEAM, 0.01),
        (-8.0, 4.0, 1.5, 12.0, SOFT_SEAM, 0.01),
        (8.0, -4.0, 1.5, 12.0, SOFT_SEAM, 0.01),
        (0.0, 0.0, 1.5, 12.0, SOFT_SEAM, 0.01),
    ],
};

/// 火焰（32×32）：外橙 + 中黄 + 亮核。
pub const FLAME_DEF: SpriteDef = SpriteDef {
    size: Vec2::splat(BM_FLAME_SIZE),
    parts: parts![
        (0.0, 0.0, 32.0, 22.0, FLAME_OUT, 0.0),
        (0.0, 0.0, 22.0, 32.0, FLAME_OUT, 0.0),
        (0.0, 0.0, 22.0, 16.0, FLAME_MID, 0.02),
        (0.0, 0.0, 16.0, 22.0, FLAME_MID, 0.02),
        (0.0, 0.0, 11.0, 11.0, FLAME_CORE, 0.04),
    ],
};

/// 出口（30×30）：暗底 + 紫色传送门 + 上行箭头。
pub const EXIT_DEF: SpriteDef = SpriteDef {
    size: Vec2::splat(BM_TILE - 6.0),
    parts: parts![
        (0.0, 0.0, 30.0, 30.0, EXIT_BASE, 0.0),
        (0.0, 0.0, 22.0, 22.0, EXIT_PORTAL, 0.02),
        (0.0, 0.0, 14.0, 14.0, EXIT_BASE, 0.03),
        (0.0, 1.0, 3.0, 10.0, EXIT_ARROW, 0.05),
        (0.0, 4.0, 8.0, 3.0, EXIT_ARROW, 0.05),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bomb_maze_sprite_defs_well_formed() {
        for (name, def) in [
            ("PLAYER1", &BM_PLAYER1),
            ("PLAYER2", &BM_PLAYER2),
            ("BALLOOM", &ENEMY_BALLOOM),
            ("ONEAL", &ENEMY_ONEAL),
            ("DOLL", &ENEMY_DOLL),
            ("KONDORIA", &ENEMY_KONDORIA),
            ("BOMB", &BOMB_DEF),
            ("HARD", &HARD_WALL),
            ("SOFT", &SOFT_WALL),
            ("FLAME", &FLAME_DEF),
            ("EXIT", &EXIT_DEF),
        ] {
            def.check(name);
        }
    }
}
