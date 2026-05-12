use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession, SaveData, Velocity};

use super::super::components::*;
use super::super::constants::{BULLET_SIZE, RESPAWN_TIME, STAGE_TOTAL_ENEMIES};
use super::super::geometry::{aabb_overlap, play_max, play_min};
use super::super::resources::TankStage;
use super::super::setup::spawn_explosion;

pub fn tank_bullet_update(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<TankStage>,
    mut bullets: Query<(Entity, &mut Transform, &Velocity, &BulletFC)>,
    mut tanks: Query<(Entity, &Transform, &Collider, &mut TankFC), Without<BulletFC>>,
    players: Query<&PlayerTankFC>,
    bricks: Query<(Entity, &Transform, &Collider), (With<BrickFC>, Without<BulletFC>)>,
    steels: Query<(&Transform, &Collider), (With<SteelFC>, Without<BulletFC>)>,
    bases: Query<(Entity, &Transform, &Collider), (With<BaseFC>, Without<BulletFC>)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let pmin = play_min();
    let pmax = play_max();

    let mut consumed: Vec<Entity> = Vec::new();
    let mut destroyed_tanks: Vec<Entity> = Vec::new();
    let mut bullet_owner_release: Vec<Entity> = Vec::new();

    for (entity, mut tf, vel, bullet) in &mut bullets {
        if consumed.contains(&entity) {
            continue;
        }
        tf.translation += vel.extend(0.0) * delta;
        let pos = tf.translation.truncate();
        let half = BULLET_SIZE * 0.5;
        if pos.x - half < pmin.x
            || pos.x + half > pmax.x
            || pos.y - half < pmin.y
            || pos.y + half > pmax.y
        {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        let bsize = Vec2::splat(BULLET_SIZE);
        // 命中砖块（破最近的若干子砖）
        let mut hit_brick = false;
        for (be, bt, bc) in &bricks {
            if aabb_overlap(pos, bsize, bt.translation.truncate(), bc.size) {
                commands.entity(be).despawn();
                hit_brick = true;
                break;
            }
        }
        if hit_brick {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        // 命中钢墙
        let mut hit_steel = false;
        for (st, sc) in &steels {
            if aabb_overlap(pos, bsize, st.translation.truncate(), sc.size) {
                hit_steel = true;
                break;
            }
        }
        if hit_steel {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        // 命中老巢
        let mut hit_base = false;
        for (base_e, bt, bc) in &bases {
            if aabb_overlap(pos, bsize, bt.translation.truncate(), bc.size) {
                commands.entity(base_e).despawn();
                hit_base = true;
                break;
            }
        }
        if hit_base {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            stage.base_alive = false;
            spawn_explosion(&mut commands, pos, true);
            session.finished = true;
            session.won = false;
            session.status = "基地被打掉了！".to_string();
            continue;
        }

        // 命中坦克
        let mut hit_tank: Option<Entity> = None;
        for (te, tt, tc, _) in tanks.iter() {
            // 同阵营不互伤；自己不伤自己
            if Some(te) == bullet.owner {
                continue;
            }
            if aabb_overlap(pos, bsize, tt.translation.truncate(), tc.size) {
                hit_tank = Some(te);
                break;
            }
        }
        if let Some(victim) = hit_tank {
            if let Ok((_, _, _, mut victim_tank)) = tanks.get_mut(victim) {
                if victim_tank.side == bullet.side {
                    // 同阵营：仅消耗子弹
                    consumed.push(entity);
                    if let Some(owner) = bullet.owner {
                        bullet_owner_release.push(owner);
                    }
                    spawn_explosion(&mut commands, pos, false);
                    continue;
                }
                if victim_tank.shield_left > 0.0 {
                    consumed.push(entity);
                    if let Some(owner) = bullet.owner {
                        bullet_owner_release.push(owner);
                    }
                    spawn_explosion(&mut commands, pos, false);
                    continue;
                }
                victim_tank.hp = victim_tank.hp.saturating_sub(1);
                if victim_tank.hp == 0 {
                    destroyed_tanks.push(victim);
                }
            }
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, true);
        }
    }

    for owner in bullet_owner_release {
        if let Ok((_, _, _, mut t)) = tanks.get_mut(owner) {
            t.bullets_alive = t.bullets_alive.saturating_sub(1);
        }
    }

    for victim in destroyed_tanks {
        if let Ok((_, vt, _, vtank)) = tanks.get(victim) {
            let pos = vt.translation.truncate();
            let was_player = vtank.side == TankSide::Player;
            spawn_explosion(&mut commands, pos, true);
            commands.entity(victim).despawn();
            if was_player {
                if let Ok(p) = players.get(victim) {
                    apply_player_death(&mut stage, p.id);
                }
            } else {
                stage.kills += 1;
                session.score += 100;
            }
        }
    }

    for entity in consumed {
        commands.entity(entity).despawn();
    }

    check_stage_completion(&mut session, &mut save, &stage);
}

fn apply_player_death(stage: &mut TankStage, player_id: usize) {
    if player_id == 0 {
        if stage.p1_lives > 0 {
            stage.p1_lives -= 1;
            stage.p1_respawn = RESPAWN_TIME;
        } else {
            stage.p1_respawn = -1.0;
        }
    } else if stage.p2_lives > 0 {
        stage.p2_lives -= 1;
        stage.p2_respawn = RESPAWN_TIME;
    } else {
        stage.p2_respawn = -1.0;
    }
}

fn check_stage_completion(
    session: &mut GameSession,
    save: &mut SaveData,
    stage: &TankStage,
) {
    if session.finished {
        return;
    }
    if stage.kills >= STAGE_TOTAL_ENEMIES {
        session.finished = true;
        session.won = true;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else if !stage.base_alive {
        session.finished = true;
        session.won = false;
    } else if stage.p1_respawn < 0.0 && stage.p2_respawn < 0.0 {
        session.finished = true;
        session.won = false;
        session.status = "全员阵亡！".to_string();
    }
}
