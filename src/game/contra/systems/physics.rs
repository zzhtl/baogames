use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{aabb_overlap, player_size};
use super::super::resources::ContraStage;

pub fn contra_physics(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    stage: Res<ContraStage>,
    mut player_q: Query<(&mut ContraPlayer, &mut Transform), Without<ContraSolid>>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraPlayer>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((mut player, mut tr)) = player_q.single_mut() else {
        return;
    };
    if player.invincible > 0.0 {
        player.invincible = (player.invincible - dt).max(0.0);
    }
    player.visual_t += dt;
    player.landing_t = (player.landing_t - dt).max(0.0);

    if player.dead_timer > 0.0 {
        player.vel.y -= GRAVITY * dt;
        player.vel.y = player.vel.y.max(-FALL_MAX);
        tr.translation.x += player.vel.x * dt;
        tr.translation.y += player.vel.y * dt;
        return;
    }

    let was_on_ground = player.on_ground;
    player.vel.y -= GRAVITY * dt;
    player.vel.y = player.vel.y.max(-FALL_MAX);
    let fall_speed = (-player.vel.y).max(0.0);

    let mut pos = tr.translation.truncate();
    let p_size = player_size(player.prone);

    pos.x += player.vel.x * dt;
    let left_min = 110.0 + PLAYER_W * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let mut right_max = WORLD_W - PLAYER_W * 0.5;
    if stage.boss_spawned && !stage.boss_dead {
        right_max = right_max.min(stage.boss_x - BOSS_W * 0.5 - PLAYER_W * 0.5 - 4.0);
    }
    if pos.x > right_max {
        pos.x = right_max;
        if player.vel.x > 0.0 {
            player.vel.x = 0.0;
        }
    }
    for (st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dx = pos.x - sp.x;
            let push = (p_size.x + s.size.x) * 0.5 - dx.abs();
            if push > 0.0 {
                if dx > 0.0 {
                    pos.x += push;
                } else {
                    pos.x -= push;
                }
                player.vel.x = 0.0;
            }
        }
    }

    pos.y += player.vel.y * dt;
    let mut on_ground = false;
    for (st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dy = pos.y - sp.y;
            let push = (p_size.y + s.size.y) * 0.5 - dy.abs();
            if push > 0.0 {
                if dy > 0.0 {
                    pos.y += push;
                    if player.vel.y < 0.0 {
                        on_ground = true;
                        player.vel.y = 0.0;
                    }
                } else {
                    pos.y -= push;
                    if player.vel.y > 0.0 {
                        player.vel.y = 0.0;
                    }
                }
            }
        }
    }
    player.on_ground = on_ground;
    if !was_on_ground && on_ground && fall_speed > 180.0 {
        player.landing_t = 0.11;
    }

    tr.translation.x = pos.x;
    tr.translation.y = pos.y;

    if pos.y < FALL_DEATH_Y && player.dead_timer <= 0.0 {
        player.dead_timer = RESPAWN_TIME;
        player.vel = Vec2::new(0.0, 0.0);
        player.on_ground = false;
        player.coyote_ticks = 0;
        session.lives -= 1;
    }
}
