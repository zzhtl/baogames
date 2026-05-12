use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::player_size;
use super::super::resources::ContraStage;

pub fn contra_player_respawn(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    stage: Res<ContraStage>,
    mut player_q: Query<(&mut ContraPlayer, &mut Transform, &mut Sprite), Without<Camera>>,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraPlayer>)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut tr, mut sprite)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= dt;
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
            let cam_x = cam_q
                .single()
                .map(|t| t.translation.x)
                .unwrap_or(stage.player_spawn.x);
            let mut x = (cam_x - ARENA_W * 0.35).max(stage.player_spawn.x);
            if stage.boss_spawned && !stage.boss_dead {
                x = x.min(stage.boss_x - BOSS_W * 0.5 - PLAYER_W * 0.5 - 16.0);
            }
            tr.translation.x = x;
            tr.translation.y = ARENA_H * 0.5 - 40.0;
            player.vel = Vec2::ZERO;
            player.dead_timer = 0.0;
            player.invincible = INVINCIBLE_TIME;
            player.weapon = Weapon::M;
            player.fire_cd = 0.0;
            player.prone = false;
            sprite.custom_size = Some(player_size(false));
        }
    }
}
