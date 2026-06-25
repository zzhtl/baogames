use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession, SaveData, Velocity};

use super::super::components::*;
use super::super::constants::{BULLET_SIZE, RESPAWN_TIME, STAGE_TOTAL_ENEMIES};
use super::super::geometry::{aabb_overlap, play_max, play_min};
use super::super::resources::TankStage;
use super::super::setup::{spawn_explosion, spawn_powerup};

/// 掉落道具种类：按累计击杀数与关卡循环 6 种。
fn powerup_for(kills: u8, stage_num: u8) -> PowerUpKind {
    const CYCLE: [PowerUpKind; 6] = [
        PowerUpKind::Star,
        PowerUpKind::Grenade,
        PowerUpKind::Helmet,
        PowerUpKind::Tank,
        PowerUpKind::Clock,
        PowerUpKind::Shovel,
    ];
    CYCLE[(kills as usize / 5 + stage_num as usize) % CYCLE.len()]
}

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
                // 每累计 5 杀掉一个道具（最后一杀通关，不掉）
                if stage.kills % 5 == 0 && stage.kills < STAGE_TOTAL_ENEMIES {
                    spawn_powerup(&mut commands, pos, powerup_for(stage.kills, stage.stage_num));
                }
            }
        }
    }

    for entity in consumed {
        commands.entity(entity).despawn();
    }

    if check_stage_completion(&mut session, &mut save, &stage) {
        save.store();
    }
}

fn apply_player_death(stage: &mut TankStage, player_id: usize) {
    // 死亡后先扣命，扣完才判断是否还有剩余命可以复活：
    // - 还有剩余命 → 设置 RESPAWN_TIME 等待 spawner 安排复活
    // - 没有剩余命 → respawn 标记为 -1.0，告诉 spawner 永久死亡，并触发全员阵亡判定
    if player_id == 0 {
        if stage.p1_lives > 0 {
            stage.p1_lives -= 1;
        }
        stage.p1_respawn = if stage.p1_lives > 0 {
            RESPAWN_TIME
        } else {
            -1.0
        };
    } else {
        if stage.p2_lives > 0 {
            stage.p2_lives -= 1;
        }
        stage.p2_respawn = if stage.p2_lives > 0 {
            RESPAWN_TIME
        } else {
            -1.0
        };
    }
}

/// 返回值表示是否需要把 `save` 持久化到磁盘。
/// 把 IO 副作用抽出来便于单测，避免测试污染真实存档。
fn check_stage_completion(
    session: &mut GameSession,
    save: &mut SaveData,
    stage: &TankStage,
) -> bool {
    if session.finished {
        return false;
    }
    if stage.kills >= STAGE_TOTAL_ENEMIES {
        session.finished = true;
        session.won = true;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
        return true;
    } else if !stage.base_alive {
        session.finished = true;
        session.won = false;
    } else if stage.p1_respawn < 0.0 && stage.p2_respawn < 0.0 {
        session.finished = true;
        session.won = false;
        session.status = "全员阵亡！".to_string();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::{GameKind, GameSession, SaveData};

    fn fresh_stage() -> TankStage {
        TankStage {
            remaining_to_spawn: STAGE_TOTAL_ENEMIES,
            spawn_timer: 0.0,
            spawn_idx: 0,
            stage_num: 1,
            p1_lives: 2,
            p2_lives: 2,
            p1_respawn: 0.0,
            p2_respawn: 0.0,
            base_alive: true,
            kills: 0,
            two_player: false,
            mode_selected: true,
            freeze_timer: 0.0,
        }
    }

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::Tank,
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
    fn player_death_consumes_one_life_and_arms_respawn() {
        let mut stage = fresh_stage();
        apply_player_death(&mut stage, 0);
        assert_eq!(stage.p1_lives, 1);
        assert!((stage.p1_respawn - RESPAWN_TIME).abs() < 1e-3);
    }

    #[test]
    fn player_death_with_last_life_marks_permanent() {
        let mut stage = fresh_stage();
        stage.p1_lives = 1;
        apply_player_death(&mut stage, 0);
        // 唯一一条命用完后 lives 归零、respawn=-1，避免“假复活”和卡死
        assert_eq!(stage.p1_lives, 0);
        assert_eq!(stage.p1_respawn, -1.0);
    }

    #[test]
    fn player_death_with_zero_lives_stays_permanent() {
        let mut stage = fresh_stage();
        stage.p1_lives = 0;
        stage.p1_respawn = -1.0;
        apply_player_death(&mut stage, 0);
        assert_eq!(stage.p1_lives, 0);
        assert_eq!(stage.p1_respawn, -1.0);
    }

    #[test]
    fn player2_death_independent_from_player1() {
        let mut stage = fresh_stage();
        apply_player_death(&mut stage, 1);
        assert_eq!(stage.p1_lives, 2);
        assert_eq!(stage.p2_lives, 1);
        assert!((stage.p2_respawn - RESPAWN_TIME).abs() < 1e-3);
    }

    #[test]
    fn full_death_loop_ends_in_permanent_state() {
        // 复现 bug：连续死亡直到 lives 归零，必须到达 respawn=-1 状态
        let mut stage = fresh_stage();
        stage.p1_lives = 2;
        apply_player_death(&mut stage, 0); // lives 2 -> 1
        apply_player_death(&mut stage, 0); // lives 1 -> 0
        assert_eq!(stage.p1_lives, 0);
        assert_eq!(stage.p1_respawn, -1.0);
    }

    #[test]
    fn stage_completion_won_when_all_enemies_killed() {
        let mut stage = fresh_stage();
        stage.kills = STAGE_TOTAL_ENEMIES;
        let mut session = fresh_session();
        session.score = 250;
        let mut save = SaveData::default();
        let needs_save = check_stage_completion(&mut session, &mut save, &stage);
        assert!(needs_save);
        assert!(session.finished);
        assert!(session.won);
        // 通关后高分应被更新
        assert_eq!(save.high_scores[GameKind::Tank.index()], 250);
        // 下一关被解锁
        assert!(save.unlocked_levels[GameKind::Tank.index()] >= 2);
    }

    #[test]
    fn stage_completion_lost_when_base_destroyed() {
        let mut stage = fresh_stage();
        stage.base_alive = false;
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let needs_save = check_stage_completion(&mut session, &mut save, &stage);
        assert!(!needs_save);
        assert!(session.finished);
        assert!(!session.won);
    }

    #[test]
    fn stage_completion_lost_when_both_players_permanent_dead() {
        let mut stage = fresh_stage();
        stage.p1_respawn = -1.0;
        stage.p2_respawn = -1.0;
        let mut session = fresh_session();
        let mut save = SaveData::default();
        check_stage_completion(&mut session, &mut save, &stage);
        assert!(session.finished);
        assert!(!session.won);
        assert!(session.status.contains("全员阵亡"));
    }

    #[test]
    fn stage_completion_single_player_uses_p2_sentinel() {
        let mut stage = fresh_stage();
        stage.p1_respawn = -1.0;
        stage.p2_respawn = -1.0;
        stage.p2_lives = 0;
        let mut session = fresh_session();
        let mut save = SaveData::default();
        check_stage_completion(&mut session, &mut save, &stage);
        assert!(session.finished);
    }

    #[test]
    fn stage_completion_in_progress_does_nothing() {
        let stage = fresh_stage();
        let mut session = fresh_session();
        let snapshot_status = session.status.clone();
        let mut save = SaveData::default();
        let needs_save = check_stage_completion(&mut session, &mut save, &stage);
        assert!(!needs_save);
        assert!(!session.finished);
        assert_eq!(session.status, snapshot_status);
    }

    #[test]
    fn stage_completion_caps_unlocked_level_at_ten() {
        let mut stage = fresh_stage();
        stage.kills = STAGE_TOTAL_ENEMIES;
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        check_stage_completion(&mut session, &mut save, &stage);
        assert_eq!(save.unlocked_levels[GameKind::Tank.index()], 10);
    }

    #[test]
    fn stage_completion_does_not_demote_save_progress() {
        // 玩家从更高关回来打低关，不应回退已解锁的关卡或最高分
        let mut stage = fresh_stage();
        stage.kills = STAGE_TOTAL_ENEMIES;
        let mut session = fresh_session();
        session.level = 1;
        session.score = 50;
        let mut save = SaveData::default();
        save.high_scores[GameKind::Tank.index()] = 999;
        save.unlocked_levels[GameKind::Tank.index()] = 5;
        check_stage_completion(&mut session, &mut save, &stage);
        assert_eq!(save.high_scores[GameKind::Tank.index()], 999);
        assert_eq!(save.unlocked_levels[GameKind::Tank.index()], 5);
    }

    #[test]
    fn powerup_cycle_covers_kinds_across_stages() {
        let mut seen = std::collections::HashSet::new();
        for stage in 1..=6u8 {
            for k in (5..=15).step_by(5) {
                seen.insert(format!("{:?}", powerup_for(k, stage)));
            }
        }
        // 跨关卡应能见到全部 6 种道具
        assert_eq!(seen.len(), 6);
    }
}
