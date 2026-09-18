use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{SimonPhase, SimonStage};

/// 音板 0..4 对应上 / 下 / 左 / 右，和 `PuzzleControls` 的方向缓冲一致。
pub(super) fn pad_of(dir: (i32, i32)) -> Option<usize> {
    match dir {
        (0, -1) => Some(0),
        (0, 1) => Some(1),
        (-1, 0) => Some(2),
        (1, 0) => Some(3),
        _ => None,
    }
}

fn tone(pad: usize) -> SfxKind {
    match pad {
        0 => SfxKind::Tone0,
        1 => SfxKind::Tone1,
        2 => SfxKind::Tone2,
        _ => SfxKind::Tone3,
    }
}

pub fn simon_update(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<SimonStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Simon || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.clock = (stage.clock - dt).max(0.0);
    stage.lit_clock = (stage.lit_clock - dt).max(0.0);
    if stage.lit_clock == 0.0 && stage.phase != SimonPhase::Playback {
        stage.lit = None;
    }

    match stage.phase {
        SimonPhase::Ready => {
            controls.clear();
            if stage.clock == 0.0 {
                stage.phase = SimonPhase::Playback;
                stage.play_index = 0;
                stage.clock = 0.0;
                stage.message = "看好顺序".to_string();
            }
        }
        SimonPhase::Playback => {
            controls.clear();
            if stage.clock > 0.0 {
                return;
            }
            if stage.lit.is_some() {
                // 刚放完一个音，插一段间隔再放下一个
                stage.lit = None;
                stage.clock = stage.gap_time();
                stage.play_index += 1;
                return;
            }
            if stage.play_index >= stage.sequence.len() {
                stage.phase = SimonPhase::Input;
                stage.input_index = 0;
                stage.clock = stage.input_time;
                stage.message = if stage.reverse {
                    "倒着按回去！".to_string()
                } else {
                    "该你按了".to_string()
                };
                return;
            }
            let pad = stage.sequence[stage.play_index] as usize;
            stage.lit = Some(pad);
            stage.lit_clock = stage.step_time;
            stage.clock = stage.step_time;
            sfx.write(PlaySfx(tone(pad)));
        }
        SimonPhase::Input => {
            if stage.clock == 0.0 {
                fail_round(&mut stage, &mut session, "太慢了", &mut sfx);
                return;
            }
            let Some(dir) = controls.take_dir() else {
                return;
            };
            let Some(pad) = pad_of(dir) else {
                return;
            };
            stage.lit = Some(pad);
            stage.lit_clock = PRESS_LIGHT_TIME;
            sfx.write(PlaySfx(tone(pad)));
            if stage.expected() != Some(pad as u8) {
                fail_round(&mut stage, &mut session, "按错了", &mut sfx);
                return;
            }
            stage.input_index += 1;
            if stage.input_index < stage.sequence.len() {
                stage.clock = stage.input_time;
                return;
            }
            // 一轮答对
            session.score += round_score(stage.sequence.len());
            stage.round += 1;
            stage.phase = SimonPhase::Feedback;
            stage.clock = FEEDBACK_TIME;
            stage.message = "答对了！".to_string();
            sfx.write(PlaySfx(SfxKind::Match));
        }
        SimonPhase::Feedback => {
            controls.clear();
            if stage.clock > 0.0 {
                return;
            }
            if stage.is_cleared() || stage.failed {
                return; // 交给 check_finish 收尾
            }
            stage.begin_next_round(palette::PAD_DIM.len(), &mut rand::thread_rng());
        }
    }
}

/// 按错或超时：扣一条命，同一段序列重放一次。
fn fail_round(
    stage: &mut SimonStage,
    session: &mut GameSession,
    reason: &str,
    sfx: &mut MessageWriter<PlaySfx>,
) {
    session.lives -= 1;
    stage.phase = SimonPhase::Feedback;
    stage.clock = FEEDBACK_TIME;
    stage.message = format!("{reason}，重来");
    stage.retry = true;
    stage.failed = session.lives <= 0;
    sfx.write(PlaySfx(SfxKind::Deny));
}

pub fn simon_render_sync(
    session: Res<GameSession>,
    stage: Res<SimonStage>,
    mut pad_q: Query<(&SimonPad, &mut Sprite), Without<SimonCore>>,
    mut core_q: Query<&mut Sprite, (With<SimonCore>, Without<SimonPad>)>,
) {
    if session.kind != GameKind::Simon {
        return;
    }
    for (pad, mut sprite) in &mut pad_q {
        let color = if stage.lit == Some(pad.pad) {
            palette::PAD_LIT[pad.pad]
        } else {
            palette::PAD_DIM[pad.pad]
        };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    if let Ok(mut sprite) = core_q.single_mut() {
        let color = if stage.phase == SimonPhase::Input {
            palette::PAD_LIT[3]
        } else {
            palette::CORE
        };
        if sprite.color != color {
            sprite.color = color;
        }
    }
}

pub fn simon_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SimonStage>,
) {
    if session.kind != GameKind::Simon || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        session.score += 100 + session.lives.max(0) as u32 * 40;
        session.finished = true;
        session.won = true;
        session.status = format!(
            "{} 轮全对 · 最长 {} · 剩 {} 命",
            stage.rounds_total,
            stage.sequence.len(),
            session.lives.max(0),
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::Simon.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.failed && stage.phase == SimonPhase::Feedback && stage.clock <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!("命没了，撑到第 {} 轮", stage.round);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn simon_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SimonStage>,
    mut hud: Query<&mut Text2d, (With<SimonHud>, Without<SimonScoreHud>, Without<SimonMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<SimonScoreHud>, Without<SimonHud>, Without<SimonMessage>)>,
    mut msg: Query<&mut Text2d, (With<SimonMessage>, Without<SimonHud>, Without<SimonScoreHud>)>,
) {
    if session.kind != GameKind::Simon {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  第 {}/{} 轮  命 {}",
                session.level,
                stage.round.min(stage.rounds_total),
                stage.rounds_total,
                session.lives.max(0),
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::Simon.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  长度 {}", session.score, high, stage.sequence.len()),
        );
    }
    if let Ok(mut t) = msg.single_mut() {
        let value = if session.finished { "" } else { stage.message.as_str() };
        set_text(&mut t, value);
    }
}
