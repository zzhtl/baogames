use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::logic::{manhattan_lower_bound, slide};
use super::palette;
use super::resources::SlidingStage;

pub fn sliding_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<SlidingStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Sliding || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);

    if controls.take_reset() {
        stage.board = stage.initial_board.clone();
        stage.moves = 0;
        stage.history.clear();
        stage.recompute_lower_bound();
        stage.message = "本关已重置".to_string();
        stage.message_clock = 1.2;
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }

    if controls.take_secondary() {
        // 撤销 = 把上一步反着滑回去。
        if let Some((dx, dy)) = stage.history.pop() {
            let size = stage.size;
            slide(&mut stage.board, size, (-dx, -dy));
            stage.moves = stage.moves.saturating_sub(1);
            stage.message = "已撤销一步".to_string();
            stage.message_clock = 0.8;
            sfx.write(PlaySfx(SfxKind::Flip));
        } else {
            stage.message = "没有可撤销的".to_string();
            stage.message_clock = 0.8;
            sfx.write(PlaySfx(SfxKind::Deny));
        }
        return;
    }

    let Some(dir) = controls.take_dir() else {
        return;
    };
    let size = stage.size;
    if slide(&mut stage.board, size, dir).is_some() {
        stage.moves += 1;
        stage.history.push(dir);
        sfx.write(PlaySfx(SfxKind::Place));
    } else {
        stage.message = "这边滑不动".to_string();
        stage.message_clock = 0.8;
        sfx.write(PlaySfx(SfxKind::Deny));
    }
}

pub fn sliding_render_sync(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<SlidingStage>,
    mut tile_q: Query<(&SlideTile, &mut Transform), Without<SlideTileEdge>>,
    mut edge_q: Query<(&SlideTile, &mut Sprite), (With<SlideTileEdge>, Without<SlideTileFace>)>,
    mut face_q: Query<(&SlideTile, &mut Sprite), (With<SlideTileFace>, Without<SlideTileEdge>)>,
) {
    if session.kind != GameKind::Sliding {
        return;
    }
    let blend = (time.delta_secs() * TILE_BLEND).min(1.0);
    for (tile, mut transform) in &mut tile_q {
        let Some((col, row)) = stage.cell_of(tile.value) else {
            continue;
        };
        let target = stage.layout.cell_center(col, row);
        transform.translation.x += (target.x - transform.translation.x) * blend;
        transform.translation.y += (target.y - transform.translation.y) * blend;
    }
    let settled = |value: u8| {
        stage
            .board
            .get(value as usize - 1)
            .is_some_and(|&v| v == value)
    };
    for (tile, mut sprite) in &mut edge_q {
        let color = if settled(tile.value) { palette::TILE_DONE_EDGE } else { palette::TILE_EDGE };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    for (tile, mut sprite) in &mut face_q {
        let color = if settled(tile.value) { palette::TILE_DONE_FACE } else { palette::TILE_FACE };
        if sprite.color != color {
            sprite.color = color;
        }
    }
}

pub fn sliding_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SlidingStage>,
) {
    if session.kind != GameKind::Sliding || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        let bonus = efficiency_bonus(stage.moves, manhattan_lower_bound(&stage.initial_board, stage.size));
        session.score += 100 + time_bonus + bonus;
        session.finished = true;
        session.won = true;
        session.status = format!("{0}×{0} 复原 · {1} 步 · 效率 +{2}", stage.size, stage.moves, bonus);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::Sliding.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = format!("时间到，归位 {}/{}", stage.settled(), stage.tiles_total());
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn sliding_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SlidingStage>,
    mut hud: Query<&mut Text2d, (With<SlidingHud>, Without<SlidingScoreHud>, Without<SlidingMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<SlidingScoreHud>, Without<SlidingHud>, Without<SlidingMessage>)>,
    mut msg: Query<&mut Text2d, (With<SlidingMessage>, Without<SlidingHud>, Without<SlidingScoreHud>)>,
) {
    if session.kind != GameKind::Sliding {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.settled(),
                stage.tiles_total(),
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::Sliding.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  步数 {}", session.score, high, stage.moves),
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
