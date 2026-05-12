pub const PLAY_W: f32 = 480.0;
pub const PLAY_H: f32 = 520.0;
pub const PLAY_OFFSET_X: f32 = -120.0;
pub const PLAY_LEFT: f32 = PLAY_OFFSET_X - PLAY_W * 0.5;
pub const PLAY_RIGHT: f32 = PLAY_OFFSET_X + PLAY_W * 0.5;
pub const PLAY_TOP: f32 = PLAY_H * 0.5;
pub const PLAY_BOTTOM: f32 = -PLAY_H * 0.5;

pub const BUBBLE_R: f32 = 18.0;
pub const BUBBLE_D: f32 = 36.0;
pub const ROW_H: f32 = 31.1769;
pub const COLS_EVEN: i32 = 12;
pub const COLS_ODD: i32 = 11;
pub const ROWS_MAX: usize = 14;
pub const TOP_PAD: f32 = 14.0;
pub const DEAD_Y: f32 = PLAY_BOTTOM + 70.0;

pub const CANNON_X: f32 = PLAY_OFFSET_X;
pub const CANNON_Y: f32 = PLAY_BOTTOM + 28.0;
pub const SHOT_SPEED: f32 = 720.0;
pub const MAX_AIM: f32 = 1.36;
pub const AIM_SPEED: f32 = 1.7;

pub const POP_LIFETIME: f32 = 0.22;
pub const FALL_GRAVITY: f32 = -780.0;

pub const Z_FRAME: f32 = 0.4;
pub const Z_BUBBLE: f32 = 1.2;
pub const Z_FLYING: f32 = 1.5;
pub const Z_CANNON: f32 = 2.0;
pub const Z_HUD: f32 = 8.0;

pub const COLORS_BY_LEVEL: [u8; 11] = [3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 6];
pub const INITIAL_ROWS_BY_LEVEL: [usize; 11] = [4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10];
pub const SHOTS_TO_DESCEND_BY_LEVEL: [i32; 11] = [12, 12, 11, 10, 10, 9, 9, 8, 8, 7, 6];
