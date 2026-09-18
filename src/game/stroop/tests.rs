use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::palette;
use super::resources::{RuleMode, StroopRule, make_question};
use super::setup::swatch_center;
use crate::common::px::WORLD_PER_PX;
use crate::common::settings::DisplayMode;

#[test]
fn palette_names_and_inks_line_up() {
    assert_eq!(palette::INK.len(), palette::NAMES.len());
    for (level, _) in LEVEL_COLORS.iter().enumerate() {
        assert!(LEVEL_COLORS[level] <= palette::INK.len());
    }
    assert!(JUNIOR_COLORS <= palette::INK.len());
}

#[test]
fn answer_follows_the_rule() {
    let mut rng = StdRng::seed_from_u64(7);
    for _ in 0..500 {
        let q = make_question(&mut rng, 6, RuleMode::Mixed);
        match q.rule {
            StroopRule::InkColor => assert_eq!(q.answer(), q.ink),
            StroopRule::WordMeaning => assert_eq!(q.answer(), q.word),
        }
        assert!(q.word < 6 && q.ink < 6);
    }
}

#[test]
fn fixed_mode_never_asks_for_word_meaning() {
    // 5-7 岁档不识字也要能玩，规则必须恒为「看颜色」，也不能带干扰色提示。
    let mut rng = StdRng::seed_from_u64(11);
    for _ in 0..200 {
        let q = make_question(&mut rng, JUNIOR_COLORS, RuleMode::Fixed);
        assert_eq!(q.rule, StroopRule::InkColor);
        assert_eq!(q.prompt_ink, None);
    }
}

#[test]
fn distracted_mode_always_paints_the_prompt() {
    let mut rng = StdRng::seed_from_u64(13);
    for _ in 0..200 {
        let q = make_question(&mut rng, 6, RuleMode::MixedDistracted);
        assert!(q.prompt_ink.is_some_and(|ink| ink < 6));
    }
}

/// 干扰题（字义≠墨色）要占多数，但不能是全部 —— 全是干扰题的话
/// 「无脑只看墨色」反而成了最优解，Stroop 效应就没了。
#[test]
fn incongruent_questions_are_the_majority_but_not_all() {
    let mut rng = StdRng::seed_from_u64(17);
    let total = 2000;
    let incongruent = (0..total)
        .filter(|_| {
            let q = make_question(&mut rng, 6, RuleMode::Mixed);
            q.word != q.ink
        })
        .count();
    let ratio = incongruent as f64 / total as f64;
    assert!((0.68..0.82).contains(&ratio), "干扰题比例 {ratio} 偏离预期");
}

#[test]
fn junior_tier_is_easier_on_every_level() {
    for level in 1..=10u8 {
        assert_eq!(colors_for(level, AgeTier::Junior), JUNIOR_COLORS);
        assert_eq!(rule_mode_for(level, AgeTier::Junior), RuleMode::Fixed);
        assert!(question_time_for(level, AgeTier::Junior) > question_time_for(level, AgeTier::Senior));
    }
    assert_eq!(rule_mode_for(4, AgeTier::Senior), RuleMode::Mixed);
    assert_eq!(rule_mode_for(7, AgeTier::Senior), RuleMode::MixedDistracted);
    assert_eq!(colors_for(10, AgeTier::Senior), 6);
}

/// 六个色块横排必须留在 4:3 画布内，且每块都落在整像素上。
#[test]
fn swatch_row_fits_the_narrow_canvas() {
    let half_visible = DisplayMode::Classic4x3.world_width() * 0.5;
    for colors in [JUNIOR_COLORS, palette::INK.len()] {
        for slot in 0..colors {
            let center = swatch_center(slot, colors);
            let edge = center.x.abs() + crate::common::px::px(SWATCH_W) * 0.5;
            assert!(edge <= half_visible - crate::common::px::px(4.0), "{colors} 色时色块出屏");
            let in_px = center.x / WORLD_PER_PX;
            assert_eq!(in_px, in_px.round(), "{colors} 色时第 {slot} 块落在半像素上");
        }
    }
}

#[test]
fn every_level_allows_finishing_within_the_budget() {
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let budget = level_time_for(level, age);
            let need = questions_for(level) as f32 * question_time_for(level, age);
            assert!(budget > need, "第 {level} 关 {age:?} 档时间不够答完");
        }
    }
}

#[test]
fn streak_bonus_grows_then_caps() {
    assert_eq!(hit_score(1), 10);
    assert_eq!(hit_score(2), 14);
    assert_eq!(hit_score(10), 46);
    assert_eq!(hit_score(50), 46);
}
