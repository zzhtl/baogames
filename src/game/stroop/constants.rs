use crate::game::model::AgeTier;

use super::resources::RuleMode;

/// 每关要答对多少题。
pub const LEVEL_QUESTIONS: [u32; 10] = [10, 12, 14, 16, 18, 18, 20, 20, 20, 20];

/// 每题限时（秒），逐关收紧。
pub const LEVEL_Q_TIME: [f32; 10] = [4.0, 3.6, 3.2, 3.0, 2.6, 2.4, 2.2, 2.0, 1.8, 1.6];

/// 每关启用的颜色数：先四色，第 5 关起上到六色。
pub const LEVEL_COLORS: [usize; 10] = [4, 4, 4, 4, 6, 6, 6, 6, 6, 6];

/// 5-7 岁档固定四色。
pub const JUNIOR_COLORS: usize = 4;

/// 关卡总时长 = 题数 × 每题限时 × 这个宽裕系数。
pub const LEVEL_TIME_SLACK: f32 = 1.35;

/// 答错扣的秒数。
pub const WRONG_TIME_PENALTY: f32 = 1.5;

/// 出题时"字义与墨色不同"的目标比例。全同色就没有干扰，全不同又太机械。
pub const INCONGRUENT_RATIO: f64 = 0.75;

/// 色块版面（画布像素）。
pub const SWATCH_W: f32 = 28.0;
pub const SWATCH_H: f32 = 20.0;
pub const SWATCH_GAP: f32 = 6.0;
pub const SWATCH_Y: f32 = -46.0;
/// 题面汉字与规则提示的高度。
pub const WORD_Y: f32 = 10.0;
pub const PROMPT_Y: f32 = 46.0;

pub fn level_index(level: u8) -> usize {
    (level.clamp(1, 10) - 1) as usize
}

pub fn colors_for(level: u8, age: AgeTier) -> usize {
    if age.is_junior() {
        JUNIOR_COLORS
    } else {
        LEVEL_COLORS[level_index(level)]
    }
}

pub fn questions_for(level: u8) -> u32 {
    LEVEL_QUESTIONS[level_index(level)]
}

pub fn question_time_for(level: u8, age: AgeTier) -> f32 {
    LEVEL_Q_TIME[level_index(level)] * age.time_scale()
}

pub fn level_time_for(level: u8, age: AgeTier) -> f32 {
    questions_for(level) as f32 * question_time_for(level, age) * LEVEL_TIME_SLACK
}

pub fn rule_mode_for(level: u8, age: AgeTier) -> RuleMode {
    if age.is_junior() {
        return RuleMode::Fixed;
    }
    match level.clamp(1, 10) {
        1..=3 => RuleMode::Fixed,
        4..=6 => RuleMode::Mixed,
        _ => RuleMode::MixedDistracted,
    }
}

/// 连续答对的加分：第 1 题 10 分，之后每题 +4，封顶 46。
pub fn hit_score(streak: u32) -> u32 {
    10 + streak.saturating_sub(1).min(9) * 4
}
