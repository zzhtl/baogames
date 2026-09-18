use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::SchulteStage;
use super::setup::number_color;

pub fn schulte_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<SchulteStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Schulte || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);
    stage.wrong_clock = (stage.wrong_clock - dt).max(0.0);
    if stage.wrong_clock == 0.0 {
        stage.wrong_cell = None;
    }
    stage.cursor.tick(dt);

    let (cols, rows) = (stage.layout.cols, stage.layout.rows);
    // 舒尔特是纯视觉搜索，光标绕边界回卷比撞墙停住更省按键。
    if stage.cursor.advance(&mut controls, cols, rows, true) {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    // 动作二和重置键都用来重排题面：点到卡壳时换一张，比干瞪眼强。
    if controls.take_reset() || controls.take_secondary() {
        reshuffle(&mut stage);
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }

    if !controls.take_primary() {
        return;
    }
    let (col, row) = (stage.cursor.col, stage.cursor.row);
    let Some((index, cell)) = stage.cell_at(col, row) else {
        return;
    };
    if stage.found[index] {
        stage.message = "这格已经点过了".to_string();
        stage.message_clock = 0.9;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    }
    if Some(cell) == stage.target() {
        stage.found[index] = true;
        stage.progress += 1;
        stage.streak += 1;
        stage.best_streak = stage.best_streak.max(stage.streak);
        session.score += hit_score(stage.streak);
        stage.message.clear();
        stage.message_clock = 0.0;
        sfx.write(PlaySfx(SfxKind::Match));
    } else {
        stage.streak = 0;
        stage.mistakes += 1;
        stage.time_left = (stage.time_left - MISTAKE_TIME_PENALTY).max(0.0);
        stage.wrong_cell = Some(index);
        stage.wrong_clock = 0.35;
        stage.message = format!("点错了，-{MISTAKE_TIME_PENALTY:.0} 秒");
        stage.message_clock = 1.1;
        sfx.write(PlaySfx(SfxKind::Deny));
    }
}

/// 重排还没点掉的格子，已点掉的位置保持不动。
fn reshuffle(stage: &mut SchulteStage) {
    use rand::seq::SliceRandom;
    let mut open: Vec<(u32, u8)> = stage
        .cells
        .iter()
        .zip(stage.found.iter())
        .filter_map(|(cell, found)| (!found).then_some(*cell))
        .collect();
    open.shuffle(&mut rand::thread_rng());
    let mut next = open.into_iter();
    for (index, cell) in stage.cells.iter_mut().enumerate() {
        if !stage.found[index]
            && let Some(value) = next.next()
        {
            *cell = value;
        }
    }
    stage.streak = 0;
    stage.message = "已重新排列".to_string();
    stage.message_clock = 1.0;
}

/// 格子底板 / 数字按「已点掉 · 点错闪烁 · 正常」三态换色。
pub fn schulte_render_sync(
    session: Res<GameSession>,
    stage: Res<SchulteStage>,
    mut fill_q: Query<(&SchulteCell, &mut Sprite), (With<SchulteCellFill>, Without<SchulteCellBorder>)>,
    mut border_q: Query<(&SchulteCell, &mut Sprite), (With<SchulteCellBorder>, Without<SchulteCellFill>)>,
    mut text_q: Query<(&SchulteCell, &mut TextColor, &mut Text2d), With<SchulteCellText>>,
) {
    if session.kind != GameKind::Schulte {
        return;
    }
    let set_color = |sprite: &mut Sprite, color: Color| {
        if sprite.color != color {
            sprite.color = color;
        }
    };
    for (cell, mut sprite) in &mut fill_q {
        let color = if stage.wrong_cell == Some(cell.index) {
            palette::WRONG_FLASH
        } else if stage.found[cell.index] {
            palette::DONE_FILL
        } else {
            palette::CELL_FILL
        };
        set_color(&mut sprite, color);
    }
    for (cell, mut sprite) in &mut border_q {
        let color = if stage.found[cell.index] {
            palette::DONE_BORDER
        } else {
            palette::CELL_BORDER
        };
        set_color(&mut sprite, color);
    }
    for (cell, mut color, mut label) in &mut text_q {
        let (value, color_id) = stage.cells[cell.index];
        set_text(&mut label, &value.to_string());
        let next = if stage.found[cell.index] {
            palette::DONE_TEXT
        } else {
            number_color(color_id, stage.mode)
        };
        if color.0 != next {
            color.0 = next;
        }
    }
}

pub fn schulte_cursor_follow(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<SchulteStage>,
    mut cursor_q: Query<&mut Transform, With<SchulteCursor>>,
) {
    if session.kind != GameKind::Schulte {
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

pub fn schulte_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SchulteStage>,
) {
    if session.kind != GameKind::Schulte || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        session.score += 100 + time_bonus;
        session.finished = true;
        session.won = true;
        // 结算面板只有 204 画布像素宽，文案按 12px 字宽估到 170 以内。
        session.status = format!(
            "用时 {:.0} 秒 · 连对 {} · 错 {}",
            stage.initial_time - stage.time_left,
            stage.best_streak,
            stage.mistakes,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::Schulte.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!("时间到，还差 {} 个", stage.order.len() - stage.progress);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn schulte_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SchulteStage>,
    mut hud: Query<&mut Text2d, (With<SchulteHud>, Without<SchulteScoreHud>, Without<SchulteTarget>)>,
    mut score_hud: Query<&mut Text2d, (With<SchulteScoreHud>, Without<SchulteHud>, Without<SchulteTarget>)>,
    mut target_hud: Query<(&mut Text2d, &mut TextColor), (With<SchulteTarget>, Without<SchulteHud>, Without<SchulteScoreHud>)>,
) {
    if session.kind != GameKind::Schulte {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.progress,
                stage.order.len(),
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::Schulte.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  连对 {}", session.score, high, stage.streak),
        );
    }
    if let Ok((mut t, mut color)) = target_hud.single_mut() {
        let (value, next_color) = match stage.target() {
            Some((value, color_id)) if stage.message_clock <= 0.0 => (
                format!("下一个 {value}"),
                number_color(color_id, stage.mode),
            ),
            _ => (stage.message.clone(), palette::CURSOR),
        };
        set_text(&mut t, &value);
        if color.0 != next_color {
            color.0 = next_color;
        }
    }
}
