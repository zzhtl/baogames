//! 迷宫探险配色。

use bevy::prelude::Color;

pub const WALL: Color = Color::srgb(0.157, 0.271, 0.196);
pub const WALL_TOP: Color = Color::srgb(0.231, 0.388, 0.271);
pub const FLOOR: Color = Color::srgb(0.063, 0.106, 0.086);
/// 走过的地面留一道浅痕，回头路一眼能看出来。
pub const FLOOR_TRAIL: Color = Color::srgb(0.098, 0.169, 0.133);

pub const PLAYER: Color = Color::srgb(1.0, 0.878, 0.400);
pub const PLAYER_DARK: Color = Color::srgb(0.851, 0.639, 0.180);
pub const KEY: Color = Color::srgb(0.996, 0.831, 0.310);
pub const EXIT_OPEN: Color = Color::srgb(0.447, 0.878, 0.427);
pub const EXIT_LOCKED: Color = Color::srgb(0.878, 0.353, 0.322);
