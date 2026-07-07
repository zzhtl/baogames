use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;
use super::koopa::apply_player_damage;

pub fn mario_goomba_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_goomba: Query<(Entity, &mut Goomba, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<Goomba>)>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let g_size = Vec2::new(GOOMBA_W, GOOMBA_H);
    for (e, mut g, mut tr) in &mut q_goomba {
        if g.dead {
            commands.entity(e).despawn();
            continue;
        }
        if g.squashed > 0.0 {
            g.squashed -= dt;
            tr.scale.y = (g.squashed / GOOMBA_SQUASH_TIME).max(0.1);
            if g.squashed <= 0.0 {
                g.dead = true;
            }
            continue;
        }
        g.vel.y -= GRAVITY * dt;
        g.vel.y = g.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();
        pos.x += g.vel.x * dt;
        let mut bumped_x = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, g_size, sp, s.size) {
                let dx = pos.x - sp.x;
                let push = (g_size.x + s.size.x) * 0.5 - dx.abs();
                if push > 0.0 {
                    if dx > 0.0 {
                        pos.x += push;
                    } else {
                        pos.x -= push;
                    }
                    bumped_x = true;
                }
            }
        }
        if bumped_x {
            g.vel.x = -g.vel.x;
        }
        pos.y += g.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, g_size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (g_size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if g.vel.y < 0.0 {
                            g.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if g.vel.y > 0.0 {
                            g.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        g.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

pub fn mario_player_vs_goomba(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<Goomba>>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<MarioPlayer>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let star = player.invincible > 5.0;
    for (mut g, gtr) in &mut goomba_q {
        if g.dead || g.squashed > 0.0 {
            continue;
        }
        let g_pos = gtr.translation.truncate();
        let g_size = Vec2::new(GOOMBA_W, GOOMBA_H);
        if !aabb_overlap(p_pos, p_size, g_pos, g_size) {
            continue;
        }
        if star {
            g.squashed = GOOMBA_SQUASH_TIME;
            g.vel.x = 0.0;
            session.score = session.score.saturating_add(200);
            continue;
        }
        let p_foot = p_pos.y - p_size.y * 0.5;
        if p_foot > g_pos.y - 4.0 && player.vel.y <= 30.0 {
            g.squashed = GOOMBA_SQUASH_TIME;
            g.vel.x = 0.0;
            player.vel.y = STOMP_BOUNCE;
            session.score = session.score.saturating_add(100);
            sfx.write(PlaySfx(SfxKind::Stomp));
        } else {
            apply_player_damage(&mut commands, player_e, &mut player, &mut session);
        }
    }
}
