//! 记忆序列配色：四个音板用经典的绿 / 红 / 黄 / 蓝。
//!
//! 每个音板有"暗"和"亮"两档，亮起时明度拉开一大截 —— 12px 画布上只靠饱和度
//! 区分是看不出来的。

use bevy::prelude::Color;

pub const PAD_DIM: [Color; 4] = [
    Color::srgb(0.106, 0.400, 0.208),
    Color::srgb(0.435, 0.118, 0.137),
    Color::srgb(0.451, 0.361, 0.071),
    Color::srgb(0.106, 0.239, 0.478),
];

pub const PAD_LIT: [Color; 4] = [
    Color::srgb(0.443, 0.949, 0.549),
    Color::srgb(1.0, 0.475, 0.478),
    Color::srgb(1.0, 0.898, 0.318),
    Color::srgb(0.475, 0.741, 1.0),
];

pub const PAD_EDGE: Color = Color::srgb(0.063, 0.078, 0.110);
pub const CORE: Color = Color::srgb(0.086, 0.106, 0.149);
pub const CORE_EDGE: Color = Color::srgb(0.204, 0.239, 0.298);
