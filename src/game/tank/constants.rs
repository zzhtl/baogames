// 坦克大战的关卡常量与地图布局。

pub const SUBTILE: f32 = 16.0;
pub const MAP_TILES_PER_SIDE: i32 = 13;
pub const SUBTILES_PER_SIDE: i32 = MAP_TILES_PER_SIDE * 2;
pub const PLAY_SIZE: f32 = SUBTILES_PER_SIDE as f32 * SUBTILE;
pub const PLAY_OFFSET_X: f32 = -120.0;
pub const PLAY_OFFSET_Y: f32 = -10.0;

pub const TILE: f32 = 2.0 * SUBTILE;
pub const TANK_SIZE: f32 = 2.0 * SUBTILE;
pub const BULLET_SIZE: f32 = 6.0;

pub const PLAYER_SPEED: f32 = 110.0;
pub const ENEMY_SPEED_BASE: f32 = 70.0;
pub const PLAYER_BULLET_SPEED: f32 = 320.0;
pub const ENEMY_BULLET_SPEED: f32 = 220.0;
pub const PLAYER_FIRE_CD: f32 = 0.25;
pub const ENEMY_FIRE_CD: f32 = 1.4;
pub const SPAWN_SHIELD_TIME: f32 = 2.0;

pub const STAGE_TOTAL_ENEMIES: u8 = 20;
pub const MAX_ALIVE_ENEMIES: u8 = 4;
pub const SPAWN_INTERVAL: f32 = 3.0;
pub const RESPAWN_TIME: f32 = 1.2;

pub const Z_TILE: f32 = 0.5;
pub const Z_TANK: f32 = 1.0;
pub const Z_BULLET: f32 = 1.6;
pub const Z_BASE: f32 = 1.0;
pub const Z_BUSH: f32 = 3.0;

pub const ENEMY_SPAWN_COLS: [i32; 3] = [0, 6, 12];
pub const PLAYER1_SPAWN: (i32, i32) = (4, 12);
pub const PLAYER2_SPAWN: (i32, i32) = (8, 12);

// 一个简单的关卡布局，13x13 全块格。
// '.' 空 / 'b' 砖 / 's' 钢 / 'w' 水 / 'g' 草 / 'i' 冰 / 'E' 老巢
pub const STAGE_MAP: [&str; MAP_TILES_PER_SIDE as usize] = [
    ".............",
    ".bb.......bb.",
    ".bb.......bb.",
    "....s...s....",
    ".............",
    ".b.bb...bb.b.",
    ".b.b.....b.b.",
    ".b.b.www.b.b.",
    ".b.b.....b.b.",
    ".b.bb...bb.b.",
    ".....ggg.....",
    ".....bbb.....",
    ".....bEb.....",
];
