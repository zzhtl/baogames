use bevy::prelude::*;

use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::LEVEL_TIME;
use super::super::resources::MarioStage;
use super::super::setup_actors::build_player_visual;

pub fn mario_respawn(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &mut Transform)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, mut tr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= time.delta_secs();
        if player.dead_timer <= 0.0 {
            if session.lives <= 0 {
                session.finished = true;
                session.won = false;
                session.status = "GAME OVER  Enter 重玩 / Backspace 返回菜单".to_string();
                let idx = session.kind.index();
                if session.score > save.high_scores[idx] {
                    save.high_scores[idx] = session.score;
                }
                save.store();
                return;
            }
            tr.translation.x = stage.player_spawn.x;
            tr.translation.y = stage.player_spawn.y;
            player.vel = Vec2::ZERO;
            player.on_ground = false;
            player.invincible = 1.6;
            player.dead_timer = 0.0;
            player.state = PowerState::Small;
            player.transform_t = 0.0;
            player.fire_cd = 0.0;
            commands.entity(player_e).despawn_related::<Children>();
            build_player_visual(&mut commands, player_e, PowerState::Small);
            stage.time_left = LEVEL_TIME;
        }
    }
}
