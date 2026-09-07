// 炸弹迷宫所有数值常量。

pub const BM_COLS: i32 = 15;
pub const BM_ROWS: i32 = 11;
pub const BM_TILE: f32 = 36.0;
// 迷宫 180 画布像素宽：原来的 -100 会让最左一列落到可见区(±120 像素)之外
pub const BM_OFFSET_X: f32 = -84.0;
pub const BM_OFFSET_Y: f32 = -10.0;
pub const BM_PLAY_W: f32 = BM_COLS as f32 * BM_TILE;
pub const BM_PLAY_H: f32 = BM_ROWS as f32 * BM_TILE;

pub const BM_PLAYER_SIZE: f32 = 24.0;
pub const BM_ENEMY_SIZE: f32 = 24.0;
pub const BM_BOMB_SIZE: f32 = 28.0;
pub const BM_FLAME_SIZE: f32 = 32.0;
pub const BM_PLAYER_SPEED_BASE: f32 = 100.0;
pub const BM_PLAYER_SPEED_BONUS: f32 = 22.0;
pub const BM_BOMB_FUSE: f32 = 2.6;
pub const BM_FLAME_LIFE: f32 = 0.55;
pub const BM_PLACE_CD: f32 = 0.16;
pub const BM_RESPAWN_TIME: f32 = 1.4;
pub const BM_INVULN_TIME: f32 = 1.6;
pub const BM_TURN_WINDOW: f32 = 5.0;

pub const Z_FLOOR: f32 = -1.0;
pub const Z_GRID: f32 = -0.8;
pub const Z_TILE: f32 = 0.5;
pub const Z_ITEM: f32 = 0.4;
pub const Z_BOMB: f32 = 1.0;
pub const Z_ACTOR: f32 = 1.2;
pub const Z_FLAME: f32 = 1.6;

pub const P1_SPAWN: (i32, i32) = (1, 1);
pub const P2_SPAWN: (i32, i32) = (1, 9);

// 软砖生成概率（除安全区）
pub const SOFT_WALL_DENSITY: f32 = 0.62;
