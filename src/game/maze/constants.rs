use crate::game::model::AgeTier;

/// 每关的房间数（格坐标），瓦片网格是它的 2n+1。
pub const LEVEL_CELLS: [(usize, usize); 10] = [
    (4, 3), (5, 4), (6, 4), (6, 5), (7, 5), (8, 6), (8, 6), (9, 7), (10, 7), (10, 7),
];

/// 每关限时（秒）。
pub const LEVEL_TIME: [f32; 10] = [
    60.0, 75.0, 90.0, 105.0, 120.0, 140.0, 150.0, 175.0, 190.0, 210.0,
];

/// 第几关起出口上锁、要先捡钥匙。
pub const KEY_FROM_LEVEL: u8 = 4;
/// 第几关起开雾视野。
pub const FOG_FROM_LEVEL: u8 = 7;
/// 雾视野半径（瓦片，切比雪夫距离）。
pub const FOG_RADIUS: i32 = 7;

/// 5-7 岁档的房间数上限（对应 15×11 的瓦片网格），且永不开雾。
pub const JUNIOR_MAX_CELLS: (usize, usize) = (7, 5);

/// 长按连走的节奏。
pub const MOVE_COOLDOWN: f32 = 0.11;
/// 玩家追格子的插值速度。
pub const PLAYER_BLEND: f32 = 22.0;

/// 走回头路（踏进已经走过的格子）每步扣的分，结算时统一算。
pub const BACKTRACK_PENALTY: u32 = 1;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn cells_for(level: u8, age: AgeTier) -> (usize, usize) {
    let (w, h) = LEVEL_CELLS[level_index(level)];
    if age.is_junior() {
        (w.min(JUNIOR_MAX_CELLS.0), h.min(JUNIOR_MAX_CELLS.1))
    } else {
        (w, h)
    }
}

pub fn needs_key(level: u8) -> bool {
    level.clamp(1, 10) >= KEY_FROM_LEVEL
}

/// 低龄档永远不开雾：看不见路对这个年龄段是挫败而不是挑战。
pub fn fog_radius(level: u8, age: AgeTier) -> Option<i32> {
    (!age.is_junior() && level.clamp(1, 10) >= FOG_FROM_LEVEL).then_some(FOG_RADIUS)
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    let (fw, fh) = LEVEL_CELLS[level_index(level)];
    let (aw, ah) = cells_for(level, age);
    // 低龄档迷宫变小，按房间数折算基准时间，再乘年龄档的宽放系数。
    let shrink = (aw * ah) as f32 / (fw * fh) as f32;
    base * shrink * age.time_scale()
}
