use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::BM_RESPAWN_TIME;
use super::super::geometry::aabb_overlap;
use super::super::resources::BMStage;

pub fn bm_flame_damage(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<BMStage>,
    flames: Query<(&Transform, &Collider), With<BMFlame>>,
    enemies: Query<(Entity, &Transform, &Collider, &BMEnemy)>,
    mut players: Query<(Entity, &Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let flame_data: Vec<(Vec2, Vec2)> = flames
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    if flame_data.is_empty() {
        return;
    }

    for (enemy_e, et, ec, enemy) in &enemies {
        let ep = et.translation.truncate();
        for (fp, fs) in &flame_data {
            if aabb_overlap(ep, ec.size, *fp, *fs) {
                session.score += enemy.kind.score();
                commands.entity(enemy_e).despawn();
                break;
            }
        }
    }

    for (player_e, pt, pc, mut player) in &mut players {
        if player.invuln > 0.0 {
            continue;
        }
        let pp = pt.translation.truncate();
        for (fp, fs) in &flame_data {
            if aabb_overlap(pp, pc.size, *fp, *fs) {
                hurt_player(&mut commands, &mut stage, player_e, &mut player);
                break;
            }
        }
    }
}

pub fn bm_enemy_touch(
    mut commands: Commands,
    session: Res<GameSession>,
    mut stage: ResMut<BMStage>,
    enemies: Query<(&Transform, &Collider), With<BMEnemy>>,
    mut players: Query<(Entity, &Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let enemy_data: Vec<(Vec2, Vec2)> = enemies
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    for (player_e, pt, pc, mut player) in &mut players {
        if player.invuln > 0.0 {
            continue;
        }
        let pp = pt.translation.truncate();
        for (ep, es) in &enemy_data {
            if aabb_overlap(pp, pc.size, *ep, *es) {
                hurt_player(&mut commands, &mut stage, player_e, &mut player);
                break;
            }
        }
    }
}

pub fn bm_powerup_pickup(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    powerups: Query<(Entity, &Transform, &Collider, &BMPowerup)>,
    mut players: Query<(&Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let mut consumed: Vec<Entity> = Vec::new();
    for (item_e, it, ic, powerup) in &powerups {
        let ip = it.translation.truncate();
        for (pt, pc, mut player) in &mut players {
            if aabb_overlap(ip, ic.size, pt.translation.truncate(), pc.size) {
                player.apply_powerup(powerup.kind);
                session.score += 50;
                consumed.push(item_e);
                break;
            }
        }
    }
    for e in consumed {
        commands.entity(e).despawn();
    }
}

fn hurt_player(
    commands: &mut Commands,
    stage: &mut BMStage,
    entity: Entity,
    player: &mut BMPlayer,
) {
    commands.entity(entity).despawn();
    if player.id == 0 {
        stage.p1_lives -= 1;
        if stage.p1_lives >= 0 {
            stage.p1_respawn = BM_RESPAWN_TIME;
        }
    } else {
        stage.p2_lives -= 1;
        if stage.p2_lives >= 0 {
            stage.p2_respawn = BM_RESPAWN_TIME;
        }
    }
}
