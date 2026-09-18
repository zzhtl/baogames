use crate::game::model::AgeTier;

/// 每关的图元网格。
pub const LEVEL_GRID: [(i32, i32); 10] = [
    (4, 4), (4, 4), (5, 4), (5, 5), (5, 5), (6, 5), (6, 5), (6, 6), (6, 6), (6, 6),
];

/// 每关的不同点个数。
pub const LEVEL_DIFFS: [usize; 10] = [3, 3, 4, 4, 5, 5, 6, 6, 7, 8];

/// 每关限时（秒）。
pub const LEVEL_TIME: [f32; 10] = [
    60.0, 55.0, 70.0, 75.0, 85.0, 95.0, 105.0, 115.0, 125.0, 135.0,
];

/// 第几关起改动只发生在明暗上。
pub const SUBTLE_FROM_LEVEL: u8 = 6;

/// 5-7 岁档：网格与不同点上限，且永远不用"只改明暗"这种细微差别。
pub const JUNIOR_MAX_GRID: (i32, i32) = (5, 4);
pub const JUNIOR_MAX_DIFFS: usize = 4;

/// 两张图各自可用的宽度（世界单位）。两张加中缝正好落在 4:3 画布里。
pub const PANEL_W: f32 = 300.0;
pub const PANEL_H: f32 = 340.0;
pub const PANEL_GAP: f32 = 72.0;
pub const PANEL_CENTER_Y: f32 = -10.0;

/// 标错一次扣的秒数。
pub const WRONG_TIME_PENALTY: f32 = 3.0;
/// 找到一个不同点的得分。
pub const FOUND_SCORE: u32 = 20;
/// 标错一次扣的分（结算时统一算）。
pub const WRONG_SCORE_PENALTY: u32 = 5;

pub const CURSOR_BLEND: f32 = 24.0;

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

pub fn diffs_for(level: u8, age: AgeTier) -> usize {
    let wanted = LEVEL_DIFFS[level_index(level)];
    let wanted = if age.is_junior() {
        wanted.min(JUNIOR_MAX_DIFFS)
    } else {
        wanted
    };
    // 不同点不能多到超过格子数。
    let (w, h) = grid_for(level, age);
    wanted.min((w * h) as usize)
}

/// 低龄档永远只改形状或颜色这种一眼能看出的差别。
pub fn subtle_for(level: u8, age: AgeTier) -> bool {
    !age.is_junior() && level.clamp(1, 10) >= SUBTLE_FROM_LEVEL
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    let full = LEVEL_DIFFS[level_index(level)];
    let actual = diffs_for(level, age);
    // 低龄档不同点变少，按比例折算基准时间，再乘年龄档的宽放系数。
    let shrink = actual as f32 / full as f32;
    base * shrink * age.time_scale()
}
