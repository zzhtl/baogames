use crate::common::constants::ARENA_H;

pub const TILE: f32 = 24.0;
// 地面抬到屏幕垂直中线偏下，让岩壁断面占满屏幕下半部
pub const FLOOR_Y: f32 = -ARENA_H * 0.5 + 250.0;
pub const GROUND_TOP: f32 = FLOOR_Y + TILE * 0.5;

pub const PLAYER_W: f32 = 18.0;
pub const PLAYER_H: f32 = 32.0;
pub const PRONE_H: f32 = 14.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const GRAVITY: f32 = 1500.0;
pub const JUMP_VEL: f32 = 480.0;
pub const FALL_MAX: f32 = 800.0;

pub const BULLET_SIZE: f32 = 6.0;
pub const BULLET_BIG_SIZE: f32 = 9.0;
pub const FLAME_SIZE: f32 = 16.0;
pub const LASER_SIZE: f32 = 9.0;
pub const PLAYER_BULLET_SPEED: f32 = 620.0;
pub const FLAME_SPEED: f32 = 360.0;
pub const LASER_SPEED: f32 = 760.0;
pub const ENEMY_BULLET_SPEED: f32 = 280.0;
pub const FIRE_CD_M: f32 = 0.20;
pub const FIRE_CD_R: f32 = 0.10;
pub const FIRE_CD_S: f32 = 0.20;
pub const FIRE_CD_F: f32 = 0.34;
pub const FIRE_CD_L: f32 = 0.30;
pub const BULLET_LIFE: f32 = 1.6;

pub const ENEMY_W: f32 = 18.0;
pub const ENEMY_H: f32 = 28.0;
pub const ENEMY_SPEED: f32 = 78.0;
pub const ENEMY_FIRE_CD: f32 = 1.6;
pub const SNIPER_FIRE_CD: f32 = 1.4;
pub const GUNNER_FIRE_CD: f32 = 0.9;
pub const JUMPER_JUMP_VEL: f32 = 360.0;

pub const FALCON_W: f32 = 32.0;
pub const FALCON_H: f32 = 18.0;
pub const FALCON_SPEED: f32 = 150.0;
pub const PICKUP_SIZE: f32 = 22.0;
pub const PICKUP_FALL_SPEED: f32 = 110.0;

pub const BOSS_W: f32 = 220.0;
pub const BOSS_H: f32 = 240.0;
pub const BOSS_HP: i32 = 30;
pub const TURRET_W: f32 = 42.0;
pub const TURRET_H: f32 = 26.0;
pub const TURRET_FIRE_CD: f32 = 1.5;
pub const TURRET_HP: i32 = 6;

pub const CAMERA_FOLLOW_OFFSET: f32 = 60.0;
pub const FALL_DEATH_Y: f32 = -ARENA_H * 0.5 - 40.0;
pub const WORLD_W: f32 = 4400.0;
pub const RESPAWN_TIME: f32 = 1.4;
pub const INVINCIBLE_TIME: f32 = 1.6;
pub const ENEMY_DESPAWN_BACK: f32 = 200.0;

pub const Z_BG: f32 = -8.0;
pub const Z_BG2: f32 = -6.0;
pub const Z_TILE: f32 = 0.4;
pub const Z_PLATFORM: f32 = 0.45;
pub const Z_BOSS: f32 = 0.6;
pub const Z_PICKUP: f32 = 1.05;
pub const Z_ENEMY: f32 = 1.0;
pub const Z_PLAYER: f32 = 1.2;
pub const Z_BULLET: f32 = 1.4;
pub const Z_EXPL: f32 = 1.6;
