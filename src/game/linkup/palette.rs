//! 连连看配色。

use bevy::prelude::Color;

pub const TILE_EDGE: Color = Color::srgb(0.157, 0.204, 0.278);
pub const TILE_FACE: Color = Color::srgb(0.937, 0.945, 0.910);
pub const TILE_SELECTED: Color = Color::srgb(0.361, 0.937, 0.702);
pub const CURSOR: Color = Color::srgb(1.0, 0.878, 0.322);
pub const PATH: Color = Color::srgb(0.361, 0.937, 0.702);

/// 12 种图案的字符与配色。字符取自常用汉字，12px 点阵下笔画不粘连。
pub const PATTERN_CHARS: [&str; 12] = [
    "猫", "狗", "兔", "熊", "猪", "羊", "鸭", "蛙", "虾", "蟹", "龟", "蝶",
];

pub const PATTERN_COLORS: [Color; 12] = [
    Color::srgb(0.90, 0.25, 0.32),
    Color::srgb(0.95, 0.52, 0.16),
    Color::srgb(0.85, 0.66, 0.10),
    Color::srgb(0.42, 0.66, 0.18),
    Color::srgb(0.13, 0.66, 0.36),
    Color::srgb(0.11, 0.62, 0.66),
    Color::srgb(0.16, 0.46, 0.88),
    Color::srgb(0.32, 0.32, 0.84),
    Color::srgb(0.56, 0.26, 0.86),
    Color::srgb(0.84, 0.30, 0.70),
    Color::srgb(0.84, 0.20, 0.48),
    Color::srgb(0.48, 0.34, 0.20),
];
