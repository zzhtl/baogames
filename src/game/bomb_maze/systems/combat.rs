use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
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
    mut sfx: MessageWriter<PlaySfx>,
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
                sfx.write(PlaySfx(SfxKind::Hit));
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
                sfx.write(PlaySfx(SfxKind::Hit));
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
    mut sfx: MessageWriter<PlaySfx>,
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
                sfx.write(PlaySfx(SfxKind::Hit));
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
    mut sfx: MessageWriter<PlaySfx>,
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
                sfx.write(PlaySfx(SfxKind::Powerup));
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
    apply_death(stage, player.id);
}

/// 把"扣一条命 + 安排复活"的纯逻辑剥出来便于单测。
/// 约定：lives < 0 表示永久死亡；spawner 据此停止复活，exit 据此判定 game over。
pub(super) fn apply_death(stage: &mut BMStage, player_id: usize) {
    if player_id == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_stage() -> BMStage {
        BMStage {
            level: 1,
            time_left: 200.0,
            p1_lives: 3,
            p2_lives: 3,
            p1_respawn: 0.0,
            p2_respawn: 0.0,
            all_enemies_dead_msg_shown: false,
            status: String::new(),
        }
    }

    #[test]
    fn death_decrements_lives_and_arms_respawn() {
        let mut s = fresh_stage();
        apply_death(&mut s, 0);
        assert_eq!(s.p1_lives, 2);
        assert!((s.p1_respawn - BM_RESPAWN_TIME).abs() < 1e-3);
        // p2 不受影响
        assert_eq!(s.p2_lives, 3);
        assert_eq!(s.p2_respawn, 0.0);
    }

    #[test]
    fn last_life_still_arms_respawn() {
        let mut s = fresh_stage();
        s.p1_lives = 1;
        apply_death(&mut s, 0);
        // 减到 0 仍 >=0 → 允许复活
        assert_eq!(s.p1_lives, 0);
        assert!(s.p1_respawn > 0.0);
    }

    #[test]
    fn lives_below_zero_does_not_arm_respawn() {
        let mut s = fresh_stage();
        s.p1_lives = 0;
        apply_death(&mut s, 0);
        // 减到 -1 → 永久死亡，不应再设置 respawn
        assert_eq!(s.p1_lives, -1);
        assert_eq!(s.p1_respawn, 0.0);
    }

    #[test]
    fn death_affects_player2_independently() {
        let mut s = fresh_stage();
        apply_death(&mut s, 1);
        assert_eq!(s.p2_lives, 2);
        assert!(s.p2_respawn > 0.0);
        assert_eq!(s.p1_lives, 3);
    }
}
