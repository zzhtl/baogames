use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession, SaveData};

use super::super::components::*;
use super::super::constants::{P1_SPAWN, P2_SPAWN};
use super::super::geometry::aabb_overlap;
use super::super::resources::BMStage;
use super::super::setup::spawn_bm_player;

pub fn bm_exit_and_respawn(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<BMStage>,
    enemies: Query<&BMEnemy>,
    exits: Query<(&Transform, &Collider), With<BMExit>>,
    players: Query<(&Transform, &Collider), With<BMPlayer>>,
    all_players: Query<&BMPlayer>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    if session.finished || session.paused {
        return;
    }

    let enemy_count = enemies.iter().count();

    if enemy_count == 0 && !stage.all_enemies_dead_msg_shown {
        stage.all_enemies_dead_msg_shown = true;
        stage.status = "敌人清干净了，去出口！".to_string();
    }

    // 出口判定
    if enemy_count == 0 {
        let exit_data: Vec<(Vec2, Vec2)> = exits
            .iter()
            .map(|(t, c)| (t.translation.truncate(), c.size))
            .collect();
        for (pt, pc) in &players {
            let pp = pt.translation.truncate();
            for (ep, es) in &exit_data {
                if aabb_overlap(pp, pc.size, *ep, *es) {
                    finish_bomb_maze(&mut session, &mut save, &stage, true);
                    return;
                }
            }
        }
    }

    // 时间到 → 失败
    if stage.time_left <= 0.0 {
        finish_bomb_maze(&mut session, &mut save, &stage, false);
        return;
    }

    // 双方阵亡 → 失败
    let any_alive = !all_players.is_empty();
    let p1_dead_done = stage.p1_lives < 0;
    let p2_dead_done = stage.p2_lives < 0;
    if !any_alive && p1_dead_done && p2_dead_done {
        finish_bomb_maze(&mut session, &mut save, &stage, false);
        return;
    }

    // 复活逻辑：玩家不存在 + 还有命 → 重新生成
    let mut p1_alive = false;
    let mut p2_alive = false;
    for p in &all_players {
        if p.id == 0 {
            p1_alive = true;
        } else if p.id == 1 {
            p2_alive = true;
        }
    }
    if !p1_alive && stage.p1_lives >= 0 && stage.p1_respawn <= 0.0 {
        spawn_bm_player(&mut commands, 0, P1_SPAWN);
    }
    if !p2_alive && stage.p2_lives >= 0 && stage.p2_respawn <= 0.0 {
        spawn_bm_player(&mut commands, 1, P2_SPAWN);
    }
}

fn finish_bomb_maze(
    session: &mut GameSession,
    save: &mut SaveData,
    stage: &BMStage,
    won: bool,
) {
    session.finished = true;
    session.won = won;
    if won {
        session.score += 500 + (stage.time_left * 5.0) as u32;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((stage.level + 1).min(10));
        save.store();
        session.status = "成功逃出迷宫！Enter 重玩，Esc 返回".to_string();
    } else {
        session.status = "迷宫之旅失败……Enter 重试，Esc 返回".to_string();
    }
}

pub fn bm_player_blink(
    time: Res<Time>,
    session: Res<GameSession>,
    mut players: Query<(&BMPlayer, &mut Sprite)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let t = time.elapsed_secs();
    for (player, mut sprite) in &mut players {
        if player.invuln > 0.0 {
            let pulse = ((t * 18.0).sin() * 0.5 + 0.5) as f32;
            sprite.color = Color::srgba(0.95, 0.95, 0.98, 0.4 + 0.6 * pulse);
        } else {
            sprite.color = Color::srgb(0.95, 0.95, 0.98);
        }
    }
}
