use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::resources::SpotDiffStage;

pub fn spotdiff_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<SpotDiffStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::SpotDiff || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);
    stage.cursor.tick(dt);

    let (cols, rows) = (stage.cols, stage.rows);
    if stage.cursor.advance(&mut controls, cols, rows, true) {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }
    controls.take_secondary();
    controls.take_reset();
    if !controls.take_primary() {
        return;
    }

    let index = stage.cursor_index();
    match stage.diff_slot(index) {
        Some(slot) if !stage.found[slot] => {
            stage.found[slot] = true;
            session.score += FOUND_SCORE;
            let left = stage.diffs.len() - stage.found_count();
            stage.message = if left == 0 {
                "全找到了！".to_string()
            } else {
                format!("还剩 {left} 个")
            };
            stage.message_clock = 1.2;
            sfx.write(PlaySfx(SfxKind::Match));
        }
        Some(_) => {
            stage.message = "这个已经找到了".to_string();
            stage.message_clock = 0.8;
            sfx.write(PlaySfx(SfxKind::Deny));
        }
        None => {
            stage.mistakes += 1;
            stage.time_left = (stage.time_left - WRONG_TIME_PENALTY).max(0.0);
            stage.message = format!("这里一样，-{WRONG_TIME_PENALTY:.0} 秒");
            stage.message_clock = 1.2;
            sfx.write(PlaySfx(SfxKind::Deny));
        }
    }
}

pub fn spotdiff_render_sync(
    session: Res<GameSession>,
    stage: Res<SpotDiffStage>,
    mut marks: Query<(&FoundMark, &mut Visibility)>,
) {
    if session.kind != GameKind::SpotDiff {
        return;
    }
    for (mark, mut visibility) in &mut marks {
        let target = if stage.is_found(mark.index) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != target {
            *visibility = target;
        }
    }
}

pub fn spotdiff_cursor_follow(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<SpotDiffStage>,
    mut cursor_q: Query<(&SpotCursor, &mut Transform)>,
) {
    if session.kind != GameKind::SpotDiff {
        return;
    }
    let blend = (time.delta_secs() * CURSOR_BLEND).min(1.0);
    for (cursor, mut transform) in &mut cursor_q {
        let Some(layout) = stage.layouts.get(cursor.side) else {
            continue;
        };
        let target = layout.cell_center(stage.cursor.col, stage.cursor.row);
        transform.translation.x += (target.x - transform.translation.x) * blend;
        transform.translation.y += (target.y - transform.translation.y) * blend;
    }
}

pub fn spotdiff_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SpotDiffStage>,
) {
    if session.kind != GameKind::SpotDiff || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        let penalty = stage.mistakes * WRONG_SCORE_PENALTY;
        session.score += (100 + time_bonus).saturating_sub(penalty);
        session.finished = true;
        session.won = true;
        session.status = format!(
            "找到 {} 处 · 标错 {} · 奖励 +{}",
            stage.diffs.len(),
            stage.mistakes,
            time_bonus,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::SpotDiff.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!(
            "时间到，还差 {} 处",
            stage.diffs.len() - stage.found_count()
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn spotdiff_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SpotDiffStage>,
    mut hud: Query<&mut Text2d, (With<SpotDiffHud>, Without<SpotDiffScoreHud>, Without<SpotDiffMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<SpotDiffScoreHud>, Without<SpotDiffHud>, Without<SpotDiffMessage>)>,
    mut msg: Query<&mut Text2d, (With<SpotDiffMessage>, Without<SpotDiffHud>, Without<SpotDiffScoreHud>)>,
) {
    if session.kind != GameKind::SpotDiff {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.found_count(),
                stage.diffs.len(),
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::SpotDiff.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  标错 {}", session.score, high, stage.mistakes),
        );
    }
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
