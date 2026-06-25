//! 太空射击配色板（原创配色，雷电 / 1942 风）。
//!
//! 敌机配色原先烘在 `EnemyKind` 方法里，现收敛到此处供 `sprites.rs` 与枚举共享。

use bevy::prelude::Color;

// 玩家战机：蓝色喷气战斗机
pub const P_BODY: Color = Color::srgb(0.44, 0.80, 0.97);
pub const P_BODY_HI: Color = Color::srgb(0.66, 0.92, 1.0);
pub const P_WING: Color = Color::srgb(0.30, 0.62, 0.90);
pub const P_WING_DK: Color = Color::srgb(0.16, 0.40, 0.66);
pub const P_CANOPY: Color = Color::srgb(0.12, 0.20, 0.38);
pub const P_CANOPY_HI: Color = Color::srgb(0.54, 0.74, 0.98);
pub const P_NOSE: Color = Color::srgb(0.92, 0.98, 1.0);
pub const P_THRUST: Color = Color::srgb(1.0, 0.70, 0.26);
pub const P_THRUST_HI: Color = Color::srgb(1.0, 0.92, 0.52);

// 通用描边 + 金属高光
pub const OUTLINE: Color = Color::srgb(0.04, 0.06, 0.12);
pub const METAL_HI: Color = Color::srgb(0.88, 0.94, 1.0);

// 敌机配色（body / 暗面），由 sprites.rs 各敌机 SpriteDef 直接引用
pub const COLOR_E_SCOUT: Color = Color::srgb(0.92, 0.36, 0.40);
pub const COLOR_E_SCOUT_DK: Color = Color::srgb(0.62, 0.16, 0.18);
pub const COLOR_E_SNIPER: Color = Color::srgb(0.96, 0.66, 0.30);
pub const COLOR_E_SNIPER_DK: Color = Color::srgb(0.62, 0.32, 0.10);
pub const COLOR_E_BOMBER: Color = Color::srgb(0.42, 0.60, 0.36);
pub const COLOR_E_BOMBER_DK: Color = Color::srgb(0.18, 0.34, 0.18);
pub const COLOR_E_CARRIER: Color = Color::srgb(0.92, 0.42, 0.86);
pub const COLOR_E_CARRIER_DK: Color = Color::srgb(0.55, 0.18, 0.50);
pub const COLOR_E_BOSS: Color = Color::srgb(0.78, 0.32, 0.36);
pub const COLOR_E_BOSS_DK: Color = Color::srgb(0.32, 0.12, 0.16);

// 敌机座舱/核心高光
pub const E_CORE: Color = Color::srgb(0.98, 0.86, 0.42);
pub const E_CORE_HI: Color = Color::srgb(1.0, 0.96, 0.74);

// 各敌机机身受光高光（金属体积感）
pub const COLOR_E_SCOUT_HI: Color = Color::srgb(1.0, 0.56, 0.58);
pub const COLOR_E_SNIPER_HI: Color = Color::srgb(1.0, 0.82, 0.52);
pub const COLOR_E_BOMBER_HI: Color = Color::srgb(0.62, 0.80, 0.54);
pub const COLOR_E_CARRIER_HI: Color = Color::srgb(1.0, 0.62, 0.98);
pub const COLOR_E_BOSS_HI: Color = Color::srgb(0.94, 0.50, 0.52);

// 道具：火力升级（红芯 P）
pub const PU_BG: Color = Color::srgb(1.0, 0.86, 0.32);
pub const PU_CORE: Color = Color::srgb(0.88, 0.30, 0.30);
pub const PU_MARK: Color = Color::srgb(1.0, 0.96, 0.80);
