//! 数字华容道配色。

use bevy::prelude::Color;

pub const BOARD_BACK: Color = Color::srgb(0.063, 0.078, 0.110);
pub const TILE_EDGE: Color = Color::srgb(0.780, 0.541, 0.176);
pub const TILE_FACE: Color = Color::srgb(0.949, 0.749, 0.365);
/// 已经归位的块换成绿色，进度一眼可见。
pub const TILE_DONE_EDGE: Color = Color::srgb(0.180, 0.502, 0.259);
pub const TILE_DONE_FACE: Color = Color::srgb(0.443, 0.812, 0.463);
pub const TILE_TEXT: Color = Color::srgb(0.102, 0.086, 0.063);
