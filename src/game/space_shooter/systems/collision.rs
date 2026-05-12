use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession, SaveData};

use super::super::components::*;
use super::super::constants::{PLAYER_INVINCIBLE, PLAYER_RESPAWN_X, PLAYER_RESPAWN_Y};
use super::super::geometry::aabb;
use super::super::resources::SpaceState;
use super::super::setup::{spawn_explosion, spawn_powerup};

pub fn space_collisions(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut state: ResMut<SpaceState>,
    mut enemies: Query<
        (Entity, &Transform, &Collider, &mut SpaceEnemy),
        (Without<SpaceShipPlayer>, Without<SpaceBullet>),
    >,
    bullets: Query<
        (Entity, &Transform, &Collider, &SpaceBullet),
        (Without<SpaceShipPlayer>, Without<SpaceEnemy>),
    >,
    mut players: Query<
        (Entity, &mut Transform, &Collider, &mut SpaceShipPlayer),
        (Without<SpaceEnemy>, Without<SpaceBullet>),
    >,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }

    // 子弹 vs 敌机 / 玩家
    for (be, bt, bc, b) in &bullets {
        if b.from_player {
            for (ee, et, ec, mut enemy) in &mut enemies {
                if aabb(
                    bt.translation.truncate(),
                    bc.size,
                    et.translation.truncate(),
                    ec.size,
                ) {
                    enemy.hp -= b.damage;
                    commands.entity(be).despawn();
                    if enemy.hp <= 0 {
                        session.score += enemy.points;
                        let pos = et.translation.truncate();
                        let big = matches!(enemy.kind, EnemyKind::Bomber | EnemyKind::Boss);
                        spawn_explosion(&mut commands, pos, big);
                        if enemy.drops_power {
                            spawn_powerup(&mut commands, pos);
                        }
                        if enemy.kind == EnemyKind::Boss {
                            state.boss_defeated = true;
                            state.message = "BOSS 已摧毁！".to_string();
                            state.message_clock = 3.0;
                            finish_space(&mut session, &mut save, true);
                        }
                        commands.entity(ee).despawn();
                    }
                    break;
                }
            }
        } else {
            // 敌人子弹击中玩家
            for (pe, mut pt, pc, mut player) in &mut players {
                if player.invincible_left > 0.0 {
                    continue;
                }
                if aabb(
                    bt.translation.truncate(),
                    bc.size,
                    pt.translation.truncate(),
                    pc.size,
                ) {
                    commands.entity(be).despawn();
                    player_take_hit(
                        &mut commands,
                        &mut session,
                        &mut save,
                        &mut state,
                        &mut player,
                        &mut pt,
                        pe,
                    );
                    break;
                }
            }
        }
    }

    // 敌机撞玩家
    let mut enemies_to_kill: Vec<Entity> = Vec::new();
    for (ee, et, ec, enemy) in &enemies {
        for (pe, mut pt, pc, mut player) in &mut players {
            if player.invincible_left > 0.0 {
                continue;
            }
            if aabb(
                et.translation.truncate(),
                ec.size,
                pt.translation.truncate(),
                pc.size,
            ) {
                if enemy.kind != EnemyKind::Boss {
                    enemies_to_kill.push(ee);
                    spawn_explosion(&mut commands, et.translation.truncate(), false);
                }
                player_take_hit(
                    &mut commands,
                    &mut session,
                    &mut save,
                    &mut state,
                    &mut player,
                    &mut pt,
                    pe,
                );
                break;
            }
        }
    }
    for e in enemies_to_kill {
        commands.entity(e).despawn();
    }
}

fn player_take_hit(
    commands: &mut Commands,
    session: &mut GameSession,
    save: &mut SaveData,
    state: &mut SpaceState,
    player: &mut SpaceShipPlayer,
    player_t: &mut Transform,
    player_entity: Entity,
) {
    spawn_explosion(commands, player_t.translation.truncate(), true);
    session.lives -= 1;
    if session.lives < 0 {
        session.lives = 0;
        finish_space(session, save, false);
        commands.entity(player_entity).despawn();
        return;
    }
    state.power = state.power.saturating_sub(1).max(1);
    state.message = "再来一次！".to_string();
    state.message_clock = 1.2;
    player_t.translation.x = PLAYER_RESPAWN_X;
    player_t.translation.y = PLAYER_RESPAWN_Y;
    player.invincible_left = PLAYER_INVINCIBLE;
}

fn finish_space(session: &mut GameSession, save: &mut SaveData, won: bool) {
    if session.finished {
        return;
    }
    session.finished = true;
    session.won = won;
    if won {
        session.score += 1000;
        let idx = GameKind::SpaceShooter.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else {
        session.status = "战机被击毁……Enter 重试，Esc 返回".to_string();
    }
}
