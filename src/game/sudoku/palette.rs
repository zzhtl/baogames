//! 儿童数独配色。

use bevy::prelude::Color;

pub const CELL_FILL: Color = Color::srgb(0.094, 0.110, 0.149);
/// 题目给定的格子压暗一档，和自己填的一眼分开。
pub const GIVEN_FILL: Color = Color::srgb(0.145, 0.168, 0.216);
pub const CURSOR_FILL: Color = Color::srgb(0.180, 0.231, 0.275);
pub const CONFLICT_FILL: Color = Color::srgb(0.404, 0.133, 0.145);

/// 格线与宫线。宫线更亮更粗，否则 9×9 在 16 像素格上分不出宫。
pub const CELL_LINE: Color = Color::srgb(0.204, 0.239, 0.298);
pub const BOX_LINE: Color = Color::srgb(0.435, 0.529, 0.643);

pub const GIVEN_TEXT: Color = Color::srgb(0.667, 0.729, 0.808);
pub const FILLED_TEXT: Color = Color::srgb(0.996, 0.863, 0.373);
pub const CONFLICT_TEXT: Color = Color::srgb(1.0, 0.647, 0.616);
pub const CURSOR: Color = Color::srgb(0.361, 0.937, 0.702);
