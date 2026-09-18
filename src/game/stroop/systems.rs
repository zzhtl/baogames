use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{StroopStage, make_question};
use super::setup::swatch_center;

pub fn stroop_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<StroopStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Stroop || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);

    // 作答后的定格：让对错反馈看得见，再翻下一题。
    if stage.settle > 0.0 {
        stage.settle = (stage.settle - dt).max(0.0);
        if stage.settle == 0.0 && !stage.is_cleared() {
            next_question(&mut stage);
        }
        controls.clear();
        return;
    }

    if let Some((dx, _)) = controls.take_dir()
        && dx != 0
    {
        let colors = stage.colors;
        // 色块横排，左右回卷；上下键不参与，避免误触。
        stage.cursor = (stage.cursor as i32 + dx).rem_euclid(colors as i32) as usize;
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    stage.question_left = (stage.question_left - dt).max(0.0);
    if stage.question_left == 0.0 {
        register_miss(&mut stage, "超时了！", &mut sfx);
        return;
    }
    if !controls.take_primary() {
        return;
    }
    if stage.cursor == stage.question.answer() {
        stage.asked += 1;
        stage.correct += 1;
        stage.streak += 1;
        stage.best_streak = stage.best_streak.max(stage.streak);
        session.score += hit_score(stage.streak);
        stage.message = "答对了！".to_string();
        stage.message_clock = 0.8;
        stage.settle = 0.22;
        sfx.write(PlaySfx(SfxKind::Match));
    } else {
        register_miss(&mut stage, "答错了", &mut sfx);
    }
}

fn register_miss(stage: &mut StroopStage, reason: &str, sfx: &mut MessageWriter<PlaySfx>) {
    stage.asked += 1;
    stage.streak = 0;
    stage.mistakes += 1;
    stage.time_left = (stage.time_left - WRONG_TIME_PENALTY).max(0.0);
    stage.message = format!("{reason} -{WRONG_TIME_PENALTY:.0} 秒");
    stage.message_clock = 1.0;
    stage.settle = 0.22;
    sfx.write(PlaySfx(SfxKind::Deny));
}

fn next_question(stage: &mut StroopStage) {
    stage.question = make_question(&mut rand::thread_rng(), stage.colors, stage.mode);
    stage.question_left = stage.question_time;
}

pub fn stroop_render_sync(
    session: Res<GameSession>,
    stage: Res<StroopStage>,
    mut word_q: Query<(&mut Text2d, &mut TextColor), (With<StroopWord>, Without<StroopPrompt>)>,
    mut prompt_q: Query<(&mut Text2d, &mut TextColor), (With<StroopPrompt>, Without<StroopWord>)>,
    mut cursor_q: Query<&mut Transform, With<StroopCursor>>,
) {
    if session.kind != GameKind::Stroop {
        return;
    }
    if let Ok((mut label, mut color)) = word_q.single_mut() {
        set_text(&mut label, palette::NAMES[stage.question.word]);
        let next = palette::INK[stage.question.ink];
        if color.0 != next {
            color.0 = next;
        }
    }
    if let Ok((mut label, mut color)) = prompt_q.single_mut() {
        set_text(&mut label, stage.question.rule.prompt());
        let next = stage
            .question
            .prompt_ink
            .map(|ink| palette::INK[ink])
            .unwrap_or(palette::PROMPT_PLAIN);
        if color.0 != next {
            color.0 = next;
        }
    }
    if let Ok(mut transform) = cursor_q.single_mut() {
        let target = swatch_center(stage.cursor, stage.colors);
        transform.translation.x = target.x;
        transform.translation.y = target.y;
    }
}

pub fn stroop_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<StroopStage>,
) {
    if session.kind != GameKind::Stroop || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        session.score += 100 + time_bonus;
        session.finished = true;
        session.won = true;
        session.status = format!(
            "用时 {:.0} 秒 · 连对 {} · 错 {}",
            stage.initial_time - stage.time_left,
            stage.best_streak,
            stage.mistakes,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::Stroop.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!("时间到，还差 {} 题", stage.total - stage.correct);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn stroop_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<StroopStage>,
    mut hud: Query<&mut Text2d, (With<StroopHud>, Without<StroopScoreHud>, Without<StroopMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<StroopScoreHud>, Without<StroopHud>, Without<StroopMessage>)>,
    mut msg: Query<&mut Text2d, (With<StroopMessage>, Without<StroopHud>, Without<StroopScoreHud>)>,
) {
    if session.kind != GameKind::Stroop {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.correct,
                stage.total,
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::Stroop.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  连对 {}", session.score, high, stage.streak),
        );
    }
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.clone()
        } else {
            format!("本题 {:.1} 秒", stage.question_left.max(0.0))
        };
        set_text(&mut t, &value);
    }
}
