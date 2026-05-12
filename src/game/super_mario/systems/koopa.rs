use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;
use super::super::setup_actors::build_player_visual;

pub fn mario_koopa_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_koopa: Query<(Entity, &mut Koopa, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<Koopa>)>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (e, mut k, mut tr) in &mut q_koopa {
        if k.dead {
            commands.entity(e).despawn();
            continue;
        }
        let size = match k.state {
            KoopaState::Walking => Vec2::new(KOOPA_W, KOOPA_H),
            _ => Vec2::new(SHELL_W, SHELL_H),
        };
        if matches!(k.state, KoopaState::Shell) {
            k.state_t -= dt;
            if k.state_t <= 0.0 {
                k.state = KoopaState::Walking;
                k.vel.x = -KOOPA_SPEED;
            }
        }

        k.vel.y -= GRAVITY * dt;
        k.vel.y = k.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();

        pos.x += k.vel.x * dt;
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
            k.vel.x = -k.vel.x;
        }

        let edge_check = matches!(k.state, KoopaState::Walking)
            && matches!(k.kind, KoopaKind::Red)
            && k.on_ground;
        if edge_check {
            let probe = Vec2::new(
                pos.x + k.vel.x.signum() * (size.x * 0.5 + 4.0),
                pos.y - size.y * 0.5 - 4.0,
            );
            let mut on_floor = false;
            for (st, s) in &solid_q {
                let sp = st.translation.truncate();
                if (probe.x - sp.x).abs() < s.size.x * 0.5
                    && (probe.y - sp.y).abs() < s.size.y * 0.5 + 6.0
                {
                    on_floor = true;
                    break;
                }
            }
            if !on_floor {
                k.vel.x = -k.vel.x;
            }
        }

        pos.y += k.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if k.vel.y < 0.0 {
                            k.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if k.vel.y > 0.0 {
                            k.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        k.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

pub fn apply_player_damage(
    commands: &mut Commands,
    player_e: Entity,
    player: &mut MarioPlayer,
    session: &mut GameSession,
) {
    match player.state {
        PowerState::Fire => {
            player.state = PowerState::Big;
            player.transform_t = 0.6;
            player.invincible = 1.6;
            commands.entity(player_e).despawn_related::<Children>();
            build_player_visual(commands, player_e, PowerState::Big);
        }
        PowerState::Big => {
            player.state = PowerState::Small;
            player.transform_t = 0.6;
            player.invincible = 1.6;
            commands.entity(player_e).despawn_related::<Children>();
            build_player_visual(commands, player_e, PowerState::Small);
        }
        PowerState::Small => {
            player.dead_timer = 2.2;
            player.vel = Vec2::new(0.0, 420.0);
            session.lives -= 1;
        }
    }
}

pub fn mario_player_vs_koopa(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<Koopa>>,
    mut koopa_q: Query<(&mut Koopa, &Transform), Without<MarioPlayer>>,
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
    for (mut k, ktr) in &mut koopa_q {
        if k.dead {
            continue;
        }
        let k_pos = ktr.translation.truncate();
        let k_size = match k.state {
            KoopaState::Walking => Vec2::new(KOOPA_W, KOOPA_H),
            _ => Vec2::new(SHELL_W, SHELL_H),
        };
        if !aabb_overlap(p_pos, p_size, k_pos, k_size) {
            continue;
        }
        let p_foot = p_pos.y - p_size.y * 0.5;
        let stomping = p_foot > k_pos.y - 4.0 && player.vel.y <= 30.0;
        if star {
            k.dead = true;
            session.score = session.score.saturating_add(200);
            continue;
        }
        match k.state {
            KoopaState::Walking => {
                if stomping {
                    k.state = KoopaState::Shell;
                    k.state_t = 5.0;
                    k.vel.x = 0.0;
                    player.vel.y = STOMP_BOUNCE;
                    session.score = session.score.saturating_add(100);
                } else if player.invincible <= 0.0 && player.transform_t <= 0.0 {
                    apply_player_damage(&mut commands, player_e, &mut player, &mut session);
                }
            }
            KoopaState::Shell => {
                let dir = if p_pos.x < k_pos.x { 1.0 } else { -1.0 };
                k.state = KoopaState::SlidingShell;
                k.vel.x = SHELL_SPEED * dir;
                player.vel.y = STOMP_BOUNCE * 0.5;
                session.score = session.score.saturating_add(400);
            }
            KoopaState::SlidingShell => {
                if stomping {
                    k.state = KoopaState::Shell;
                    k.state_t = 5.0;
                    k.vel.x = 0.0;
                    player.vel.y = STOMP_BOUNCE;
                } else if player.invincible <= 0.0 && player.transform_t <= 0.0 {
                    apply_player_damage(&mut commands, player_e, &mut player, &mut session);
                }
            }
        }
    }
}

pub fn mario_shell_kills(
    mut session: ResMut<GameSession>,
    koopa_q: Query<(&Koopa, &Transform)>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<Koopa>>,
) {
    if session.paused {
        return;
    }
    let mut shells: Vec<Vec2> = Vec::new();
    for (k, tr) in &koopa_q {
        if matches!(k.state, KoopaState::SlidingShell) && !k.dead {
            shells.push(tr.translation.truncate());
        }
    }
    if shells.is_empty() {
        return;
    }
    let shell_size = Vec2::new(SHELL_W, SHELL_H);
    for (mut g, gtr) in &mut goomba_q {
        if g.dead || g.squashed > 0.0 {
            continue;
        }
        let gp = gtr.translation.truncate();
        for sp in &shells {
            if aabb_overlap(*sp, shell_size, gp, Vec2::new(GOOMBA_W, GOOMBA_H)) {
                g.squashed = GOOMBA_SQUASH_TIME;
                g.vel.x = 0.0;
                session.score = session.score.saturating_add(200);
                break;
            }
        }
    }
}
