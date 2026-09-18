use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::{FONT_BODY, FONT_TITLE};
use crate::common::px::px;
use crate::common::render::{UiFont, rect, text};
use crate::common::theme::{ACCENT, BORDER, SURFACE, SURFACE_SEL, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{StroopStage, make_question};

const HUD_BORDER: Color = Color::srgb(0.90, 0.48, 0.82);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let colors = colors_for(level, age);
    let mode = rule_mode_for(level, age);
    let question = make_question(&mut rand::thread_rng(), colors, mode);

    // 题面底板：纯色汉字直接压在背景网格上会被网格线切断笔画。
    let board = Vec2::new(px(96.0), px(52.0));
    rect(commands, Vec2::new(0.0, px(WORD_Y)), board, SURFACE_SEL, GameEntity);
    rect(
        commands,
        Vec2::new(0.0, px(WORD_Y)),
        board - Vec2::splat(BORDER * 2.0),
        SURFACE,
        GameEntity,
    );

    text(
        commands, font, palette::NAMES[question.word],
        Vec2::new(0.0, px(WORD_Y)), FONT_TITLE, palette::INK[question.ink], GameEntity,
    )
    .insert(StroopWord);
    text(
        commands, font, question.rule.prompt(),
        Vec2::new(0.0, px(PROMPT_Y)), FONT_BODY, palette::PROMPT_PLAIN, GameEntity,
    )
    .insert(StroopPrompt);

    for slot in 0..colors {
        let center = swatch_center(slot, colors);
        rect(
            commands, center,
            Vec2::new(px(SWATCH_W), px(SWATCH_H)),
            palette::SWATCH_BORDER, GameEntity,
        );
        rect(
            commands, center,
            Vec2::new(px(SWATCH_W), px(SWATCH_H)) - Vec2::splat(BORDER * 2.0),
            palette::INK[slot], GameEntity,
        );
    }
    spawn_cursor(commands, swatch_center(0, colors));
    spawn_hud(commands, font, hud_root);

    let total_time = level_time_for(level, age);
    let question_time = question_time_for(level, age);
    commands.insert_resource(StroopStage {
        colors,
        mode,
        question,
        cursor: 0,
        correct: 0,
        total: questions_for(level),
        asked: 0,
        streak: 0,
        best_streak: 0,
        mistakes: 0,
        question_time,
        question_left: question_time,
        time_left: total_time,
        initial_time: total_time,
        message: "左右选色块作答".to_string(),
        message_clock: 2.4,
        settle: 0.0,
    });
}

/// 色块横排居中；`colors` 为偶数时半格偏移仍落在整像素上（步距 34 是偶数）。
pub fn swatch_center(slot: usize, colors: usize) -> Vec2 {
    let step = SWATCH_W + SWATCH_GAP;
    let offset = slot as f32 - (colors.max(1) - 1) as f32 * 0.5;
    Vec2::new(px(offset * step), px(SWATCH_Y))
}

fn spawn_cursor(commands: &mut Commands, center: Vec2) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(5.0)),
            Visibility::default(),
            StroopCursor,
            GameEntity,
        ))
        .id();
    let thickness = BORDER * 2.0;
    let (half_w, half_h) = (px(SWATCH_W) * 0.5 + thickness, px(SWATCH_H) * 0.5 + thickness);
    let (full_w, full_h) = (half_w * 2.0, half_h * 2.0);
    for (dx, dy, w, h) in [
        (0.0, half_h, full_w, thickness),
        (0.0, -half_h, full_w, thickness),
        (-half_w, 0.0, thickness, full_h),
        (half_w, 0.0, thickness, full_h),
    ] {
        commands.spawn((
            Sprite::from_color(palette::SWATCH_SELECTED, Vec2::new(w, h)),
            Transform::from_xyz(dx, dy, 0.0),
            GameEntity,
            ChildOf(parent),
        ));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "色字大挑战",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, StroopHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, StroopScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, StroopMessage,
    );
}
