use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::MazeStage;

pub fn maze_input(
    mut controls: ResMut<PuzzleControls>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<MazeStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::MazeRun || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.message_clock = (stage.message_clock - dt).max(0.0);
    stage.move_cd = (stage.move_cd - dt).max(0.0);

    if controls.take_reset() {
        stage.player = stage.start;
        stage.message = "回到起点".to_string();
        stage.message_clock = 1.0;
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }
    controls.take_primary();
    controls.take_secondary();

    let buffered = controls.take_dir();
    let immediate = buffered.is_some();
    let Some(dir) = buffered.or_else(|| controls.held_dir()) else {
        return;
    };
    if !immediate && stage.move_cd > 0.0 {
        return;
    }

    let (nc, nr) = (stage.player.0 + dir.0, stage.player.1 + dir.1);
    if stage.is_wall(nc, nr) {
        // 撞墙时缩短冷却，长按贴着墙拐弯才跟手。
        stage.move_cd = MOVE_COOLDOWN * 0.5;
        if immediate {
            sfx.write(PlaySfx(SfxKind::Deny));
        }
        return;
    }
    stage.player = (nc, nr);
    stage.steps += 1;
    stage.move_cd = MOVE_COOLDOWN;
    if let Some(index) = stage.index(nc, nr) {
        if stage.visited[index] {
            stage.backtracks += 1;
        } else {
            stage.visited[index] = true;
        }
    }

    if stage.key == Some((nc, nr)) && !stage.has_key {
        stage.has_key = true;
        stage.message = "拿到钥匙了！".to_string();
        stage.message_clock = 1.6;
        sfx.write(PlaySfx(SfxKind::Coin));
        return;
    }
    if (nc, nr) == stage.exit && !stage.exit_open() {
        stage.message = "先找到钥匙".to_string();
        stage.message_clock = 1.4;
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    }
    sfx.write(PlaySfx(SfxKind::MenuMove));
}

pub fn maze_render_sync(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<MazeStage>,
    mut player_q: Query<&mut Transform, With<MazePlayer>>,
    mut floor_q: Query<(&MazeTile, &mut Sprite), (With<MazeFloor>, Without<MazeExit>, Without<MazeKey>)>,
    mut exit_q: Query<&mut Sprite, (With<MazeExit>, Without<MazeFloor>, Without<MazeKey>)>,
    mut key_q: Query<&mut Visibility, (With<MazeKey>, Without<MazePlayer>)>,
) {
    if session.kind != GameKind::MazeRun {
        return;
    }
    if let Ok(mut transform) = player_q.single_mut() {
        let target = stage.layout.cell_center(stage.player.0, stage.player.1);
        let blend = (time.delta_secs() * PLAYER_BLEND).min(1.0);
        transform.translation.x += (target.x - transform.translation.x) * blend;
        transform.translation.y += (target.y - transform.translation.y) * blend;
    }
    for (tile, mut sprite) in &mut floor_q {
        let walked = stage
            .index(tile.col, tile.row)
            .is_some_and(|index| stage.visited[index]);
        let color = if walked { palette::FLOOR_TRAIL } else { palette::FLOOR };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    if let Ok(mut sprite) = exit_q.single_mut() {
        let color = if stage.exit_open() { palette::EXIT_OPEN } else { palette::EXIT_LOCKED };
        if sprite.color != color {
            sprite.color = color;
        }
    }
    if let Ok(mut visibility) = key_q.single_mut() {
        let target = if stage.has_key { Visibility::Hidden } else { Visibility::Inherited };
        if *visibility != target {
            *visibility = target;
        }
    }
}

/// 雾视野：只显示玩家周围一圈瓦片。没开雾时这套判断恒为真，等于不做事。
pub fn maze_fog_sync(
    session: Res<GameSession>,
    stage: Res<MazeStage>,
    mut tiles: Query<(&MazeTile, &mut Visibility), Without<MazePlayer>>,
) {
    if session.kind != GameKind::MazeRun || stage.fog_radius.is_none() {
        return;
    }
    for (tile, mut visibility) in &mut tiles {
        let target = if stage.is_visible(tile.col, tile.row) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != target {
            *visibility = target;
        }
    }
}

pub fn maze_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<MazeStage>,
) {
    if session.kind != GameKind::MazeRun || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.is_cleared() {
        let time_bonus = (stage.time_left * 2.0) as u32;
        let penalty = stage.backtracks * BACKTRACK_PENALTY;
        session.score += (100 + time_bonus).saturating_sub(penalty);
        session.finished = true;
        session.won = true;
        session.status = format!(
            "走了 {} 步 · 回头 {} · 奖励 +{}",
            stage.steps, stage.backtracks, time_bonus,
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(GameKind::MazeRun.max_level());
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = if stage.exit_open() {
            "时间到，没能走到出口".to_string()
        } else {
            "时间到，钥匙还没找到".to_string()
        };
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn maze_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<MazeStage>,
    mut hud: Query<&mut Text2d, (With<MazeHud>, Without<MazeScoreHud>, Without<MazeMessage>)>,
    mut score_hud: Query<&mut Text2d, (With<MazeScoreHud>, Without<MazeHud>, Without<MazeMessage>)>,
    mut msg: Query<&mut Text2d, (With<MazeMessage>, Without<MazeHud>, Without<MazeScoreHud>)>,
) {
    if session.kind != GameKind::MazeRun {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let key_state = match (stage.key.is_some(), stage.has_key) {
            (false, _) => "",
            (true, true) => "  钥匙 ○",
            (true, false) => "  钥匙 ×",
        };
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}{}",
                session.level,
                stage.time_left.max(0.0),
                key_state,
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        let high = save.high_scores[GameKind::MazeRun.index()].max(session.score);
        set_text(
            &mut t,
            &format!("分数 {}  纪录 {}  步数 {}", session.score, high, stage.steps),
        );
    }
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else if stage.fog_radius.is_some() {
            "雾很浓，小心"
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
