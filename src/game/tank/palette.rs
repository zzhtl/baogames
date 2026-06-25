//! 坦克大战配色板（原创配色，思路接近经典 FC Battle City）。
//!
//! 原先颜色硬编码在 setup.rs，现收敛成具名常量供 sprites.rs 与游戏共享。

use bevy::prelude::Color;

// 玩家坦克：P1 金黄、P2 蓝（与菜单/HUD 的 P1/P2 主题一致）
pub const COLOR_TANK_P1: Color = Color::srgb(0.86, 0.72, 0.22);
pub const COLOR_TANK_P1_DK: Color = Color::srgb(0.52, 0.40, 0.08);
pub const COLOR_TANK_P2: Color = Color::srgb(0.42, 0.64, 0.92);
pub const COLOR_TANK_P2_DK: Color = Color::srgb(0.18, 0.34, 0.62);

// 敌坦克 4 型：普通灰 / 快速青 / 重炮褐 / 装甲绿
pub const COLOR_TANK_E_BASIC: Color = Color::srgb(0.76, 0.76, 0.80);
pub const COLOR_TANK_E_BASIC_DK: Color = Color::srgb(0.42, 0.42, 0.48);
pub const COLOR_TANK_E_FAST: Color = Color::srgb(0.70, 0.86, 0.92);
pub const COLOR_TANK_E_FAST_DK: Color = Color::srgb(0.34, 0.52, 0.60);
pub const COLOR_TANK_E_POWER: Color = Color::srgb(0.86, 0.72, 0.48);
pub const COLOR_TANK_E_POWER_DK: Color = Color::srgb(0.50, 0.38, 0.18);
pub const COLOR_TANK_E_ARMOR: Color = Color::srgb(0.56, 0.78, 0.46);
pub const COLOR_TANK_E_ARMOR_DK: Color = Color::srgb(0.28, 0.46, 0.22);

// 各坦克车体受光高光色（4 阶明暗用）
pub const COLOR_TANK_P1_HI: Color = Color::srgb(1.0, 0.88, 0.42);
pub const COLOR_TANK_P2_HI: Color = Color::srgb(0.64, 0.84, 1.0);
pub const COLOR_TANK_E_BASIC_HI: Color = Color::srgb(0.92, 0.92, 0.96);
pub const COLOR_TANK_E_FAST_HI: Color = Color::srgb(0.88, 0.98, 1.0);
pub const COLOR_TANK_E_POWER_HI: Color = Color::srgb(1.0, 0.88, 0.62);
pub const COLOR_TANK_E_ARMOR_HI: Color = Color::srgb(0.74, 0.94, 0.60);

// 坦克通用部件
pub const COLOR_TANK_TREAD: Color = Color::srgb(0.20, 0.20, 0.24);
pub const COLOR_TANK_TREAD_HI: Color = Color::srgb(0.44, 0.44, 0.50);
pub const COLOR_TANK_BARREL: Color = Color::srgb(0.12, 0.12, 0.14);
pub const COLOR_TANK_OUTLINE: Color = Color::srgb(0.04, 0.04, 0.06);

// 地形
pub const COLOR_BRICK: Color = Color::srgb(0.74, 0.40, 0.18);
pub const COLOR_BRICK_DK: Color = Color::srgb(0.46, 0.22, 0.08);
pub const COLOR_BRICK_HI: Color = Color::srgb(0.90, 0.56, 0.30);
pub const COLOR_STEEL: Color = Color::srgb(0.72, 0.76, 0.82);
pub const COLOR_STEEL_HI: Color = Color::srgb(0.93, 0.97, 1.0);
pub const COLOR_STEEL_DK: Color = Color::srgb(0.44, 0.48, 0.56);
pub const COLOR_WATER: Color = Color::srgb(0.16, 0.38, 0.78);
pub const COLOR_WATER_DK: Color = Color::srgb(0.08, 0.22, 0.52);
pub const COLOR_WATER_HI: Color = Color::srgb(0.52, 0.76, 1.0);
pub const COLOR_BUSH: Color = Color::srgb(0.16, 0.56, 0.22);
pub const COLOR_BUSH_HI: Color = Color::srgb(0.32, 0.76, 0.34);
pub const COLOR_BUSH_DK: Color = Color::srgb(0.08, 0.34, 0.12);
pub const COLOR_ICE: Color = Color::srgb(0.80, 0.92, 1.0);
pub const COLOR_ICE_HI: Color = Color::srgb(0.96, 0.99, 1.0);
pub const COLOR_ICE_DK: Color = Color::srgb(0.60, 0.76, 0.92);
pub const COLOR_BASE: Color = Color::srgb(0.20, 0.20, 0.24);
pub const COLOR_BASE_EAGLE: Color = Color::srgb(0.95, 0.85, 0.30);
pub const COLOR_BASE_EAGLE_DK: Color = Color::srgb(0.55, 0.40, 0.10);

// 道具徽章：金边 + 暗底 + 各色符号
pub const COLOR_PU_BORDER: Color = Color::srgb(0.96, 0.84, 0.28);
pub const COLOR_PU_BG: Color = Color::srgb(0.10, 0.10, 0.14);
pub const COLOR_PU_STAR: Color = Color::srgb(1.0, 0.86, 0.24);
pub const COLOR_PU_GRENADE: Color = Color::srgb(0.42, 0.82, 0.40);
pub const COLOR_PU_HELMET: Color = Color::srgb(0.50, 0.74, 1.0);
pub const COLOR_PU_TANK: Color = Color::srgb(0.96, 0.56, 0.34);
pub const COLOR_PU_CLOCK: Color = Color::srgb(0.78, 0.60, 1.0);
pub const COLOR_PU_SHOVEL: Color = Color::srgb(0.80, 0.60, 0.36);
