use crate::game::model::AgeTier;

use super::generator::SudokuSpec;

/// 每关的边长：先 4×4 上手，再 6×6，最后 9×9。
pub const LEVEL_SIZE: [usize; 10] = [4, 4, 4, 6, 6, 6, 9, 9, 9, 9];

/// 每关想挖掉的格数。实际挖掉的可能少一点 —— 唯一解优先。
pub const LEVEL_HOLES: [usize; 10] = [6, 8, 10, 14, 18, 22, 36, 42, 48, 54];

/// 每关限时（秒）。
pub const LEVEL_TIME: [f32; 10] = [
    120.0, 150.0, 180.0, 260.0, 300.0, 340.0, 540.0, 600.0, 660.0, 720.0,
];

/// 5-7 岁档封顶 6×6：9×9 的扫描负担对这个年龄段太重。
pub const JUNIOR_MAX_SIZE: usize = 6;

/// 每种边长安全的挖空上限。再多就很难保住唯一解，白白浪费生成时间。
pub fn max_holes(size: usize) -> usize {
    match size {
        4 => 10,
        6 => 24,
        _ => 55,
    }
}

/// 棋盘可用区域：数独格子里要塞 12 像素的数字，比默认棋盘区多要一点高度。
/// 432 世界单位 = 144 画布像素，上下各到 ±72，正好卡在 HUD 通栏内沿（±74.5）里面。
pub const BOARD_W: f32 = 660.0;
pub const BOARD_H: f32 = 432.0;
pub const BOARD_CENTER_Y: f32 = 0.0;

pub const CURSOR_BLEND: f32 = 24.0;

/// 填错一格（造成冲突）扣的分，结算时统一算。
pub const CONFLICT_PENALTY: u32 = 5;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn size_for(level: u8, age: AgeTier) -> usize {
    let size = LEVEL_SIZE[level_index(level)];
    if age.is_junior() {
        size.min(JUNIOR_MAX_SIZE)
    } else {
        size
    }
}

pub fn spec_for(level: u8, age: AgeTier) -> SudokuSpec {
    SudokuSpec::for_size(size_for(level, age))
}

/// 低龄档被压回 6×6 后，用更多挖空把难度补回来，但不越过唯一解的安全线。
///
/// 不能按格数比例折算 9×9 的挖空数 —— 那样第 7 关（6×6 挖 20）会比第 6 关
/// （6×6 挖 22）还简单。正确做法是接着「原表里这个边长的最后一关」往
/// `max_holes` 递进。剩下的难度靠限时收紧（`time_for` 里按格数折算基准时间）。
pub fn holes_for(level: u8, age: AgeTier) -> usize {
    let idx = level_index(level);
    let size = size_for(level, age);
    let natural = LEVEL_SIZE[idx];
    let ceiling = max_holes(size);
    if !age.is_junior() || natural == size {
        return LEVEL_HOLES[idx].min(ceiling);
    }
    let last_same_size = LEVEL_SIZE.iter().rposition(|&s| s == size).unwrap_or(idx);
    let base = LEVEL_HOLES[last_same_size].min(ceiling);
    let spread = (LEVEL_SIZE.len() - last_same_size - 1).max(1);
    let step = idx - last_same_size;
    base + (ceiling - base) * step / spread
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    let full = LEVEL_SIZE[level_index(level)];
    let actual = size_for(level, age);
    // 低龄档格子变少，按格数折算基准时间，再乘上年龄档的宽放系数。
    let shrink = (actual * actual) as f32 / (full * full) as f32;
    base * shrink * age.time_scale()
}
