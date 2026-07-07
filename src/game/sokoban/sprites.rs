//! 推箱子精灵（游戏渲染 + 离线预览同源）。
//!
//! 地块按 canonical 40×40、箱子 34×34、小人 30×30 绘制，
//! 游戏内按 `cell / canonical` 整体缩放组节点。
//! `cargo run --bin preview -- soko box` 出预览图。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::palette::*;

pub const TILE_CANONICAL: f32 = 40.0;
pub const BOX_CANONICAL: f32 = 34.0;
pub const PLAYER_CANONICAL: f32 = 30.0;

/// 砖墙：暗描边 + 砖面（顶部受光、底部暗）+ 错位砖缝。
pub const WALL_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(TILE_CANONICAL),
    parts: parts![
        (0.0, 0.0, 40.0, 40.0, WALL_OUTLINE, 0.0),
        (0.0, 0.0, 36.0, 36.0, WALL_BASE, 0.02),
        // 明暗阶：顶部亮 → 顶缘高光；底部暗
        (0.0, 15.5, 36.0, 5.0, WALL_HI, 0.03),
        (0.0, 17.0, 34.0, 2.0, WALL_TOP, 0.04),
        (0.0, -16.0, 36.0, 4.0, WALL_DK, 0.03),
        // 横向砖缝 2 条
        (0.0, 6.5, 36.0, 1.6, WALL_MORTAR, 0.05),
        (0.0, -6.5, 36.0, 1.6, WALL_MORTAR, 0.05),
        // 竖向砖缝错位（上排中间、中排两侧、下排中间）
        (0.0, 12.0, 1.6, 9.5, WALL_MORTAR, 0.05),
        (-9.0, 0.0, 1.6, 11.5, WALL_MORTAR, 0.05),
        (9.0, 0.0, 1.6, 11.5, WALL_MORTAR, 0.05),
        (0.0, -12.0, 1.6, 9.5, WALL_MORTAR, 0.05),
    ],
};

/// 地板：暗边 + 平面 + 左上微光（低对比背景）。
pub const FLOOR_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(TILE_CANONICAL),
    parts: parts![
        (0.0, 0.0, 40.0, 40.0, FLOOR_EDGE, 0.0),
        (0.0, 0.0, 37.0, 37.0, FLOOR_BASE, 0.02),
        (-9.0, 9.0, 15.0, 15.0, FLOOR_HI, 0.03),
    ],
};

/// 目标点：地板 + 外发光圈 + 橙色靶环 + 暗内圈 + 发光靶心。
pub const GOAL_TILE: SpriteDef = SpriteDef {
    size: Vec2::splat(TILE_CANONICAL),
    parts: parts![
        (0.0, 0.0, 40.0, 40.0, FLOOR_EDGE, 0.0),
        (0.0, 0.0, 37.0, 37.0, FLOOR_BASE, 0.02),
        (0.0, 0.0, 30.0, 30.0, GOAL_FAINT, 0.03),
        (0.0, 0.0, 24.0, 24.0, GOAL_RING, 0.04),
        (0.0, 0.0, 18.0, 18.0, GOAL_INNER, 0.05),
        (0.0, 0.0, 8.0, 8.0, GOAL_GLOW, 0.06),
    ],
};

/// 木箱几何：描边 + 边框 + 板面（顶亮底暗）+ 木纹 + 四角包角 + 中心铆钉。
macro_rules! box_def {
    ($outline:expr, $edge:expr, $face:expr, $hi:expr, $dk:expr, $corner:expr) => {
        SpriteDef {
            size: Vec2::splat(BOX_CANONICAL),
            parts: parts![
                // 圆角描边（十字叠出 2px 圆角）
                (0.0, 0.0, 34.0, 30.0, $outline, 0.0),
                (0.0, 0.0, 30.0, 34.0, $outline, 0.0),
                // 边框区 + 板面
                (0.0, 0.0, 30.0, 30.0, $edge, 0.02),
                (0.0, 0.0, 25.0, 25.0, $face, 0.03),
                // 明暗阶
                (0.0, 10.5, 25.0, 4.0, $hi, 0.04),
                (0.0, -10.5, 25.0, 4.0, $dk, 0.04),
                // 横向木纹 2 条（比板面深一阶才可读）
                (0.0, 4.0, 25.0, 2.0, $dk, 0.05),
                (0.0, -4.0, 25.0, 2.0, $dk, 0.05),
                // 四角金属包角 + 对称高光点
                (-13.0, 13.0, 7.0, 7.0, $corner, 0.06),
                (13.0, 13.0, 7.0, 7.0, $corner, 0.06),
                (-13.0, -13.0, 7.0, 7.0, $corner, 0.06),
                (13.0, -13.0, 7.0, 7.0, $corner, 0.06),
                (-14.0, 14.0, 2.5, 2.5, $hi, 0.07),
                (14.0, 14.0, 2.5, 2.5, $hi, 0.07),
                (-14.0, -14.0, 2.5, 2.5, $hi, 0.07),
                (14.0, -14.0, 2.5, 2.5, $hi, 0.07),
                // 中心铆钉
                (0.0, 0.0, 4.0, 4.0, $corner, 0.06),
            ],
        }
    };
}

pub const BOX_WOOD: SpriteDef =
    box_def!(BOX_OUTLINE, BOX_EDGE, BOX_FACE, BOX_HI, BOX_DK, BOX_CORNER);
pub const BOX_DONE: SpriteDef =
    box_def!(DONE_OUTLINE, DONE_EDGE, DONE_FACE, DONE_HI, DONE_DK, DONE_CORNER);

/// 工帽小人：安全帽（帽顶高光 + 帽檐）+ 圆脸（眼睛 + 微笑）+ 蓝工装（背带 + 底暗）。
pub const SOKO_PLAYER: SpriteDef = SpriteDef {
    size: Vec2::splat(PLAYER_CANONICAL),
    parts: parts![
        // 描边底板（十字叠圆角）
        (0.0, -0.5, 28.0, 25.0, P_OUTLINE, 0.0),
        (0.0, -0.5, 24.0, 29.0, P_OUTLINE, 0.0),
        // 蓝色工装（下半）
        (0.0, -8.0, 24.0, 12.0, SUIT, 0.02),
        (0.0, -13.0, 24.0, 3.0, SUIT_DK, 0.03),
        (-8.0, -7.0, 4.0, 10.0, SUIT_HI, 0.03),
        // 背带扣 2 颗
        (-5.0, -4.5, 3.0, 3.0, HAT, 0.04),
        (5.0, -4.5, 3.0, 3.0, HAT, 0.04),
        // 脸
        (0.0, 1.5, 22.0, 11.0, SKIN, 0.04),
        // 眼睛 + 微笑
        (-4.5, 2.5, 3.0, 4.0, EYE, 0.06),
        (4.5, 2.5, 3.0, 4.0, EYE, 0.06),
        (0.0, -2.0, 8.0, 2.0, MOUTH, 0.06),
        // 安全帽：帽檐 + 帽体 + 帽顶高光 + 中缝
        (0.0, 7.0, 26.0, 3.0, HAT_DK, 0.05),
        (0.0, 10.5, 20.0, 6.0, HAT, 0.05),
        (-5.0, 12.0, 7.0, 2.5, HAT_HI, 0.06),
        (0.0, 11.0, 3.0, 7.0, HAT_DK, 0.055),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soko_defs_fit_canonical_bounds() {
        WALL_TILE.check("soko WALL_TILE");
        FLOOR_TILE.check("soko FLOOR_TILE");
        GOAL_TILE.check("soko GOAL_TILE");
        BOX_WOOD.check("soko BOX_WOOD");
        BOX_DONE.check("soko BOX_DONE");
        SOKO_PLAYER.check("soko SOKO_PLAYER");
    }
}
