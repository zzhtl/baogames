use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use crate::common::collide::{Solid as Solid_, resolve};

use super::super::geometry::player_size;
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

    let p_size = player_size(player.prone);
    // 与超级玛丽同源的公共解算：先脱困再分轴推进。
    // 旧写法在 X 轴上无条件按穿透深度整体推出，木桥比陆地低 1 单位时
    // 会把人每帧顶回岸边（走不上岸），近跳跃顶点时又会把人瞬移到平台上面。
    let shapes: Vec<Solid_> = solid_q
        .iter()
        .map(|(st, s)| Solid_::fixed(st.translation.truncate(), s.size))
        .collect();
    let resolved = resolve(tr.translation.truncate(), p_size, player.vel, dt, &shapes);
    let mut pos = resolved.pos;
    player.vel = resolved.vel;
    player.on_ground = resolved.on_ground;
    let on_ground = resolved.on_ground;

    let left_min = 110.0 + p_size.x * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let mut right_max = WORLD_W - p_size.x * 0.5;
    if stage.boss_spawned && !stage.boss_dead {
        right_max = right_max.min(stage.boss_x - BOSS_W * 0.5 - p_size.x * 0.5 - 4.0);
    }
    if pos.x > right_max {
        pos.x = right_max;
        if player.vel.x > 0.0 {
            player.vel.x = 0.0;
        }
    }
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
