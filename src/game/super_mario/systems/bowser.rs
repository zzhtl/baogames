use bevy::prelude::*;

use crate::game::model::{GameEntity, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;
use super::super::palette::COLOR_FIREBALL_EDGE;
use super::super::resources::MarioStage;
use super::koopa::apply_player_damage;

pub fn mario_bowser_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_bow: Query<(&mut Bowser, &mut Transform), (Without<MarioPlayer>, Without<Solid>)>,
    solid_q: Query<(&Transform, &Solid), (Without<Bowser>, Without<MarioPlayer>)>,
    player_q: Query<&Transform, (With<MarioPlayer>, Without<Bowser>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok(ptr) = player_q.single() else { return };
    let p_pos = ptr.translation.truncate();

    for (mut bow, mut tr) in &mut q_bow {
        if bow.dead {
            continue;
        }
        bow.dir_t -= dt;
        if bow.dir_t <= 0.0 {
            bow.vel.x = -bow.vel.x;
            bow.dir_t = 2.0 + (tr.translation.x.abs().fract() * 1.3);
        }

        bow.vel.y -= GRAVITY * dt;
        bow.vel.y = bow.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();
        let size = Vec2::new(BOWSER_W, BOWSER_H);

        pos.x += bow.vel.x * dt;
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
            bow.vel.x = -bow.vel.x;
        }

        pos.y += bow.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if bow.vel.y < 0.0 {
                            bow.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if bow.vel.y > 0.0 {
                            bow.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        bow.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if on_ground && (bow.dir_t * 1.7).fract() > 0.85 {
            bow.vel.y = 360.0;
        }

        bow.fire_cd -= dt;
        if bow.fire_cd <= 0.0 {
            bow.fire_cd = BOWSER_FIRE_CD;
            let dir = if p_pos.x < pos.x { -1.0 } else { 1.0 };
            commands.spawn((
                Sprite::from_color(COLOR_FIREBALL_EDGE, Vec2::splat(16.0)),
                Transform::from_translation(Vec3::new(
                    pos.x + dir * 28.0,
                    pos.y + 4.0,
                    Z_FIREBALL,
                )),
                BowserFireball {
                    vel: Vec2::new(BOWSER_FIREBALL_SPEED * dir, 0.0),
                    life: 4.0,
                },
                GameEntity,
            ));
        }
    }
}

pub fn mario_bowser_fireball_update(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut q: Query<(Entity, &mut BowserFireball, &mut Transform), Without<MarioPlayer>>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<BowserFireball>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();

    for (e, mut fb, mut tr) in &mut q {
        fb.life -= dt;
        if fb.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        tr.translation.x += fb.vel.x * dt;
        let pos = tr.translation.truncate();
        if aabb_overlap(pos, Vec2::splat(16.0), p_pos, p_size) {
            commands.entity(e).despawn();
            if player.invincible <= 0.0
                && player.transform_t <= 0.0
                && !(player.invincible > 5.0)
            {
                apply_player_damage(&mut commands, player_e, &mut player, &mut session);
            }
        }
    }
}

pub fn mario_player_fire_vs_bowser(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    fire_q: Query<(Entity, &Transform), (With<Fireball>, Without<Bowser>)>,
    mut bow_q: Query<(&mut Bowser, &Transform), Without<Fireball>>,
) {
    let bowser_size = Vec2::new(BOWSER_W, BOWSER_H);
    for (mut bow, btr) in &mut bow_q {
        if bow.dead {
            continue;
        }
        let bp = btr.translation.truncate();
        for (fe, ftr) in &fire_q {
            let fp = ftr.translation.truncate();
            if aabb_overlap(fp, Vec2::splat(FIREBALL_SIZE), bp, bowser_size) {
                bow.hp -= 1;
                commands.entity(fe).despawn();
                session.score = session.score.saturating_add(200);
                if bow.hp <= 0 {
                    bow.dead = true;
                    session.score = session.score.saturating_add(5000);
                }
            }
        }
    }
}

pub fn mario_bowser_cleanup(mut commands: Commands, q: Query<(Entity, &Bowser)>) {
    for (e, b) in &q {
        if b.dead {
            commands.entity(e).despawn();
        }
    }
}

pub fn mario_axe_check(
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut player_q: Query<(&mut MarioPlayer, &Transform), Without<Axe>>,
    axe_q: Query<&Transform, (With<Axe>, Without<MarioPlayer>)>,
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
    let Ok(atr) = axe_q.single() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    let a_pos = atr.translation.truncate();
    if aabb_overlap(p_pos, player.state.size(), a_pos, Vec2::splat(AXE_SIZE)) {
        player.finished = true;
        stage.finish_timer = 0.0;
        session.score = session.score.saturating_add(2000);
    }
}
