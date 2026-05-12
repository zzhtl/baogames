use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;
use super::super::setup_actors::build_player_visual;

pub fn mario_powerup_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut PowerUp, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<PowerUp>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let size = Vec2::splat(POWERUP_SIZE);
    for (e, mut pu, mut tr) in &mut q {
        let mut pos = tr.translation.truncate();

        if !pu.emerged {
            pos.y += POWERUP_EMERGE_SPEED * dt;
            if pos.y >= pu.emerge_y_target {
                pos.y = pu.emerge_y_target;
                pu.emerged = true;
                pu.vel = match pu.kind {
                    PowerUpKind::Mushroom | PowerUpKind::OneUp => Vec2::new(MUSHROOM_SPEED, 0.0),
                    PowerUpKind::Star => Vec2::new(MUSHROOM_SPEED * 0.9, 320.0),
                    PowerUpKind::FireFlower => Vec2::ZERO,
                };
            }
            tr.translation.x = pos.x;
            tr.translation.y = pos.y;
            continue;
        }

        if matches!(pu.kind, PowerUpKind::FireFlower) {
            tr.translation.x = pos.x;
            tr.translation.y = pos.y;
            if pos.y < FALL_DEATH_Y {
                commands.entity(e).despawn();
            }
            continue;
        }

        pu.vel.y -= GRAVITY * dt;
        pu.vel.y = pu.vel.y.max(-FALL_MAX);

        pos.x += pu.vel.x * dt;
        let mut bumped_x = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dx = pos.x - sp.x;
                let push = (size.x + s.size.x) * 0.5 - dx.abs();
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
            pu.vel.x = -pu.vel.x;
        }

        pos.y += pu.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if matches!(pu.kind, PowerUpKind::Star) {
                            pu.vel.y = 320.0;
                        } else if pu.vel.y < 0.0 {
                            pu.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if pu.vel.y > 0.0 {
                            pu.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        pu.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

pub fn mario_player_vs_powerup(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<PowerUp>>,
    powerup_q: Query<(Entity, &PowerUp, &Transform), Without<MarioPlayer>>,
) {
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let pu_size = Vec2::splat(POWERUP_SIZE);
    for (e, pu, ptr2) in &powerup_q {
        let _ = pu.emerged;
        let pp = ptr2.translation.truncate();
        if !aabb_overlap(p_pos, p_size, pp, pu_size) {
            continue;
        }
        match pu.kind {
            PowerUpKind::Mushroom => {
                if matches!(player.state, PowerState::Small) {
                    upgrade_player(&mut commands, player_e, &mut player, PowerState::Big);
                }
                session.score = session.score.saturating_add(1000);
            }
            PowerUpKind::FireFlower => {
                let next = match player.state {
                    PowerState::Small => PowerState::Big,
                    PowerState::Big | PowerState::Fire => PowerState::Fire,
                };
                if next != player.state {
                    upgrade_player(&mut commands, player_e, &mut player, next);
                }
                session.score = session.score.saturating_add(1000);
            }
            PowerUpKind::OneUp => {
                session.lives += 1;
            }
            PowerUpKind::Star => {
                player.invincible = 9.0;
                session.score = session.score.saturating_add(1000);
            }
        }
        commands.entity(e).despawn();
    }
}

fn upgrade_player(
    commands: &mut Commands,
    player_e: Entity,
    player: &mut MarioPlayer,
    target: PowerState,
) {
    player.state = target;
    player.transform_t = 0.6;
    commands.entity(player_e).despawn_related::<Children>();
    build_player_visual(commands, player_e, target);
}
