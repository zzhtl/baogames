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
                            if finish_space(&mut session, &mut save, true) {
                                save.store();
                            }
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
        let _ = finish_space(session, save, false);
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

/// 返回 true 表示 `save` 已被更新、需要持久化到磁盘。
/// IO 副作用从这里抽出便于单测。
fn finish_space(session: &mut GameSession, save: &mut SaveData, won: bool) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = won;
    if won {
        session.score += 1000;
        let idx = GameKind::SpaceShooter.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
        true
    } else {
        session.status = "战机被击毁……Enter 重试，Esc 返回".to_string();
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::SpaceShooter,
            level: 1,
            score: 0,
            lives: 3,
            paused: false,
            finished: false,
            won: false,
            status: String::new(),
        }
    }

    #[test]
    fn win_grants_bonus_unlocks_level_and_marks_dirty() {
        let mut session = fresh_session();
        session.score = 500;
        let mut save = SaveData::default();
        let dirty = finish_space(&mut session, &mut save, true);
        assert!(dirty);
        assert!(session.finished);
        assert!(session.won);
        assert_eq!(session.score, 1500);
        assert_eq!(save.high_scores[GameKind::SpaceShooter.index()], 1500);
        assert!(save.unlocked_levels[GameKind::SpaceShooter.index()] >= 2);
    }

    #[test]
    fn loss_does_not_unlock_or_dirty_save() {
        let mut session = fresh_session();
        session.score = 200;
        let mut save = SaveData::default();
        let dirty = finish_space(&mut session, &mut save, false);
        assert!(!dirty);
        assert!(session.finished);
        assert!(!session.won);
        // 失败不更新最高分，只在通关时记录
        assert_eq!(save.high_scores[GameKind::SpaceShooter.index()], 0);
    }

    #[test]
    fn finish_is_idempotent() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        finish_space(&mut session, &mut save, true);
        let snapshot = session.score;
        let second = finish_space(&mut session, &mut save, false);
        assert!(!second);
        assert_eq!(session.score, snapshot);
        assert!(session.won, "second call must not flip outcome");
    }

    #[test]
    fn high_score_does_not_demote_existing_record() {
        let mut session = fresh_session();
        session.score = 100;
        let mut save = SaveData::default();
        save.high_scores[GameKind::SpaceShooter.index()] = 9999;
        finish_space(&mut session, &mut save, true);
        assert_eq!(save.high_scores[GameKind::SpaceShooter.index()], 9999);
    }

    #[test]
    fn unlocked_level_caps_at_ten() {
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        finish_space(&mut session, &mut save, true);
        assert_eq!(save.unlocked_levels[GameKind::SpaceShooter.index()], 10);
    }
}
