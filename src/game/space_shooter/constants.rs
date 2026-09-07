pub const PLAY_W: f32 = 460.0;
pub const PLAY_H: f32 = 520.0;
pub const PLAY_OFFSET_X: f32 = -120.0;
pub const PLAY_LEFT: f32 = PLAY_OFFSET_X - PLAY_W * 0.5;
pub const PLAY_RIGHT: f32 = PLAY_OFFSET_X + PLAY_W * 0.5;
pub const PLAY_TOP: f32 = PLAY_H * 0.5;
pub const PLAY_BOTTOM: f32 = -PLAY_H * 0.5;

pub const PLAYER_SPEED: f32 = 280.0;
pub const PLAYER_FIRE_CD_LV1: f32 = 0.18;
pub const PLAYER_FIRE_CD_LV2: f32 = 0.12;
pub const PLAYER_FIRE_CD_LV3: f32 = 0.09;
pub const PLAYER_BULLET_SPEED: f32 = 640.0;
pub const ENEMY_BULLET_SPEED: f32 = 230.0;
pub const PLAYER_INVINCIBLE: f32 = 1.6;
pub const PLAYER_ROLL_TIME: f32 = 0.72;
pub const PLAYER_ROLL_INVINCIBLE: f32 = 0.82;
pub const PLAYER_RESPAWN_X: f32 = PLAY_OFFSET_X;
pub const PLAYER_RESPAWN_Y: f32 = PLAY_BOTTOM + 60.0;

// 层级：关卡底板(-10) < 战场底色 < 星空 < 精灵(0)
pub const Z_SPACE_BG: f32 = -2.0;
pub const Z_SPACE_BAND: f32 = -1.5;
pub const Z_STAR: f32 = -0.5;
pub const Z_ENEMY: f32 = 1.0;
pub const Z_BULLET: f32 = 1.4;
pub const Z_PLAYER: f32 = 1.6;
pub const Z_PARTICLE: f32 = 2.0;
pub const Z_POWERUP: f32 = 1.8;

pub const TOTAL_WAVES_BEFORE_BOSS: usize = 7;
