use crate::game::model::AgeTier;

/// 每关边长。
pub const LEVEL_SIZE: [usize; 10] = [3, 3, 4, 4, 4, 5, 5, 5, 5, 5];

/// 每关的打乱步数。
pub const LEVEL_SHUFFLE: [usize; 10] = [20, 40, 60, 90, 130, 150, 200, 260, 320, 400];

/// 每关限时（秒）。
pub const LEVEL_TIME: [f32; 10] = [
    90.0, 120.0, 180.0, 220.0, 260.0, 320.0, 380.0, 440.0, 500.0, 560.0,
];

/// 5-7 岁档封顶 4×4。
pub const JUNIOR_MAX_SIZE: usize = 4;

/// 块之间的缝隙占格宽的比例。
pub const TILE_GAP_RATIO: f32 = 0.08;

/// 滑动动画的插值速度。
pub const TILE_BLEND: f32 = 20.0;

/// 步数超过下界的这个倍数之后才开始扣效率分。
pub const EFFICIENCY_SLACK: f32 = 2.5;
/// 效率分满分。
pub const EFFICIENCY_BONUS: u32 = 60;

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

/// 低龄档打乱步数减半；被压小边长的关卡再按格数比例收一道。
pub fn shuffle_for(level: u8, age: AgeTier) -> usize {
    let steps = LEVEL_SHUFFLE[level_index(level)];
    if !age.is_junior() {
        return steps;
    }
    let full = LEVEL_SIZE[level_index(level)];
    let actual = size_for(level, age);
    let scaled = steps * actual * actual / (full * full);
    (scaled / 2).max(12)
}

pub fn time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_TIME[level_index(level)];
    let full = LEVEL_SIZE[level_index(level)];
    let actual = size_for(level, age);
    let shrink = (actual * actual) as f32 / (full * full) as f32;
    base * shrink * age.time_scale()
}

/// 效率奖励：步数在下界的 `EFFICIENCY_SLACK` 倍以内拿满分，之后每多一步扣 1。
pub fn efficiency_bonus(moves: u32, lower_bound: u32) -> u32 {
    let budget = (lower_bound as f32 * EFFICIENCY_SLACK) as u32;
    if moves <= budget {
        EFFICIENCY_BONUS
    } else {
        EFFICIENCY_BONUS.saturating_sub(moves - budget)
    }
}
