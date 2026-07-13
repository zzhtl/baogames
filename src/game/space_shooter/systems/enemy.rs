use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{
    ENEMY_BULLET_SPEED, PLAY_BOTTOM, PLAY_OFFSET_X, PLAY_TOP, TOTAL_WAVES_BEFORE_BOSS,
};
use super::super::resources::SpaceState;
use super::super::setup::{spawn_bullet, spawn_enemy};
use super::super::waves::build_wave;

pub fn space_spawner(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut state: ResMut<SpaceState>,
    enemies: Query<(), With<SpaceEnemy>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();

    if state.message_clock > 0.0 {
        state.message_clock -= delta;
        if state.message_clock <= 0.0 {
            state.message.clear();
        }
    }

    if state.wave_in_progress {
        state.wave_clock += delta;
        let mut spawned_now = Vec::new();
        let now = state.wave_clock;
        let mut remaining = Vec::with_capacity(state.pending.len());
        std::mem::swap(&mut remaining, &mut state.pending);
        for spawn in remaining.into_iter() {
            if now >= spawn.delay {
                spawned_now.push(spawn);
            } else {
                state.pending.push(spawn);
            }
        }
        let spawned_any = !spawned_now.is_empty();
        for s in spawned_now {
            spawn_enemy(&mut commands, s.kind, s.pos, s.drops_power);
        }
        // 当所有 pending 都已 spawn 且场上敌人清空 → 进入下一波
        if wave_is_clear(state.pending.is_empty(), enemies.iter().count(), spawned_any) {
            state.wave_in_progress = false;
            state.between_wave_clock = 1.2;
        }
    } else {
        state.between_wave_clock -= delta;
        if state.between_wave_clock <= 0.0 {
            let next_idx = state.wave_idx + 1;
            if next_idx >= TOTAL_WAVES_BEFORE_BOSS && !state.boss_spawned {
                // BOSS 出场
                let boss_pos = Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 90.0);
                spawn_enemy(&mut commands, EnemyKind::Boss, boss_pos, false);
                state.boss_spawned = true;
                state.boss_hp_max = EnemyKind::Boss.hp();
                state.message = "警告：BOSS 来袭".to_string();
                state.message_clock = 2.5;
                state.wave_in_progress = true;
                state.wave_clock = 0.0;
                state.pending.clear();
                state.wave_idx = next_idx;
            } else if state.boss_spawned {
                // 不再产生新波次
            } else {
                state.wave_idx = next_idx;
                state.wave_clock = 0.0;
                state.pending = build_wave(next_idx, session.level);
                state.wave_in_progress = true;
                state.message = format!("第 {} 波", next_idx + 1);
                state.message_clock = 1.4;
            }
        }
    }
}

pub fn space_enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    players: Query<&Transform, (With<SpaceShipPlayer>, Without<SpaceEnemy>)>,
    mut enemies: Query<(&mut Transform, &mut SpaceEnemy), Without<SpaceShipPlayer>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let player_pos = players.iter().next().map(|t| t.translation.truncate());
    for (mut t, mut enemy) in &mut enemies {
        enemy.time_alive += delta;
        enemy.hit_flash_left = (enemy.hit_flash_left - delta).max(0.0);
        update_enemy_position(&mut t, &enemy, delta, session.level);

        // 射击
        enemy.fire_cd_left -= delta;
        if enemy.fire_cd_left <= 0.0 && t.translation.y < PLAY_TOP - 20.0 {
            let pos = t.translation.truncate();
            fire_enemy_volley(&mut commands, pos, enemy.kind, enemy.time_alive, player_pos);
            enemy.fire_cd_left = enemy.kind.fire_period(session.level);
        }

        if t.translation.y < PLAY_BOTTOM - 60.0 {
            // 漏过去的小杂兵不计分
        }
    }
}

fn update_enemy_position(t: &mut Transform, enemy: &SpaceEnemy, delta: f32, level: u8) {
    match enemy.kind {
        EnemyKind::Scout => {
            let speed = 130.0 + level as f32 * 6.0;
            t.translation.y -= speed * delta;
            t.translation.x = enemy.spawn_x + (enemy.time_alive * 1.6).sin() * 24.0;
        }
        EnemyKind::Sniper => {
            let target_y = 80.0;
            if t.translation.y > target_y {
                let speed = 110.0 + level as f32 * 5.0;
                t.translation.y -= speed * delta;
            } else if enemy.time_alive < 4.0 {
                // 悬停轻微摆动
                t.translation.x = enemy.spawn_x + (enemy.time_alive * 1.2).sin() * 60.0;
            } else {
                let speed = 140.0;
                t.translation.y -= speed * delta;
            }
        }
        EnemyKind::Bomber => {
            let speed = 70.0 + level as f32 * 4.0;
            t.translation.y -= speed * delta;
            t.translation.x = enemy.spawn_x + (enemy.time_alive * 1.0).sin() * 80.0;
        }
        EnemyKind::Carrier => {
            let speed = 95.0;
            t.translation.y -= speed * delta;
            t.translation.x = enemy.spawn_x + (enemy.time_alive * 1.4).cos() * 40.0;
        }
        EnemyKind::Boss => {
            let target_y = 130.0;
            if t.translation.y > target_y {
                t.translation.y -= 60.0 * delta;
            } else {
                t.translation.x = PLAY_OFFSET_X + (enemy.time_alive * 0.8).sin() * 130.0;
            }
        }
    }
}

fn fire_enemy_volley(
    commands: &mut Commands,
    pos: Vec2,
    kind: EnemyKind,
    time_alive: f32,
    player_pos: Option<Vec2>,
) {
    match kind {
        EnemyKind::Scout => {
            spawn_bullet(
                commands,
                pos + Vec2::new(0.0, -16.0),
                Vec2::new(0.0, -ENEMY_BULLET_SPEED),
                false,
                1,
                Color::srgb(1.0, 0.55, 0.4),
                Vec2::new(6.0, 10.0),
            );
        }
        EnemyKind::Sniper => {
            if let Some(pp) = player_pos {
                let dir = (pp - pos).normalize_or_zero();
                spawn_bullet(
                    commands,
                    pos + dir * 18.0,
                    dir * ENEMY_BULLET_SPEED * 1.15,
                    false,
                    1,
                    Color::srgb(1.0, 0.78, 0.32),
                    Vec2::new(7.0, 7.0),
                );
            }
        }
        EnemyKind::Bomber => {
            for &dx in &[-110.0_f32, 0.0, 110.0] {
                spawn_bullet(
                    commands,
                    pos + Vec2::new(0.0, -18.0),
                    Vec2::new(dx, -ENEMY_BULLET_SPEED),
                    false,
                    1,
                    Color::srgb(0.7, 0.95, 0.6),
                    Vec2::new(7.0, 11.0),
                );
            }
        }
        EnemyKind::Carrier => {
            spawn_bullet(
                commands,
                pos + Vec2::new(0.0, -16.0),
                Vec2::new(0.0, -ENEMY_BULLET_SPEED * 0.85),
                false,
                1,
                Color::srgb(1.0, 0.55, 0.95),
                Vec2::new(8.0, 8.0),
            );
        }
        EnemyKind::Boss => fire_boss_pattern(commands, pos, time_alive, player_pos),
    }
}

fn fire_boss_pattern(
    commands: &mut Commands,
    pos: Vec2,
    time_alive: f32,
    player_pos: Option<Vec2>,
) {
    let phase = (time_alive / 3.0) as i32 % 3;
    match phase {
        0 => {
            // 5 路扇形
            for i in -2..=2 {
                let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * 0.18;
                let dir = Vec2::new(angle.cos(), angle.sin());
                spawn_bullet(
                    commands,
                    pos + Vec2::new(0.0, -36.0),
                    dir * ENEMY_BULLET_SPEED,
                    false,
                    1,
                    Color::srgb(1.0, 0.55, 0.5),
                    Vec2::new(9.0, 9.0),
                );
            }
        }
        1 => {
            // 双侧瞄准
            if let Some(pp) = player_pos {
                for &offset in &[-60.0_f32, 60.0] {
                    let from = pos + Vec2::new(offset, -8.0);
                    let dir = (pp - from).normalize_or_zero();
                    spawn_bullet(
                        commands,
                        from,
                        dir * ENEMY_BULLET_SPEED * 1.1,
                        false,
                        1,
                        Color::srgb(1.0, 0.78, 0.4),
                        Vec2::new(8.0, 8.0),
                    );
                }
            }
        }
        _ => {
            // 弹幕环
            for k in 0..10 {
                let a = (k as f32) * std::f32::consts::TAU / 10.0
                    - std::f32::consts::FRAC_PI_2;
                let dir = Vec2::new(a.cos(), a.sin());
                spawn_bullet(
                    commands,
                    pos + dir * 30.0,
                    dir * ENEMY_BULLET_SPEED * 0.85,
                    false,
                    1,
                    Color::srgb(1.0, 0.45, 0.55),
                    Vec2::new(8.0, 8.0),
                );
            }
        }
    }
}

pub fn space_despawn_offscreen(
    mut commands: Commands,
    session: Res<GameSession>,
    enemies: Query<(Entity, &Transform, &SpaceEnemy)>,
    powerups: Query<(Entity, &Transform), With<SpacePowerUp>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    for (e, t, enemy) in &enemies {
        if enemy.kind != EnemyKind::Boss && t.translation.y < PLAY_BOTTOM - 80.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &powerups {
        if t.translation.y < PLAY_BOTTOM - 40.0 {
            commands.entity(e).despawn();
        }
    }
}

fn wave_is_clear(pending_empty: bool, enemy_count: usize, spawned_now: bool) -> bool {
    pending_empty && enemy_count == 0 && !spawned_now
}

#[cfg(test)]
mod tests {
    use super::wave_is_clear;

    #[test]
    fn wave_does_not_finish_on_last_spawn_tick() {
        assert!(!wave_is_clear(true, 0, true));
        assert!(!wave_is_clear(false, 0, false));
        assert!(!wave_is_clear(true, 1, false));
        assert!(wave_is_clear(true, 0, false));
    }
}
