use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::{PLATFORM_RANGE, TILE};
use super::super::geometry::aabb_overlap;

pub fn mario_platform_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(&mut MovingPlatform, &mut Transform)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (mut plat, mut tr) in &mut q {
        let mut new_pos = tr.translation.truncate() + plat.vel * dt;
        let off = new_pos - plat.center;
        if plat.vertical {
            if off.y > PLATFORM_RANGE * 0.5 {
                new_pos.y = plat.center.y + PLATFORM_RANGE * 0.5;
                plat.vel.y = -plat.vel.y.abs();
            } else if off.y < -PLATFORM_RANGE * 0.5 {
                new_pos.y = plat.center.y - PLATFORM_RANGE * 0.5;
                plat.vel.y = plat.vel.y.abs();
            }
        } else if off.x > PLATFORM_RANGE * 0.5 {
            new_pos.x = plat.center.x + PLATFORM_RANGE * 0.5;
            plat.vel.x = -plat.vel.x.abs();
        } else if off.x < -PLATFORM_RANGE * 0.5 {
            new_pos.x = plat.center.x - PLATFORM_RANGE * 0.5;
            plat.vel.x = plat.vel.x.abs();
        }
        plat.last_dx = new_pos.x - tr.translation.x;
        plat.last_dy = new_pos.y - tr.translation.y;
        tr.translation.x = new_pos.x;
        tr.translation.y = new_pos.y;
    }
}

pub fn mario_lava_check(
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform), Without<LavaTile>>,
    lava_q: Query<&Transform, (With<LavaTile>, Without<MarioPlayer>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    if player.invincible > 5.0 {
        return;
    }
    for ltr in &lava_q {
        let lp = ltr.translation.truncate();
        if aabb_overlap(p_pos, p_size, lp, Vec2::splat(TILE)) {
            player.dead_timer = 2.2;
            player.vel = Vec2::new(0.0, 420.0);
            session.lives -= 1;
            return;
        }
    }
}
