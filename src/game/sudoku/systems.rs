use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::SudokuStage;
use super::setup::digit_label;

pub fn sudoku_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<SudokuStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Sudoku || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);
    stage.cursor.tick(dt);

    let (cols, rows) = (stage.layout.cols, stage.layout.rows);
    if stage.cursor.advance(&mut controls, cols, rows, false) {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    if controls.take_reset() {
        stage.board = stage.initial_board.clone();
        stage.refresh_conflicts();
        stage.message = "本关已重置".to_string();
        stage.message_clock = 1.2;
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }

    let step = i32::from(controls.take_primary()) - i32::from(controls.take_secondary());
    if step == 0 {
        return;
    }
    let index = stage.cursor_index();
    if stage.given[index] {
        stage.message = "这是题目给的数字".to_string();
        stage.message_clock = 0.9;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    }
    // 0 是空格，所以循环范围是 0..=size，动作一 +1、动作二 −1 都能经过空格。
    let modulus = stage.spec.size as i32 + 1;
    let next = (stage.board[index] as i32 + step).rem_euclid(modulus) as u8;
    stage.board[index] = next;
    let had_conflict = stage.conflicts[index];
    stage.refresh_conflicts();
    if stage.conflicts[index] {
        if !had_conflict {
            stage.mistakes += 1;
        }
        stage.message = "这里冲突了".to_string();
        stage.message_clock = 1.0;
        sfx.write(PlaySfx(SfxKind::Deny));
    } else if next == 0 {
        sfx.write(PlaySfx(SfxKind::Flip));
    } else {
        sfx.write(PlaySfx(SfxKind::Place));
    }
}

pub fn sudoku_render_sync(
    session: Res<GameSession>,
    stage: Res<SudokuStage>,
    mut fill_q: Query<(&SudokuCell, &mut Sprite), With<SudokuCellFill>>,
    mut text_q: Query<(&SudokuCell, &mut Text2d, &mut TextColor), With<SudokuCellText>>,
) {
    if session.kind != GameKind::Sudoku {
        return;
    }
    let cursor = stage.cursor_index();
    for (cell, mut sprite) in &mut fill_q {
        let color = if stage.conflicts[cell.index] {
            palette::CONFLICT_FILL
        } else if cell.index == cursor {
            palette::CURSOR_FILL
        } else if stage.given[cell.index] {
            palette::GIVEN_FILL
        } else {
            palette::CELL_FILL
        };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    for (cell, mut label, mut color) in &mut text_q {
        set_text(&mut label, &digit_label(stage.board[cell.index]));
        let next = if stage.conflicts[cell.index] {
            palette::CONFLICT_TEXT
        } else if stage.given[cell.index] {
            palette::GIVEN_TEXT
        } else {
            palette::FILLED_TEXT
        };
        if color.0 != next {
            color.0 = next;
        }
    }
}

pub fn sudoku_cursor_follow(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<SudokuStage>,
    mut cursor_q: Query<&mut Transform, With<SudokuCursor>>,
) {
    if session.kind != GameKind::Sudoku {
        return;
    }
    let Ok(mut transform) = cursor_q.single_mut() else {
        return;
    };
    let target = stage.layout.cell_center(stage.cursor.col, stage.cursor.row);
    let blend = (time.delta_secs() * CURSOR_BLEND).min(1.0);
    transform.translation.x += (target.x - transform.translation.x) * blend;
    transform.translation.y += (target.y - transform.translation.y) * blend;
}

pub fn sudoku_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SudokuStage>,
) {
    if session.kind != GameKind::Sudoku || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        let penalty = stage.mistakes * CONFLICT_PENALTY;
        session.score += (100 + time_bonus).saturating_sub(penalty);
        session.finished = true;
        session.won = true;
        session.status = format!(
            "{0}×{0} 完成 · 用时 {1:.0} 秒 · 冲突 {2}",
            stage.spec.size,
            stage.initial_time - stage.time_left,
            stage.mistakes,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::Sudoku.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        let empty = stage.spec.cells() - stage.filled();
        session.status = format!("时间到，还剩 {empty} 格没填");
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn sudoku_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SudokuStage>,
    mut hud: Query<&mut Text2d, (With<SudokuHud>, Without<SudokuScoreHud>, Without<SudokuMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<SudokuScoreHud>, Without<SudokuHud>, Without<SudokuMessage>)>,
    mut msg: Query<&mut Text2d, (With<SudokuMessage>, Without<SudokuHud>, Without<SudokuScoreHud>)>,
) {
    if session.kind != GameKind::Sudoku {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.filled(),
                stage.spec.cells(),
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::Sudoku.index()].max(session.score);
        set_text(&mut t, &format!("分数 {}  纪录 {}", session.score, high));
    }
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else if stage.has_conflict() {
            "还有冲突的格子"
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
