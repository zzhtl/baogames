//! 记忆翻翻乐配色（sprites.rs 与游戏渲染、离线预览共用）。

use bevy::prelude::Color;

// 卡背（紫色系，5 阶明暗）
pub const BACK_OUTLINE: Color = Color::srgb(0.20, 0.13, 0.38);
pub const BACK_DK: Color = Color::srgb(0.34, 0.23, 0.58);
pub const BACK_BASE: Color = Color::srgb(0.46, 0.33, 0.76);
pub const BACK_HI: Color = Color::srgb(0.58, 0.45, 0.88);
pub const BACK_GLOW: Color = Color::srgb(0.80, 0.70, 1.0);
pub const BACK_PATTERN: Color = Color::srgb(1.0, 0.88, 0.52);

// 卡面（米白，4 阶）
pub const FACE_OUTLINE: Color = Color::srgb(0.62, 0.50, 0.34);
pub const FACE_BASE: Color = Color::srgb(0.99, 0.97, 0.90);
pub const FACE_HI: Color = Color::srgb(1.0, 1.0, 0.98);
pub const FACE_DK: Color = Color::srgb(0.90, 0.84, 0.70);
pub const FACE_TRIM: Color = Color::srgb(0.90, 0.78, 0.52);

// 已配对卡面（金色，4 阶）
pub const DONE_OUTLINE: Color = Color::srgb(0.66, 0.42, 0.10);
pub const DONE_BASE: Color = Color::srgb(1.0, 0.87, 0.44);
pub const DONE_HI: Color = Color::srgb(1.0, 0.95, 0.68);
pub const DONE_DK: Color = Color::srgb(0.90, 0.70, 0.28);
