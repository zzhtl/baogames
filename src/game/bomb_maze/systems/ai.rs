use bevy::prelude::*;
use rand::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{
    BM_COLS, BM_ENEMY_SIZE, BM_OFFSET_X, BM_OFFSET_Y, BM_PLAY_H, BM_PLAY_W, BM_ROWS,
    BM_TURN_WINDOW,
};
use super::super::geometry::{nearest_point, try_turn_at_tile, world_to_tile};

pub fn bm_enemy_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut enemies: Query<(&mut Transform, &mut BMEnemy)>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<&BMTilePos, With<BMSoftWall>>,
    bombs: Query<&BMTilePos, With<BMBomb>>,
    players: Query<&Transform, (With<BMPlayer>, Without<BMEnemy>)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();

    let mut hard = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            hard[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut soft = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &soft_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            soft[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut bomb_grid = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &bombs {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            bomb_grid[tp.col as usize][tp.row as usize] = true;
        }
    }

    let player_positions: Vec<Vec2> = players.iter().map(|t| t.translation.truncate()).collect();
    let mut rng = thread_rng();

    for (mut tf, mut enemy) in &mut enemies {
        let speed = enemy.kind.speed();
        let wall_pass = enemy.kind.wall_pass();
        let pos = tf.translation.truncate();
        let dir_v = enemy.dir.vec();
        let step = dir_v * speed * delta;
        let new_pos = pos + step;

        let half = (BM_ENEMY_SIZE - 4.0) * 0.5;
        let pmin = Vec2::new(
            BM_OFFSET_X - BM_PLAY_W * 0.5 + half + 1.0,
            BM_OFFSET_Y - BM_PLAY_H * 0.5 + half + 1.0,
        );
        let pmax = Vec2::new(
            BM_OFFSET_X + BM_PLAY_W * 0.5 - half - 1.0,
            BM_OFFSET_Y + BM_PLAY_H * 0.5 - half - 1.0,
        );
        let out =
            new_pos.x < pmin.x || new_pos.x > pmax.x || new_pos.y < pmin.y || new_pos.y > pmax.y;
        let blocked_at = |c: i32, r: i32| -> bool {
            if !(0..BM_COLS).contains(&c) || !(0..BM_ROWS).contains(&r) {
                return true;
            }
            if hard[c as usize][r as usize] {
                return true;
            }
            if !wall_pass && soft[c as usize][r as usize] {
                return true;
            }
            if bomb_grid[c as usize][r as usize] {
                return true;
            }
            false
        };

        let look = pos + dir_v * (BM_ENEMY_SIZE * 0.5 + 6.0);
        let (lc, lr) = world_to_tile(look);
        let blocked_ahead = blocked_at(lc, lr);

        enemy.change_timer -= delta;

        if blocked_ahead || out || enemy.change_timer <= 0.0 {
            let mut candidates: Vec<Dir4> = Dir4::all().to_vec();
            candidates.shuffle(&mut rng);

            let hunt = enemy.kind.hunt();
            if !player_positions.is_empty() && rng.r#gen::<f32>() < hunt {
                let target = nearest_point(&player_positions, pos);
                let toward = (target - pos).normalize_or_zero();
                let mut towards: Vec<Dir4> = Vec::new();
                if toward.x.abs() > 0.1 {
                    towards.push(if toward.x > 0.0 {
                        Dir4::Right
                    } else {
                        Dir4::Left
                    });
                }
                if toward.y.abs() > 0.1 {
                    towards.push(if toward.y > 0.0 {
                        Dir4::Up
                    } else {
                        Dir4::Down
                    });
                }
                towards.shuffle(&mut rng);
                for c in towards.into_iter().rev() {
                    candidates.retain(|x| *x != c);
                    candidates.insert(0, c);
                }
            }

            let mut new_dir = enemy.dir;
            for c in &candidates {
                if blocked_ahead && *c == enemy.dir.opposite() && candidates.len() > 1 {
                    continue;
                }
                let probe = pos + c.vec() * (BM_ENEMY_SIZE * 0.5 + 6.0);
                let (pc, pr) = world_to_tile(probe);
                if !blocked_at(pc, pr) {
                    new_dir = *c;
                    break;
                }
            }
            // 如果完全被困，允许调头
            if new_dir == enemy.dir && blocked_ahead {
                let probe = pos + enemy.dir.opposite().vec() * (BM_ENEMY_SIZE * 0.5 + 6.0);
                let (pc, pr) = world_to_tile(probe);
                if !blocked_at(pc, pr) {
                    new_dir = enemy.dir.opposite();
                }
            }
            if new_dir == enemy.dir
                || try_turn_at_tile(&mut tf.translation, enemy.dir, new_dir, BM_TURN_WINDOW)
            {
                enemy.dir = new_dir;
                enemy.change_timer = rng.gen_range(0.9..2.4);
            } else {
                // 尚未走到路口中线，沿原方向继续靠近，避免提前瞬移到相邻格。
                if !out {
                    tf.translation.x = new_pos.x;
                    tf.translation.y = new_pos.y;
                }
                enemy.change_timer = 0.12;
            }
        } else {
            tf.translation.x = new_pos.x;
            tf.translation.y = new_pos.y;
        }
    }
}
