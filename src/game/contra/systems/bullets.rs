use bevy::prelude::*;

use crate::common::render::UiFont;
use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{aabb_overlap, boss_core_overlap, player_size};
use super::super::setup_actors::{spawn_explosion, spawn_pickup};

pub fn kill_player(player: &mut ContraPlayer, pos: Vec2, commands: &mut Commands) {
    player.dead_timer = RESPAWN_TIME;
    player.vel = Vec2::new(0.0, 320.0);
    player.on_ground = false;
    player.coyote_ticks = 0;
    spawn_explosion(commands, pos, 32.0, 0.45);
}

pub fn contra_bullets_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    font: Res<UiFont>,
    mut commands: Commands,
    mut bullet_q: Query<(Entity, &mut ContraBullet, &mut Transform)>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraBullet>>,
    mut enemy_q: Query<
        (Entity, &mut ContraEnemy, &Transform),
        (Without<ContraBullet>, Without<ContraSolid>),
    >,
    mut player_q: Query<
        (&mut ContraPlayer, &Transform),
        (Without<ContraBullet>, Without<ContraSolid>, Without<ContraEnemy>),
    >,
    mut turret_q: Query<
        (Entity, &mut ContraTurret, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
        ),
    >,
    mut boss_q: Query<
        (Entity, &mut ContraBoss, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
            Without<ContraTurret>,
        ),
    >,
    mut falcon_q: Query<
        (Entity, &ContraFalcon, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
            Without<ContraTurret>,
            Without<ContraBoss>,
        ),
    >,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let mut score_gain: u32 = 0;
    let mut killed_falcons: Vec<(Vec2, Weapon)> = Vec::new();

    for (be, mut bullet, mut btr) in &mut bullet_q {
        bullet.life -= dt;
        if bullet.life <= 0.0 {
            commands.entity(be).despawn();
            continue;
        }
        btr.translation.x += bullet.vel.x * dt;
        btr.translation.y += bullet.vel.y * dt;
        let bp = btr.translation.truncate();
        let bs = if matches!(bullet.weapon, Weapon::F) {
            Vec2::splat(FLAME_SIZE)
        } else if matches!(bullet.weapon, Weapon::L) {
            Vec2::splat(LASER_SIZE)
        } else if matches!(bullet.weapon, Weapon::S) {
            Vec2::splat(BULLET_BIG_SIZE)
        } else {
            Vec2::splat(BULLET_SIZE)
        };

        let mut blocked = false;
        for (st, s) in &solid_q {
            if aabb_overlap(bp, bs, st.translation.truncate(), s.size) {
                blocked = true;
                break;
            }
        }
        if blocked {
            spawn_explosion(&mut commands, bp, 14.0, 0.18);
            commands.entity(be).despawn();
            continue;
        }

        if bullet.from_player {
            let mut hit_enemy = false;
            for (ee, mut enemy, etr) in &mut enemy_q {
                if aabb_overlap(bp, bs, etr.translation.truncate(), Vec2::new(ENEMY_W, ENEMY_H)) {
                    enemy.hp -= 1;
                    enemy.hit_t = 0.10;
                    spawn_explosion(&mut commands, bp, 10.0, 0.12);
                    if enemy.hp <= 0 {
                        spawn_explosion(&mut commands, etr.translation.truncate(), 26.0, 0.32);
                        sfx.write(PlaySfx(SfxKind::Explosion));
                        commands.entity(ee).despawn();
                        score_gain += 100;
                    } else {
                        sfx.write(PlaySfx(SfxKind::Hit));
                    }
                    hit_enemy = true;
                    break;
                }
            }
            if hit_enemy {
                if !bullet.weapon.pierces_enemies() {
                    commands.entity(be).despawn();
                }
                continue;
            }
            let mut hit_t = false;
            for (te, mut turret, ttr) in &mut turret_q {
                if aabb_overlap(
                    bp,
                    bs,
                    ttr.translation.truncate(),
                    Vec2::new(TURRET_W, TURRET_H),
                ) {
                    turret.hp -= 1;
                    turret.hit_t = 0.10;
                    spawn_explosion(&mut commands, bp, 11.0, 0.12);
                    if turret.hp <= 0 {
                        spawn_explosion(&mut commands, ttr.translation.truncate(), 36.0, 0.5);
                        sfx.write(PlaySfx(SfxKind::Explosion));
                        commands.entity(te).despawn();
                        score_gain += 500;
                    } else {
                        sfx.write(PlaySfx(SfxKind::Hit));
                    }
                    hit_t = true;
                    break;
                }
            }
            if hit_t {
                if !matches!(bullet.weapon, Weapon::F | Weapon::L) {
                    commands.entity(be).despawn();
                }
                continue;
            }
            let mut hit_boss = false;
            for (_be2, mut boss, btr2) in &mut boss_q {
                let bpos = btr2.translation.truncate();
                if aabb_overlap(bp, bs, bpos, Vec2::new(BOSS_W, BOSS_H)) {
                    if boss.die_t <= 0.0 && boss_core_overlap(bp, bs, bpos) {
                        boss.hp -= 1;
                        boss.flash_t = 0.12;
                        score_gain += 50;
                        sfx.write(PlaySfx(SfxKind::Hit));
                        if boss.hp <= 0 {
                            boss.die_t = 1.4;
                        }
                    } else if boss.die_t <= 0.0 {
                        sfx.write(PlaySfx(SfxKind::Deny));
                    }
                    hit_boss = true;
                    spawn_explosion(&mut commands, bp, 14.0, 0.18);
                    break;
                }
            }
            if hit_boss {
                commands.entity(be).despawn();
                continue;
            }
            let mut hit_f = false;
            for (fe, falcon, ftr) in &mut falcon_q {
                if aabb_overlap(
                    bp,
                    bs,
                    ftr.translation.truncate(),
                    Vec2::new(FALCON_W, FALCON_H),
                ) {
                    spawn_explosion(&mut commands, ftr.translation.truncate(), 30.0, 0.3);
                    sfx.write(PlaySfx(SfxKind::Explosion));
                    killed_falcons.push((ftr.translation.truncate(), falcon.weapon));
                    commands.entity(fe).despawn();
                    score_gain += 200;
                    hit_f = true;
                    break;
                }
            }
            if hit_f {
                if !matches!(bullet.weapon, Weapon::F) {
                    commands.entity(be).despawn();
                }
                continue;
            }
        } else if let Ok((mut player, ptr)) = player_q.single_mut() {
            if player.dead_timer <= 0.0 && player.invincible <= 0.0 {
                let pp = ptr.translation.truncate();
                let ps = player_size(player.prone);
                if aabb_overlap(bp, bs, pp, ps) {
                    kill_player(&mut player, ptr.translation.truncate(), &mut commands);
                    sfx.write(PlaySfx(SfxKind::Hit));
                    commands.entity(be).despawn();
                    if session.lives > 0 {
                        session.lives -= 1;
                    }
                    continue;
                }
            }
        }
    }

    if score_gain > 0 {
        session.score += score_gain;
    }
    for (pos, w) in killed_falcons {
        spawn_pickup(&mut commands, &font, pos, w);
    }
}
