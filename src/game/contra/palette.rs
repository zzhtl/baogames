// 8-bit 丛林关调色板（原创配色，思路接近经典 FC 美术）

use bevy::prelude::Color;

// 夜空：底部最深、向上略微提亮的三段渐变
pub const COLOR_SKY: Color = Color::srgb(0.02, 0.02, 0.06);
pub const COLOR_SKY_MID: Color = Color::srgb(0.04, 0.05, 0.14);
pub const COLOR_SKY_HI: Color = Color::srgb(0.08, 0.10, 0.24);
pub const COLOR_STAR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_STAR_DIM: Color = Color::srgb(0.78, 0.82, 0.96);

// 雪山：远山纯白雪顶 + 山体阴影面，近山再叠一层深灰
pub const COLOR_MOUNTAIN: Color = Color::srgb(0.96, 0.96, 0.98);
pub const COLOR_MOUNTAIN_SHADE: Color = Color::srgb(0.56, 0.58, 0.66);
pub const COLOR_MOUNTAIN_TOP: Color = Color::srgb(0.92, 0.20, 0.12);
pub const COLOR_MOUNTAIN_DARK: Color = Color::srgb(0.05, 0.05, 0.08);
pub const COLOR_MOUNTAIN_FAR: Color = Color::srgb(0.30, 0.32, 0.42);

// 丛林：从远到近三层绿，越靠前越深
pub const COLOR_JUNGLE_FAR: Color = Color::srgb(0.10, 0.32, 0.18);
pub const COLOR_PALM_LIGHT: Color = Color::srgb(0.30, 0.74, 0.18);
pub const COLOR_PALM_DARK: Color = Color::srgb(0.04, 0.34, 0.10);
pub const COLOR_PALM_TRUNK: Color = Color::srgb(0.46, 0.26, 0.10);

// 草皮
pub const COLOR_GRASS_BRIGHT: Color = Color::srgb(0.34, 0.80, 0.20);
pub const COLOR_GRASS_MID: Color = Color::srgb(0.20, 0.58, 0.12);
pub const COLOR_GRASS_DARK: Color = Color::srgb(0.04, 0.30, 0.08);

// 石壁：高光 / 中色 / 暗色 / 描边 + 缝隙
pub const COLOR_ROCK_HI: Color = Color::srgb(0.99, 0.78, 0.40);
pub const COLOR_ROCK_MID: Color = Color::srgb(0.78, 0.46, 0.14);
pub const COLOR_ROCK_LO: Color = Color::srgb(0.42, 0.22, 0.06);
pub const COLOR_ROCK_OUT: Color = Color::srgb(0.02, 0.02, 0.02);
pub const COLOR_ROCK_SEAM: Color = Color::srgb(0.22, 0.10, 0.02);

pub const COLOR_PLATFORM: Color = Color::srgb(0.78, 0.46, 0.14);
pub const COLOR_PLATFORM_TOP: Color = Color::srgb(0.34, 0.80, 0.20);

// 水面：底色 + 高光横纹 + 白色泡沫边
pub const COLOR_WATER: Color = Color::srgb(0.16, 0.34, 0.96);
pub const COLOR_WATER_DEEP: Color = Color::srgb(0.06, 0.14, 0.56);
pub const COLOR_WATER_HI: Color = Color::srgb(0.62, 0.82, 1.00);
pub const COLOR_FOAM: Color = Color::srgb(1.00, 1.00, 1.00);
pub const COLOR_BRIDGE_PLANK: Color = Color::srgb(0.78, 0.46, 0.14);
pub const COLOR_BRIDGE_PLANK_DK: Color = Color::srgb(0.42, 0.22, 0.06);
pub const COLOR_BRIDGE_ROPE: Color = Color::srgb(0.99, 0.86, 0.46);

// 主角：丛林兵风格（头巾 / 露肩 / 短裤 / 战靴），原创配色
pub const COLOR_PLAYER_SKIN: Color = Color::srgb(0.98, 0.78, 0.56);
pub const COLOR_PLAYER_SKIN_DK: Color = Color::srgb(0.74, 0.50, 0.28);
pub const COLOR_PLAYER_HAIR: Color = Color::srgb(0.20, 0.10, 0.04);
pub const COLOR_PLAYER_HELMET: Color = Color::srgb(0.96, 0.74, 0.30);
pub const COLOR_PLAYER_HELMET_DK: Color = Color::srgb(0.60, 0.36, 0.10);
pub const COLOR_PLAYER_BODY: Color = Color::srgb(0.32, 0.72, 0.20);
pub const COLOR_PLAYER_BODY_DK: Color = Color::srgb(0.10, 0.38, 0.10);
pub const COLOR_PLAYER_PANTS: Color = Color::srgb(0.14, 0.22, 0.62);
pub const COLOR_PLAYER_PANTS_DK: Color = Color::srgb(0.06, 0.10, 0.34);
pub const COLOR_PLAYER_BOOT: Color = Color::srgb(0.30, 0.14, 0.04);
pub const COLOR_PLAYER_GUN: Color = Color::srgb(0.20, 0.20, 0.24);
pub const COLOR_PLAYER_GUN_HI: Color = Color::srgb(0.62, 0.62, 0.68);
pub const COLOR_PLAYER_OUTLINE: Color = Color::srgb(0.04, 0.04, 0.06);
pub const COLOR_PLAYER_BANDOLIER: Color = Color::srgb(0.96, 0.78, 0.28);

// 敌兵：深色制服 + 红头盔，与主角强烈区分
pub const COLOR_ENEMY_BODY: Color = Color::srgb(0.86, 0.26, 0.18);
pub const COLOR_ENEMY_BODY_DK: Color = Color::srgb(0.46, 0.12, 0.08);
pub const COLOR_ENEMY_HAT: Color = Color::srgb(0.96, 0.74, 0.30);
pub const COLOR_ENEMY_RED: Color = Color::srgb(0.74, 0.12, 0.10);
pub const COLOR_ENEMY_BLUE: Color = Color::srgb(0.22, 0.38, 0.96);
pub const COLOR_ENEMY_SKIN: Color = Color::srgb(0.98, 0.78, 0.56);
pub const COLOR_ENEMY_PANTS: Color = Color::srgb(0.66, 0.52, 0.16);
pub const COLOR_ENEMY_PANTS_DK: Color = Color::srgb(0.32, 0.22, 0.06);
pub const COLOR_ENEMY_GUN: Color = Color::srgb(0.10, 0.10, 0.12);
pub const COLOR_ENEMY_OUTLINE: Color = Color::srgb(0.04, 0.04, 0.06);

pub const COLOR_BULLET_P: Color = Color::srgb(1.0, 0.95, 0.40);
pub const COLOR_BULLET_E: Color = Color::srgb(1.0, 0.40, 0.32);
pub const COLOR_FLAME_CORE: Color = Color::srgb(1.0, 0.84, 0.30);

pub const COLOR_FALCON: Color = Color::srgb(0.88, 0.88, 0.96);
pub const COLOR_FALCON_DARK: Color = Color::srgb(0.32, 0.32, 0.38);
pub const COLOR_FALCON_BEAK: Color = Color::srgb(1.0, 0.78, 0.18);
pub const COLOR_PICKUP_BG: Color = Color::srgb(0.06, 0.06, 0.08);
pub const COLOR_PICKUP_M: Color = Color::srgb(0.42, 0.92, 0.32);
pub const COLOR_PICKUP_S: Color = Color::srgb(1.00, 0.78, 0.18);
pub const COLOR_PICKUP_F: Color = Color::srgb(1.00, 0.42, 0.16);
pub const COLOR_PICKUP_R: Color = Color::srgb(0.46, 0.78, 1.00);

pub const COLOR_BOSS_WALL: Color = Color::srgb(0.42, 0.46, 0.52);
pub const COLOR_BOSS_WALL_DARK: Color = Color::srgb(0.22, 0.24, 0.30);
pub const COLOR_BOSS_TRIM: Color = Color::srgb(0.78, 0.32, 0.22);
pub const COLOR_TURRET: Color = Color::srgb(0.30, 0.30, 0.36);
pub const COLOR_TURRET_BARREL: Color = Color::srgb(0.20, 0.20, 0.24);
pub const COLOR_BOSS_CORE: Color = Color::srgb(0.96, 0.40, 0.22);
pub const COLOR_BOSS_CORE_HI: Color = Color::srgb(1.0, 0.78, 0.42);
pub const COLOR_EXPL_HOT: Color = Color::srgb(1.0, 0.92, 0.36);
pub const COLOR_EXPL_MID: Color = Color::srgb(1.0, 0.55, 0.18);
pub const COLOR_EXPL_OUT: Color = Color::srgb(0.78, 0.18, 0.10);
