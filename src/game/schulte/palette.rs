//! 舒尔特方格配色。

use bevy::prelude::Color;

/// 未点过的格子。
pub const CELL_FILL: Color = Color::srgb(0.098, 0.133, 0.176);
pub const CELL_BORDER: Color = Color::srgb(0.180, 0.235, 0.298);
/// 已点掉的格子：压暗并去掉描边对比，视觉上"塌下去"。
pub const DONE_FILL: Color = Color::srgb(0.055, 0.075, 0.098);
pub const DONE_BORDER: Color = Color::srgb(0.086, 0.114, 0.149);
pub const DONE_TEXT: Color = Color::srgb(0.235, 0.278, 0.337);

/// 单色模式的数字。
pub const NUM_PLAIN: Color = Color::srgb(0.878, 0.918, 0.961);
/// 双色模式的两组数字。刻意选高明度差，12px 点阵下也能一眼分开。
pub const NUM_WARM: Color = Color::srgb(1.0, 0.478, 0.404);
pub const NUM_COOL: Color = Color::srgb(0.416, 0.749, 1.0);

/// 光标框。
pub const CURSOR: Color = Color::srgb(0.361, 0.937, 0.702);
/// 点错时闪一下的格子底色。
pub const WRONG_FLASH: Color = Color::srgb(0.463, 0.145, 0.157);
