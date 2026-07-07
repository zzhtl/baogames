use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::ContraStage;
use super::super::setup_actors::{spawn_enemy_bullet, spawn_explosion};

/// Boss 血量阶段：1(>2/3) / 2(1/3~2/3) / 3(<1/3)，越低越凶。
fn boss_phase(hp: i32, max_hp: i32) -> u8 {
    let frac = hp as f32 / max_hp.max(1) as f32;
    if frac > 0.66 {
        1
    } else if frac > 0.33 {
        2
    } else {
        3
    }
}

/// 各阶段炮塔射击间隔：阶段越高越密集。
fn turret_cd_for_phase(phase: u8) -> f32 {
    match phase {
        1 => TURRET_FIRE_CD,
        2 => TURRET_FIRE_CD * 0.62,
        _ => TURRET_FIRE_CD * 0.40,
    }
}

pub fn contra_boss_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut commands: Commands,
    mut stage: ResMut<ContraStage>,
    mut boss_q: Query<(Entity, &mut ContraBoss, &Transform)>,
    mut turret_q: Query<(&mut ContraTurret, &Transform), Without<ContraBoss>>,
    player_q: Query<&Transform, (With<ContraPlayer>, Without<ContraBoss>, Without<ContraTurret>)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let player_pos = player_q.single().ok().map(|t| t.translation.truncate());
    for (be, mut boss, btr) in &mut boss_q {
        boss.spawn_t += dt;
        if boss.flash_t > 0.0 {
            boss.flash_t = (boss.flash_t - dt).max(0.0);
        }
        if boss.die_t > 0.0 {
            boss.die_t -= dt;
            if (boss.die_t * 8.0).fract() < dt * 8.0 {
                let dx = ((boss.die_t * 13.0).sin()) * BOSS_W * 0.4;
                let dy = ((boss.die_t * 17.0).cos()) * BOSS_H * 0.4;
                spawn_explosion(
                    &mut commands,
                    btr.translation.truncate() + Vec2::new(dx, dy),
                    36.0,
                    0.4,
                );
            }
            if boss.die_t <= 0.0 {
                spawn_explosion(&mut commands, btr.translation.truncate(), 80.0, 0.7);
                sfx.write(PlaySfx(SfxKind::ExplosionBig));
                commands.entity(be).despawn();
                stage.boss_dead = true;
                if apply_boss_clear(&mut session, &mut save) {
                    save.store();
                }
            }
            continue;
        }
        let phase = boss_phase(boss.hp, stage.boss_hp);
        for (mut turret, ttr) in &mut turret_q {
            turret.fire_cd = (turret.fire_cd - dt).max(0.0);
            if turret.fire_cd <= 0.0 {
                if let Some(pp) = player_pos {
                    let origin = ttr.translation.truncate() + Vec2::new(-22.0, 0.0);
                    let mut dir = pp - origin;
                    if dir.length_squared() < 4.0 {
                        dir = Vec2::new(-1.0, 0.0);
                    }
                    spawn_enemy_bullet(&mut commands, origin, dir);
                    // 阶段 3：追加一发偏上的扇形弹，弹幕更密
                    if phase >= 3 {
                        spawn_enemy_bullet(&mut commands, origin, dir + Vec2::new(0.0, 70.0));
                    }
                    turret.fire_cd = turret_cd_for_phase(phase);
                }
            }
        }
    }
}

/// Boss 击破后处理通关：加分、更新存档。返回 true 表示需要持久化。
/// 抽成纯函数便于单测，IO 由调用方决定。
fn apply_boss_clear(session: &mut GameSession, save: &mut SaveData) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = true;
    // 先计入通关奖励再拼 status，覆盖层显示的得分才与存档一致
    session.score = session.score.saturating_add(5000);
    session.status = format!("要塞攻破 · 通关奖励 +5000 · 总分 {}", session.score);
    let idx = session.kind.index();
    let mut dirty = false;
    if session.score > save.high_scores[idx] {
        save.high_scores[idx] = session.score;
        dirty = true;
    }
    let next = (session.level + 1).min(10);
    if next > save.unlocked_levels[idx] {
        save.unlocked_levels[idx] = next;
        dirty = true;
    }
    dirty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::GameKind;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::Contra,
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
    fn boss_clear_marks_win_and_adds_bonus() {
        let mut session = fresh_session();
        session.score = 1000;
        let mut save = SaveData::default();
        let dirty = apply_boss_clear(&mut session, &mut save);
        assert!(dirty);
        assert!(session.finished);
        assert!(session.won);
        assert_eq!(session.score, 6000);
    }

    #[test]
    fn boss_clear_updates_save() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        apply_boss_clear(&mut session, &mut save);
        assert_eq!(save.high_scores[GameKind::Contra.index()], 5000);
        assert_eq!(save.unlocked_levels[GameKind::Contra.index()], 2);
    }

    #[test]
    fn boss_clear_caps_unlocked_level_at_ten() {
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        apply_boss_clear(&mut session, &mut save);
        assert_eq!(save.unlocked_levels[GameKind::Contra.index()], 10);
    }

    #[test]
    fn boss_clear_keeps_higher_existing_score() {
        let mut session = fresh_session();
        session.score = 100;
        let mut save = SaveData::default();
        save.high_scores[GameKind::Contra.index()] = 99999;
        let dirty = apply_boss_clear(&mut session, &mut save);
        // 关卡可能仍推进，但高分不被回退
        assert!(dirty);
        assert_eq!(save.high_scores[GameKind::Contra.index()], 99999);
    }

    #[test]
    fn boss_clear_idempotent() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let first = apply_boss_clear(&mut session, &mut save);
        let snapshot = session.score;
        let second = apply_boss_clear(&mut session, &mut save);
        assert!(first);
        assert!(!second);
        assert_eq!(session.score, snapshot, "再次调用不应加分");
    }

    #[test]
    fn boss_clear_no_progress_no_dirty() {
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        save.high_scores[GameKind::Contra.index()] = 99999;
        save.unlocked_levels[GameKind::Contra.index()] = 10;
        let dirty = apply_boss_clear(&mut session, &mut save);
        assert!(!dirty);
    }

    #[test]
    fn boss_phase_escalates_as_hp_drops() {
        assert_eq!(boss_phase(30, 30), 1);
        assert_eq!(boss_phase(18, 30), 2);
        assert_eq!(boss_phase(5, 30), 3);
        // 阶段越高炮塔射击越快
        assert!(turret_cd_for_phase(3) < turret_cd_for_phase(2));
        assert!(turret_cd_for_phase(2) < turret_cd_for_phase(1));
    }
}
