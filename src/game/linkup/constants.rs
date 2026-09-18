use crate::game::model::AgeTier;

/// 每关的棋盘大小。格数必须是偶数，否则配不成整对。
pub const LEVEL_GRID: [(i32, i32); 10] = [
    (4, 4), (6, 4), (6, 4), (6, 6), (8, 6), (8, 6), (8, 6), (10, 6), (10, 6), (10, 6),
];

/// 每关启用的图案种类数。
pub const LEVEL_PATTERNS: [usize; 10] = [4, 5, 6, 7, 8, 9, 10, 10, 11, 12];

/// 每关限时（秒）。
pub const LEVEL_TIME: [f32; 10] = [
    60.0, 80.0, 90.0, 120.0, 150.0, 155.0, 150.0, 190.0, 185.0, 180.0,
];

/// 第几关起消除后图案会往下掉。
pub const GRAVITY_FROM_LEVEL: u8 = 6;

/// 5-7 岁档的棋盘与图案上限，也不开下落。
pub const JUNIOR_MAX_GRID: (i32, i32) = (6, 4);
pub const JUNIOR_MAX_PATTERNS: usize = 6;

/// 图案之间的缝隙占格宽的比例。
pub const TILE_GAP_RATIO: f32 = 0.10;
pub const CURSOR_BLEND: f32 = 24.0;
/// 消除后连线在屏幕上停留的时间。
pub const PATH_SHOW_TIME: f32 = 0.35;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn grid_for(level: u8, age: AgeTier) -> (i32, i32) {
    let (w, h) = LEVEL_GRID[level_index(level)];
    if age.is_junior() {
        (w.min(JUNIOR_MAX_GRID.0), h.min(JUNIOR_MAX_GRID.1))
    } else {
        (w, h)
    }
}

pub fn patterns_for(level: u8, age: AgeTier) -> usize {
    let wanted = LEVEL_PATTERNS[level_index(level)];
    let wanted = if age.is_junior() {
        wanted.min(JUNIOR_MAX_PATTERNS)
    } else {
        wanted
    };
    // 图案种类不能多到配不成对：每种至少要能分到一对。
    let (w, h) = grid_for(level, age);
    wanted.clamp(1, (w * h / 2) as usize)
}

pub fn gravity_for(level: u8, age: AgeTier) -> bool {
    !age.is_junior() && level.clamp(1, 10) >= GRAVITY_FROM_LEVEL
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    let (fw, fh) = LEVEL_GRID[level_index(level)];
    let (aw, ah) = grid_for(level, age);
    let shrink = (aw * ah) as f32 / (fw * fh) as f32;
    base * shrink * age.time_scale()
}

/// 连续配对的加分：第 1 对 10 分，之后每对 +3，封顶 37。
pub fn hit_score(streak: u32) -> u32 {
    10 + streak.saturating_sub(1).min(9) * 3
}
