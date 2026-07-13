use crate::common::constants::ARENA_H;

pub const TILE: f32 = 36.0;
pub const FLOOR_Y: f32 = -ARENA_H * 0.5 + TILE * 0.5 + 9.0;

pub const ACTOR_SCALE: f32 = 1.5;
pub const PLAYER_W: f32 = 30.0;
pub const PLAYER_H: f32 = 42.0;
pub const BIG_PLAYER_H: f32 = 66.0;
pub const WALK_SPEED: f32 = 150.0;
pub const RUN_SPEED: f32 = 260.0;
pub const ACCEL: f32 = 700.0;
pub const DECEL: f32 = 900.0;
pub const AIR_ACCEL: f32 = 500.0;
pub const GRAVITY: f32 = 1700.0;
pub const JUMP_HOLD_GRAVITY: f32 = 700.0;
pub const JUMP_VEL_BASE: f32 = 520.0;
pub const JUMP_VEL_BONUS: f32 = 80.0;
pub const FALL_MAX: f32 = 700.0;
pub const STOMP_BOUNCE: f32 = 380.0;

pub const GOOMBA_W: f32 = 33.0;
pub const GOOMBA_H: f32 = 33.0;
pub const GOOMBA_SPEED: f32 = 55.0;
pub const GOOMBA_SQUASH_TIME: f32 = 0.45;

pub const POWERUP_SIZE: f32 = 33.0;
pub const MUSHROOM_SPEED: f32 = 90.0;
pub const POWERUP_EMERGE_SPEED: f32 = 36.0;

pub const FIREBALL_SIZE: f32 = 15.0;
pub const FIREBALL_SPEED: f32 = 380.0;
pub const FIREBALL_BOUNCE: f32 = 320.0;
pub const FIREBALL_LIFE: f32 = 2.0;
pub const FIRE_CD: f32 = 0.25;

pub const SHARD_SIZE: f32 = 12.0;
pub const SHARD_LIFE: f32 = 1.0;

pub const KOOPA_W: f32 = 33.0;
pub const KOOPA_H: f32 = 48.0;
pub const KOOPA_SPEED: f32 = 50.0;
pub const SHELL_SPEED: f32 = 360.0;
pub const SHELL_W: f32 = 33.0;
pub const SHELL_H: f32 = 30.0;

pub const PLATFORM_W: f32 = 108.0;
pub const PLATFORM_H: f32 = 12.0;
pub const PLATFORM_SPEED: f32 = 80.0;
pub const PLATFORM_RANGE: f32 = 120.0;

pub const BOWSER_W: f32 = 78.0;
pub const BOWSER_H: f32 = 84.0;
pub const BOWSER_SPEED: f32 = 40.0;
pub const BOWSER_HP: i32 = 5;
pub const BOWSER_FIRE_CD: f32 = 2.5;
pub const BOWSER_FIREBALL_SPEED: f32 = 240.0;

pub const AXE_SIZE: f32 = 24.0;
pub const LAVA_DAMAGE: f32 = 0.0;

pub const CAMERA_FOLLOW_OFFSET: f32 = 0.0;

pub const LEVEL_TIME: f32 = 300.0;
pub const FALL_DEATH_Y: f32 = -ARENA_H * 0.5 - 80.0;

pub const Z_BG_FAR: f32 = -8.0;
pub const Z_BG_MID: f32 = -6.0;
pub const Z_TILE: f32 = 0.4;
pub const Z_PIPE: f32 = 0.5;
pub const Z_DECOR: f32 = 0.8;
pub const Z_ENEMY: f32 = 1.0;
pub const Z_PLAYER: f32 = 1.2;
pub const Z_FLAG: f32 = 0.6;
pub const Z_COIN: f32 = 1.4;
pub const Z_POWERUP: f32 = 1.05;
pub const Z_FIREBALL: f32 = 1.3;
pub const Z_SHARD: f32 = 1.5;

pub const LEVEL_COLS: i32 = 160;
pub const LEVEL_ROWS: i32 = 14;
