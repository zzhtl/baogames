//! 记忆翻翻乐卡片精灵（游戏渲染 + 离线预览同源）。
//!
//! 卡片按 canonical 64×64 绘制，游戏内按 `cell / 64` 整体缩放组节点。
//! 圆角用「横长 + 竖长 + 中方」三块矩形叠出 4px 阶梯内收。
//! `cargo run --bin preview -- memory back` 出预览图。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::palette::*;

pub const CARD_CANONICAL: f32 = 64.0;

/// 卡背：圆角描边 + 紫面（左上受光、右下暗）+ 金菱纹样 + 四角饰点。
pub const CARD_BACK: SpriteDef = SpriteDef {
    size: Vec2::splat(CARD_CANONICAL),
    parts: parts![
        // 圆角描边底（三块叠出 4px 圆角）
        (0.0, 0.0, 64.0, 56.0, BACK_OUTLINE, 0.0),
        (0.0, 0.0, 56.0, 64.0, BACK_OUTLINE, 0.0),
        (0.0, 0.0, 60.0, 60.0, BACK_OUTLINE, 0.0),
        // 紫面主体（内缩 2.5px，同样圆角叠法）
        (0.0, 0.0, 59.0, 51.0, BACK_BASE, 0.02),
        (0.0, 0.0, 51.0, 59.0, BACK_BASE, 0.02),
        (0.0, 0.0, 55.0, 55.0, BACK_BASE, 0.02),
        // 明暗阶：顶部受光、左缘微光、底部与右缘暗
        (0.0, 25.5, 51.0, 6.0, BACK_HI, 0.03),
        (-25.5, 0.0, 6.0, 43.0, BACK_HI, 0.03),
        (0.0, -25.5, 51.0, 6.0, BACK_DK, 0.03),
        (25.5, 0.0, 6.0, 43.0, BACK_DK, 0.03),
        // 金色菱形纹样（阶梯堆叠）
        (0.0, 10.0, 6.0, 5.0, BACK_PATTERN, 0.04),
        (0.0, 5.0, 14.0, 5.0, BACK_PATTERN, 0.04),
        (0.0, 0.0, 20.0, 5.0, BACK_PATTERN, 0.04),
        (0.0, -5.0, 14.0, 5.0, BACK_PATTERN, 0.04),
        (0.0, -10.0, 6.0, 5.0, BACK_PATTERN, 0.04),
        // 菱心亮点
        (0.0, 0.0, 5.0, 5.0, BACK_GLOW, 0.05),
        // 四角饰点
        (-20.0, 20.0, 4.0, 4.0, BACK_PATTERN, 0.04),
        (20.0, 20.0, 4.0, 4.0, BACK_PATTERN, 0.04),
        (-20.0, -20.0, 4.0, 4.0, BACK_PATTERN, 0.04),
        (20.0, -20.0, 4.0, 4.0, BACK_PATTERN, 0.04),
    ],
};

/// 卡面（未配对）：暖棕描边 + 米白面 + 金色内饰线，中央留白给字符。
pub const CARD_FACE: SpriteDef = SpriteDef {
    size: Vec2::splat(CARD_CANONICAL),
    parts: parts![
        // 圆角描边底
        (0.0, 0.0, 64.0, 56.0, FACE_OUTLINE, 0.0),
        (0.0, 0.0, 56.0, 64.0, FACE_OUTLINE, 0.0),
        (0.0, 0.0, 60.0, 60.0, FACE_OUTLINE, 0.0),
        // 米白面
        (0.0, 0.0, 59.0, 51.0, FACE_BASE, 0.02),
        (0.0, 0.0, 51.0, 59.0, FACE_BASE, 0.02),
        (0.0, 0.0, 55.0, 55.0, FACE_BASE, 0.02),
        // 顶部纯白高光 + 底部暖阴影
        (0.0, 25.5, 51.0, 5.0, FACE_HI, 0.03),
        (0.0, -25.5, 51.0, 5.0, FACE_DK, 0.03),
        // 金色内饰细框（4 条）
        (0.0, 21.0, 44.0, 1.5, FACE_TRIM, 0.04),
        (0.0, -21.0, 44.0, 1.5, FACE_TRIM, 0.04),
        (-21.0, 0.0, 1.5, 40.0, FACE_TRIM, 0.04),
        (21.0, 0.0, 1.5, 40.0, FACE_TRIM, 0.04),
    ],
};

/// 卡面（已配对）：金色系，结构同 CARD_FACE，中央仍留白给字符。
pub const CARD_FACE_MATCHED: SpriteDef = SpriteDef {
    size: Vec2::splat(CARD_CANONICAL),
    parts: parts![
        (0.0, 0.0, 64.0, 56.0, DONE_OUTLINE, 0.0),
        (0.0, 0.0, 56.0, 64.0, DONE_OUTLINE, 0.0),
        (0.0, 0.0, 60.0, 60.0, DONE_OUTLINE, 0.0),
        (0.0, 0.0, 59.0, 51.0, DONE_BASE, 0.02),
        (0.0, 0.0, 51.0, 59.0, DONE_BASE, 0.02),
        (0.0, 0.0, 55.0, 55.0, DONE_BASE, 0.02),
        (0.0, 25.5, 51.0, 5.0, DONE_HI, 0.03),
        (0.0, -25.5, 51.0, 5.0, DONE_DK, 0.03),
        // 四角星点缀（配对成功的庆祝感）
        (-20.0, 20.0, 5.0, 2.0, DONE_HI, 0.04),
        (-20.0, 20.0, 2.0, 5.0, DONE_HI, 0.04),
        (20.0, 20.0, 5.0, 2.0, DONE_HI, 0.04),
        (20.0, 20.0, 2.0, 5.0, DONE_HI, 0.04),
        (-20.0, -20.0, 5.0, 2.0, DONE_HI, 0.04),
        (-20.0, -20.0, 2.0, 5.0, DONE_HI, 0.04),
        (20.0, -20.0, 5.0, 2.0, DONE_HI, 0.04),
        (20.0, -20.0, 2.0, 5.0, DONE_HI, 0.04),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_defs_fit_canonical_bounds() {
        CARD_BACK.check("memory CARD_BACK");
        CARD_FACE.check("memory CARD_FACE");
        CARD_FACE_MATCHED.check("memory CARD_FACE_MATCHED");
    }
}
