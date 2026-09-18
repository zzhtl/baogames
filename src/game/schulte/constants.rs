use crate::game::model::AgeTier;

use super::resources::SchulteMode;

/// 每关的边长。1-4 关顺序、5-7 关倒序、8-10 关双色，难度同时靠边长和模式推进。
pub const LEVEL_SIZE: [i32; 10] = [3, 3, 4, 4, 5, 5, 5, 6, 6, 6];

/// 每关限时（秒）。换更大的格子时回宽一点，模式内再逐关收紧。
pub const LEVEL_TIME: [f32; 10] = [
    70.0, 60.0, 90.0, 78.0, 120.0, 108.0, 96.0, 155.0, 140.0, 128.0,
];

/// 5-7 岁档的边长上限：6×6 对这个年龄段的视觉搜索负担过重。
pub const JUNIOR_MAX_SIZE: i32 = 4;

/// 点错一次扣的秒数。
pub const MISTAKE_TIME_PENALTY: f32 = 2.0;

/// 光标追格子的插值速度。
pub const CURSOR_BLEND: f32 = 24.0;

/// 格子之间留的缝隙占格宽的比例。
pub const CELL_GAP_RATIO: f32 = 0.10;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn size_for(level: u8, age: AgeTier) -> i32 {
    let size = LEVEL_SIZE[level_index(level)];
    if age.is_junior() {
        size.min(JUNIOR_MAX_SIZE)
    } else {
        size
    }
}

pub fn mode_for(level: u8, age: AgeTier) -> SchulteMode {
    if age.is_junior() {
        return SchulteMode::Ascending;
    }
    match level.clamp(1, 10) {
        1..=4 => SchulteMode::Ascending,
        5..=7 => SchulteMode::Descending,
        _ => SchulteMode::DualColor,
    }
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    // 低龄档格子被压到 4×4 上限时，按边长比例回收一部分时间，否则太宽松。
    let full = LEVEL_SIZE[level_index(level)];
    let actual = size_for(level, age);
    let shrink = (actual * actual) as f32 / (full * full) as f32;
    base * shrink * age.time_scale()
}

/// 连续点对的加分：第 1 次 10 分，之后每次 +3，封顶 37。
pub fn hit_score(streak: u32) -> u32 {
    10 + streak.saturating_sub(1).min(9) * 3
}
