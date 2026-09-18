use crate::game::model::AgeTier;

/// 每关的起始序列长度。
pub const LEVEL_START_LEN: [usize; 10] = [3, 3, 4, 4, 5, 5, 5, 6, 6, 6];

/// 每关要连续答对多少轮（每轮序列加长 1）。
pub const LEVEL_ROUNDS: [u32; 10] = [3, 4, 4, 5, 5, 6, 6, 6, 7, 7];

/// 每关播放一个音符的时长（秒），越往后越快。
pub const LEVEL_STEP_TIME: [f32; 10] = [
    0.62, 0.58, 0.54, 0.50, 0.42, 0.38, 0.34, 0.32, 0.30, 0.28,
];

/// 第几关起要倒序复现。
pub const REVERSE_FROM_LEVEL: u8 = 8;

/// 音符之间的间隔占音符时长的比例。
pub const GAP_RATIO: f32 = 0.35;
/// 播放开始前的准备时间。
pub const READY_TIME: f32 = 0.9;
/// 一轮答对 / 答错之后的定格时间。
pub const FEEDBACK_TIME: f32 = 0.7;
/// 玩家每按一下的限时（秒）。
pub const INPUT_TIME: f32 = 3.2;
/// 按键按下后音板亮起的时长。
pub const PRESS_LIGHT_TIME: f32 = 0.18;

/// 音板版面（画布像素）。
pub const PAD_SIZE: f32 = 40.0;
pub const PAD_OFFSET: f32 = 44.0;
pub const CORE_SIZE: f32 = 30.0;

/// 5-7 岁档：播放放慢的倍率，且不考倒序。
pub const JUNIOR_SLOWDOWN: f32 = 1.5;
/// 5-7 岁档的起始长度上限。
pub const JUNIOR_MAX_START_LEN: usize = 4;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn start_len_for(level: u8, age: AgeTier) -> usize {
    let len = LEVEL_START_LEN[level_index(level)];
    if age.is_junior() {
        len.min(JUNIOR_MAX_START_LEN)
    } else {
        len
    }
}

pub fn rounds_for(level: u8) -> u32 {
    LEVEL_ROUNDS[level_index(level)]
}

pub fn step_time_for(level: u8, age: AgeTier) -> f32 {
    let base = LEVEL_STEP_TIME[level_index(level)];
    if age.is_junior() {
        base * JUNIOR_SLOWDOWN
    } else {
        base
    }
}

pub fn input_time_for(age: AgeTier) -> f32 {
    INPUT_TIME * age.time_scale()
}

pub fn reverse_for(level: u8, age: AgeTier) -> bool {
    !age.is_junior() && level.clamp(1, 10) >= REVERSE_FROM_LEVEL
}

/// 答对一轮的加分：序列越长给得越多。
pub fn round_score(sequence_len: usize) -> u32 {
    sequence_len as u32 * 10
}
