use bevy::prelude::*;

use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::MarioStage;

pub fn mario_flag_check(
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform)>,
    flag_q: Query<&Transform, (With<FlagPole>, Without<MarioPlayer>)>,
    mut banner_q: Query<&mut FlagBanner>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.finished || player.dead_timer > 0.0 {
        return;
    }
    let Ok(ftr) = flag_q.single() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    if (p_pos.x - ftr.translation.x).abs() < 16.0 && p_pos.y > FLOOR_Y {
        player.finished = true;
        let h_ratio = ((p_pos.y - FLOOR_Y) / (10.0 * TILE)).clamp(0.0, 1.0);
        let pts = (h_ratio * 5000.0) as u32 + 100;
        session.score = session.score.saturating_add(pts);
        for mut b in &mut banner_q {
            b.y_target = FLOOR_Y + TILE + 8.0;
            b.speed = 90.0;
        }
    }
}

pub fn mario_flag_anim(time: Res<Time>, mut q: Query<(&mut Transform, &FlagBanner)>) {
    let dt = time.delta_secs();
    for (mut tr, b) in &mut q {
        if tr.translation.y > b.y_target {
            tr.translation.y = (tr.translation.y - b.speed * dt).max(b.y_target);
        }
    }
}

pub fn mario_finish_seq(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<&MarioPlayer>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(player) = player_q.single() else {
        return;
    };
    if !player.finished {
        return;
    }
    stage.finish_timer += time.delta_secs();
    if stage.time_left > 0.0 {
        let take = (200.0 * time.delta_secs()).min(stage.time_left);
        stage.time_left -= take;
        session.score = session.score.saturating_add(take.ceil() as u32 * 50);
    }
    if stage.finish_timer > 3.5 {
        session.finished = true;
        session.won = true;
        session.status = "🏰 通关！按 Enter 再来一次，Esc / Backspace 回菜单".to_string();
        let idx = session.kind.index();
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level as u8 + 1).min(4);
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
    }
}
