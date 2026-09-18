use bevy::prelude::*;
use rand::Rng;

use super::constants::INCONGRUENT_RATIO;

/// 本题要回答的是墨色还是字义。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StroopRule {
    /// 选「这个字是用什么颜色画的」。不识字也能玩。
    InkColor,
    /// 选「这个字读出来是什么颜色」。
    WordMeaning,
}

impl StroopRule {
    pub const fn prompt(self) -> &'static str {
        match self {
            StroopRule::InkColor => "看颜色",
            StroopRule::WordMeaning => "看字义",
        }
    }
}

/// 规则怎么变。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RuleMode {
    /// 整关只有「看颜色」。
    Fixed,
    /// 每题随机换规则。
    Mixed,
    /// 每题随机换规则，且规则提示本身也被涂成干扰色。
    MixedDistracted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Question {
    /// 字义的颜色编号。
    pub word: usize,
    /// 墨色的颜色编号。
    pub ink: usize,
    pub rule: StroopRule,
    /// 规则提示自己的颜色编号；`None` 表示用普通白字。
    pub prompt_ink: Option<usize>,
}

impl Question {
    pub fn answer(self) -> usize {
        match self.rule {
            StroopRule::InkColor => self.ink,
            StroopRule::WordMeaning => self.word,
        }
    }
}

#[derive(Resource)]
pub struct StroopStage {
    /// 本关启用的颜色编号（就是 `palette::INK` 的前 N 个下标）。
    pub colors: usize,
    pub mode: RuleMode,
    pub question: Question,
    pub cursor: usize,
    /// 已答对的题数。
    pub correct: u32,
    pub total: u32,
    pub asked: u32,
    pub streak: u32,
    pub best_streak: u32,
    pub mistakes: u32,
    pub question_time: f32,
    pub question_left: f32,
    pub time_left: f32,
    pub initial_time: f32,
    pub message: String,
    pub message_clock: f32,
    /// 本题已作答，等一小会儿再翻下一题，让反馈看得见。
    pub settle: f32,
}

impl StroopStage {
    pub fn is_cleared(&self) -> bool {
        self.correct >= self.total
    }
}

/// 出一道题。
///
/// `mode` 决定规则是否随机、提示是否带干扰色；[`INCONGRUENT_RATIO`] 控制
/// 字义与墨色不同的比例 —— 全都不同会让人只盯着墨色，反而没了干扰。
pub fn make_question<R: Rng + ?Sized>(rng: &mut R, colors: usize, mode: RuleMode) -> Question {
    let colors = colors.max(2);
    let word = rng.gen_range(0..colors);
    let ink = if rng.gen_bool(INCONGRUENT_RATIO) {
        // 在剩下的颜色里挑，保证与字义不同
        let offset = rng.gen_range(1..colors);
        (word + offset) % colors
    } else {
        word
    };
    let rule = match mode {
        RuleMode::Fixed => StroopRule::InkColor,
        _ => {
            if rng.gen_bool(0.5) {
                StroopRule::InkColor
            } else {
                StroopRule::WordMeaning
            }
        }
    };
    let prompt_ink = (mode == RuleMode::MixedDistracted).then(|| rng.gen_range(0..colors));
    Question {
        word,
        ink,
        rule,
        prompt_ink,
    }
}
