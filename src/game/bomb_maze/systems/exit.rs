use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
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
    mut sfx: MessageWriter<PlaySfx>,
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
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                    if finish_bomb_maze(&mut session, &mut save, &stage, true) {
                        save.store();
                    }
                    return;
                }
            }
        }
    }

    // 时间到 → 失败
    if stage.time_left <= 0.0 {
        if finish_bomb_maze(&mut session, &mut save, &stage, false) {
            save.store();
        }
        return;
    }

    // 双方阵亡 → 失败
    let any_alive = !all_players.is_empty();
    let p1_dead_done = stage.p1_lives < 0;
    let p2_dead_done = stage.p2_lives < 0;
    if !any_alive && p1_dead_done && p2_dead_done {
        if finish_bomb_maze(&mut session, &mut save, &stage, false) {
            save.store();
        }
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

/// 返回 true 表示 `save` 已被更新、需要持久化到磁盘。
/// IO 副作用从这里抽出便于单测。
fn finish_bomb_maze(
    session: &mut GameSession,
    save: &mut SaveData,
    stage: &BMStage,
    won: bool,
) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = won;
    if won {
        session.score += 500 + (stage.time_left * 5.0) as u32;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((stage.level + 1).min(10));
        session.status = format!("成功逃出迷宫 · 得分 {}", session.score);
        true
    } else {
        session.status = format!("迷宫之旅失败 · 得分 {}", session.score);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::GameKind;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::BombMaze,
            level: 1,
            score: 100,
            lives: 3,
            paused: false,
            finished: false,
            won: false,
            status: String::new(),
        }
    }

    fn fresh_stage() -> BMStage {
        BMStage {
            level: 1,
            time_left: 60.0,
            p1_lives: 3,
            p2_lives: 3,
            p1_respawn: 0.0,
            p2_respawn: 0.0,
            all_enemies_dead_msg_shown: false,
            status: String::new(),
        }
    }

    #[test]
    fn finish_win_adds_time_bonus_and_returns_dirty() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let stage = fresh_stage();
        let dirty = finish_bomb_maze(&mut session, &mut save, &stage, true);
        assert!(dirty);
        assert!(session.finished);
        assert!(session.won);
        // 100 (初始) + 500 (通关) + 60 * 5 (时间奖励) = 900
        assert_eq!(session.score, 900);
        assert_eq!(save.high_scores[GameKind::BombMaze.index()], 900);
        assert!(save.unlocked_levels[GameKind::BombMaze.index()] >= 2);
    }

    #[test]
    fn finish_loss_does_not_dirty_save() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let stage = fresh_stage();
        let dirty = finish_bomb_maze(&mut session, &mut save, &stage, false);
        assert!(!dirty);
        assert!(session.finished);
        assert!(!session.won);
        assert_eq!(save.high_scores[GameKind::BombMaze.index()], 0);
    }

    #[test]
    fn finish_is_idempotent() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let stage = fresh_stage();
        finish_bomb_maze(&mut session, &mut save, &stage, true);
        let snapshot = session.score;
        let second = finish_bomb_maze(&mut session, &mut save, &stage, false);
        assert!(!second);
        assert_eq!(session.score, snapshot, "再次调用不应改动分数");
        assert!(session.won, "won 不应被覆盖");
    }

    #[test]
    fn finish_caps_unlocked_level_at_ten() {
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        let mut stage = fresh_stage();
        stage.level = 10;
        finish_bomb_maze(&mut session, &mut save, &stage, true);
        assert_eq!(save.unlocked_levels[GameKind::BombMaze.index()], 10);
    }
}

pub fn bm_player_blink(
    time: Res<Time>,
    session: Res<GameSession>,
    mut players: Query<(&BMPlayer, &mut Visibility)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let t = time.elapsed_secs();
    for (player, mut visibility) in &mut players {
        if player.invuln > 0.0 && (t * 18.0).sin() < 0.0 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Inherited;
        }
    }
}
