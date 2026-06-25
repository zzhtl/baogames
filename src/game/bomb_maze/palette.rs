//! 炸弹迷宫配色板（原创配色，思路接近经典 Bomberman）。

use bevy::prelude::Color;

pub const OUTLINE: Color = Color::srgb(0.06, 0.07, 0.10);

// 玩家：白色装甲炸弹人，P1 蓝盔 / P2 红盔
pub const P_SUIT: Color = Color::srgb(0.95, 0.96, 0.98);
pub const P_SUIT_DK: Color = Color::srgb(0.66, 0.70, 0.78);
pub const P1_HELM: Color = Color::srgb(0.32, 0.55, 0.95);
pub const P1_HELM_DK: Color = Color::srgb(0.16, 0.32, 0.66);
pub const P2_HELM: Color = Color::srgb(0.95, 0.42, 0.42);
pub const P2_HELM_DK: Color = Color::srgb(0.66, 0.20, 0.20);
pub const P_VISOR: Color = Color::srgb(0.55, 0.80, 1.0);
pub const P_FOOT: Color = Color::srgb(0.24, 0.24, 0.30);

// 敌人 4 种（body / 暗面）
pub const E_BALLOOM: Color = Color::srgb(0.86, 0.46, 0.86);
pub const E_BALLOOM_DK: Color = Color::srgb(0.52, 0.22, 0.54);
pub const E_ONEAL: Color = Color::srgb(0.95, 0.46, 0.32);
pub const E_ONEAL_DK: Color = Color::srgb(0.60, 0.22, 0.14);
pub const E_DOLL: Color = Color::srgb(0.46, 0.85, 0.46);
pub const E_DOLL_DK: Color = Color::srgb(0.20, 0.52, 0.22);
pub const E_KONDORIA: Color = Color::srgb(0.56, 0.46, 0.80);
pub const E_KONDORIA_DK: Color = Color::srgb(0.30, 0.22, 0.50);
// 球面顶部受光高光
pub const E_BALLOOM_HI: Color = Color::srgb(0.98, 0.62, 0.98);
pub const E_ONEAL_HI: Color = Color::srgb(1.0, 0.62, 0.48);
pub const E_DOLL_HI: Color = Color::srgb(0.62, 0.98, 0.62);
pub const E_KONDORIA_HI: Color = Color::srgb(0.72, 0.62, 0.95);
pub const E_EYE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const E_PUPIL: Color = Color::srgb(0.05, 0.05, 0.08);

// 硬墙（不可炸）：立体倒角
pub const HARD: Color = Color::srgb(0.55, 0.58, 0.62);
pub const HARD_HI: Color = Color::srgb(0.80, 0.84, 0.90);
pub const HARD_DK: Color = Color::srgb(0.30, 0.32, 0.36);
// 软砖（可炸）
pub const SOFT: Color = Color::srgb(0.74, 0.42, 0.18);
pub const SOFT_HI: Color = Color::srgb(0.88, 0.56, 0.30);
pub const SOFT_SEAM: Color = Color::srgb(0.42, 0.22, 0.08);

// 炸弹
pub const BOMB: Color = Color::srgb(0.10, 0.10, 0.13);
pub const BOMB_HI: Color = Color::srgb(0.36, 0.38, 0.44);
pub const BOMB_FUSE: Color = Color::srgb(0.52, 0.38, 0.18);
pub const BOMB_SPARK: Color = Color::srgb(1.0, 0.86, 0.32);

// 火焰
pub const FLAME_OUT: Color = Color::srgb(1.0, 0.52, 0.16);
pub const FLAME_MID: Color = Color::srgb(1.0, 0.78, 0.32);
pub const FLAME_CORE: Color = Color::srgb(1.0, 0.98, 0.82);

// 出口
pub const EXIT_BASE: Color = Color::srgb(0.14, 0.09, 0.20);
pub const EXIT_PORTAL: Color = Color::srgb(0.64, 0.46, 0.90);
pub const EXIT_ARROW: Color = Color::srgb(1.0, 0.96, 0.64);
