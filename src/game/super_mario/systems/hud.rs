use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::resources::MarioStage;

pub fn mario_hud_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut hud_q: Query<(&mut Text2d, &MarioHud)>,
    player_q: Query<&MarioPlayer>,
) {
    if !session.paused
        && !session.finished
        && player_q
            .single()
            .map(|p| p.dead_timer <= 0.0 && !p.finished)
            .unwrap_or(false)
    {
        stage.time_left = (stage.time_left - time.delta_secs()).max(0.0);
    }
    for (mut t, hud) in &mut hud_q {
        match hud.kind {
            HudKind::Score => set_text(&mut t, &format!("{:06}", session.score)),
            HudKind::Coins => set_text(&mut t, &format!("x{:02}", stage.coins)),
            HudKind::Time => set_text(&mut t, &format!("{:03}", stage.time_left.ceil() as i32)),
            HudKind::World => set_text(&mut t, &format!("1-{}", stage.level)),
            HudKind::Lives => set_text(&mut t, &format!("x{}", session.lives.max(0))),
        }
    }
}

pub fn mario_time_check(
    mut session: ResMut<GameSession>,
    stage: Res<MarioStage>,
    mut player_q: Query<&mut MarioPlayer>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(mut player) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer <= 0.0 && !player.finished && stage.time_left <= 0.0 {
        player.dead_timer = 2.2;
        player.vel = Vec2::new(0.0, 380.0);
        session.lives -= 1;
    }
}
