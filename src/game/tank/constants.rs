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
pub const POWERUP_SIZE: f32 = 28.0;

pub const PLAYER_SPEED: f32 = 110.0;
pub const ENEMY_SPEED_BASE: f32 = 70.0;
pub const PLAYER_BULLET_SPEED: f32 = 320.0;
pub const ENEMY_BULLET_SPEED: f32 = 220.0;
pub const PLAYER_FIRE_CD: f32 = 0.25;
pub const ENEMY_FIRE_CD: f32 = 1.4;
pub const SPAWN_SHIELD_TIME: f32 = 2.0;
pub const TURN_WINDOW: f32 = 6.0;
pub const ICE_COAST_TIME: f32 = 0.28;

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

// 关卡布局，13x13 全块格，按关卡循环选用。
// '.' 空 / 'b' 砖 / 's' 钢 / 'w' 水 / 'g' 草 / 'i' 冰 / 'E' 老巢
// 约定：第 0 行敌人出生列(0/6/12)留空；最后一行玩家出生格(4/8)留空、基地固定在列 6。
pub const STAGE_MAPS: [[&str; MAP_TILES_PER_SIDE as usize]; 2] = [
    [
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
    ],
    [
        ".............",
        ".bb.bb.bb.bb.",
        ".bb.bb.bb.bb.",
        ".............",
        "..s.s...s.s..",
        ".b.bbb.bbb.b.",
        ".b.b.www.b.b.",
        ".b.bbb.bbb.b.",
        "..s.s...s.s..",
        ".............",
        ".....ggg.....",
        ".....bbb.....",
        ".....bEb.....",
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_maps_are_structurally_valid() {
        let n = MAP_TILES_PER_SIDE as usize;
        for (i, map) in STAGE_MAPS.iter().enumerate() {
            assert_eq!(map.len(), n, "map {i} 行数应为 {n}");
            for (r, row) in map.iter().enumerate() {
                assert_eq!(row.chars().count(), n, "map {i} 第 {r} 行宽度应为 {n}");
            }
            let bases = map.iter().flat_map(|r| r.chars()).filter(|&c| c == 'E').count();
            assert_eq!(bases, 1, "map {i} 必须恰好一个基地");
            let row0: Vec<char> = map[0].chars().collect();
            for c in ENEMY_SPAWN_COLS {
                assert_eq!(row0[c as usize], '.', "map {i} 敌人出生列 {c} 被占");
            }
            let last: Vec<char> = map[n - 1].chars().collect();
            assert_eq!(last[PLAYER1_SPAWN.0 as usize], '.', "map {i} P1 出生格被占");
            assert_eq!(last[PLAYER2_SPAWN.0 as usize], '.', "map {i} P2 出生格被占");
        }
    }
}
