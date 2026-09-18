use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::px::snap2;
use crate::common::render::set_text;
use crate::common::theme::BORDER;
use crate::game::model::{GameEntity, GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::link::{apply_gravity, find_path, has_any_move, reshuffle};
use super::palette;
use super::resources::LinkupStage;
use super::setup::{pattern_char, pattern_color};

pub fn linkup_input(
    mut commands: Commands,
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<LinkupStage>,
    mut sfx: MessageWriter<PlaySfx>,
    paths: Query<Entity, With<LinkPathSegment>>,
) {
    if session.kind != GameKind::LinkUp || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);
    if stage.path_clock > 0.0 {
        stage.path_clock = (stage.path_clock - dt).max(0.0);
        if stage.path_clock == 0.0 {
            for entity in &paths {
                commands.entity(entity).try_despawn();
            }
            stage.path.clear();
        }
    }
    stage.cursor.tick(dt);

    let (cols, rows) = (stage.cols, stage.rows);
    if stage.cursor.advance(&mut controls, cols, rows, true) {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    if controls.take_reset() {
        shuffle_board(&mut stage, &mut sfx, "已重新洗牌");
        return;
    }
    if controls.take_secondary() {
        if stage.selected.take().is_some() {
            sfx.write(PlaySfx(SfxKind::Flip));
        }
        return;
    }
    if !controls.take_primary() {
        return;
    }

    let here = (stage.cursor.col, stage.cursor.row);
    let Some(pattern) = stage.tile_at(here.0, here.1) else {
        stage.message = "这里是空格".to_string();
        stage.message_clock = 0.8;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    };
    let Some(first) = stage.selected else {
        stage.selected = Some(here);
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    };
    if first == here {
        stage.selected = None;
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }
    if stage.tile_at(first.0, first.1) != Some(pattern) {
        // 图案不同：把选中换成这一张，比强迫玩家先取消更顺手。
        stage.selected = Some(here);
        stage.streak = 0;
        stage.message = "图案不一样".to_string();
        stage.message_clock = 0.8;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    }

    let Some(path) = find_path(&stage.tiles, cols, rows, first, here) else {
        stage.streak = 0;
        stage.message = "连不过去".to_string();
        stage.message_clock = 1.0;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    };

    for cell in [first, here] {
        if let Some(index) = stage.index(cell.0, cell.1) {
            stage.tiles[index] = None;
        }
    }
    stage.selected = None;
    stage.pairs_left = stage.pairs_left.saturating_sub(1);
    stage.streak += 1;
    stage.best_streak = stage.best_streak.max(stage.streak);
    session.score += hit_score(stage.streak);
    sfx.write(PlaySfx(SfxKind::Match));

    if stage.gravity {
        apply_gravity(&mut stage.tiles, cols, rows);
    }
    spawn_path(&mut commands, &stage, &path);
    stage.path = path;
    stage.path_clock = PATH_SHOW_TIME;

    // 走进死局就自动洗牌：小朋友分不清"没得消"和"我没找到"。
    if !stage.is_cleared() && !has_any_move(&stage.tiles, cols, rows) {
        shuffle_board(&mut stage, &mut sfx, "没得消，自动洗牌");
    }
}

fn shuffle_board(stage: &mut LinkupStage, sfx: &mut MessageWriter<PlaySfx>, message: &str) {
    let mut rng = rand::thread_rng();
    let (cols, rows) = (stage.cols, stage.rows);
    // 洗到有解为止；棋盘很小，最多几次就成。
    for _ in 0..64 {
        reshuffle(&mut stage.tiles, &mut rng);
        if stage.is_cleared() || has_any_move(&stage.tiles, cols, rows) {
            break;
        }
    }
    stage.selected = None;
    stage.shuffles += 1;
    stage.streak = 0;
    stage.message = message.to_string();
    stage.message_clock = 1.4;
    sfx.write(PlaySfx(SfxKind::Flip));
}

/// 把折点序列连成几段线画出来。
fn spawn_path(commands: &mut Commands, stage: &LinkupStage, path: &[(i32, i32)]) {
    let thickness = BORDER;
    for pair in path.windows(2) {
        let a = snap2(stage.layout.cell_center(pair[0].0, pair[0].1));
        let b = snap2(stage.layout.cell_center(pair[1].0, pair[1].1));
        let center = (a + b) * 0.5;
        let size = Vec2::new(
            (b.x - a.x).abs().max(thickness),
            (b.y - a.y).abs().max(thickness),
        );
        commands.spawn((
            Sprite::from_color(palette::PATH, size),
            Transform::from_translation(center.extend(4.0)),
            LinkPathSegment,
            GameEntity,
        ));
    }
}

pub fn linkup_render_sync(
    session: Res<GameSession>,
    stage: Res<LinkupStage>,
    mut edge_q: Query<(&LinkTile, &mut Sprite), (With<LinkTileEdge>, Without<LinkTileFace>)>,
    mut face_q: Query<(&LinkTile, &mut Visibility), (With<LinkTileFace>, Without<LinkTileEdge>)>,
    mut edge_vis_q: Query<(&LinkTile, &mut Visibility), (With<LinkTileEdge>, Without<LinkTileFace>)>,
    mut text_q: Query<(&LinkTile, &mut Text2d, &mut TextColor), With<LinkTileText>>,
) {
    if session.kind != GameKind::LinkUp {
        return;
    }
    let selected_index = stage
        .selected
        .and_then(|(col, row)| stage.index(col, row));
    for (tile, mut sprite) in &mut edge_q {
        let color = if Some(tile.index) == selected_index {
            palette::TILE_SELECTED
        } else {
            palette::TILE_EDGE
        };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    let apply = |visibility: &mut Visibility, on: bool| {
        let target = if on { Visibility::Inherited } else { Visibility::Hidden };
        if *visibility != target {
            *visibility = target;
        }
    };
    for (tile, mut visibility) in &mut face_q {
        apply(&mut visibility, stage.tiles[tile.index].is_some());
    }
    for (tile, mut visibility) in &mut edge_vis_q {
        apply(&mut visibility, stage.tiles[tile.index].is_some());
    }
    for (tile, mut label, mut color) in &mut text_q {
        let pattern = stage.tiles[tile.index];
        set_text(&mut label, &pattern_char(pattern));
        let next = pattern_color(pattern);
        if color.0 != next {
            color.0 = next;
        }
    }
}

pub fn linkup_cursor_follow(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<LinkupStage>,
    mut cursor_q: Query<&mut Transform, With<LinkCursor>>,
) {
    if session.kind != GameKind::LinkUp {
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

pub fn linkup_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<LinkupStage>,
) {
    if session.kind != GameKind::LinkUp || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        session.score += 100 + time_bonus;
        session.finished = true;
        session.won = true;
        session.status = format!(
            "全部消完 · 连对 {} · 洗牌 {}",
            stage.best_streak, stage.shuffles,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::LinkUp.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!("时间到，还剩 {} 对", stage.pairs_left);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn linkup_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<LinkupStage>,
    mut hud: Query<&mut Text2d, (With<LinkupHud>, Without<LinkupScoreHud>, Without<LinkupMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<LinkupScoreHud>, Without<LinkupHud>, Without<LinkupMessage>)>,
    mut msg: Query<&mut Text2d, (With<LinkupMessage>, Without<LinkupHud>, Without<LinkupScoreHud>)>,
) {
    if session.kind != GameKind::LinkUp {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  剩 {} 对",
                session.level,
                stage.time_left.max(0.0),
                stage.pairs_left,
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::LinkUp.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  连对 {}", session.score, high, stage.streak),
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
