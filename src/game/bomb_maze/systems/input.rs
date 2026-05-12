use bevy::prelude::*;

use crate::common::input::input_for;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{aabb_overlap, tile_center, world_to_tile};
use super::super::resources::BMStage;
use super::super::setup::spawn_bm_bomb;

pub fn bm_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<BMStage>,
    mut players: Query<(Entity, &mut Transform, &mut BMPlayer)>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<&BMTilePos, With<BMSoftWall>>,
    mut bombs: Query<(Entity, &mut BMBomb, &BMTilePos)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    stage.time_left = (stage.time_left - delta).max(0.0);
    if stage.p1_respawn > 0.0 {
        stage.p1_respawn = (stage.p1_respawn - delta).max(0.0);
    }
    if stage.p2_respawn > 0.0 {
        stage.p2_respawn = (stage.p2_respawn - delta).max(0.0);
    }

    // 网格映射
    let mut blocked = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            blocked[tp.col as usize][tp.row as usize] = true;
        }
    }
    for tp in &soft_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            blocked[tp.col as usize][tp.row as usize] = true;
        }
    }
    let bomb_meta: Vec<(Entity, i32, i32, Option<Entity>, bool)> = bombs
        .iter()
        .map(|(e, b, tp)| (e, tp.col, tp.row, b.owner, b.remote))
        .collect();
    let bomb_tiles: Vec<(Entity, i32, i32)> = bomb_meta
        .iter()
        .map(|(e, c, r, _, _)| (*e, *c, *r))
        .collect();

    let mut occupied_bomb_cells: std::collections::HashSet<(i32, i32)> =
        bomb_tiles.iter().map(|(_, c, r)| (*c, *r)).collect();

    let mut place_requests: Vec<(Entity, i32, i32, i32, bool)> = Vec::new();
    let mut detonate_requests: Vec<Entity> = Vec::new();

    for (entity, mut tf, mut player) in &mut players {
        player.place_cd = (player.place_cd - delta).max(0.0);
        player.detonate_cd = (player.detonate_cd - delta).max(0.0);
        if player.invuln > 0.0 {
            player.invuln = (player.invuln - delta).max(0.0);
        }

        let input = input_for(&keys, player.id);
        let mut dir = input.move_dir;
        if dir.length_squared() > 1.01 {
            dir = dir.normalize();
        }

        let speed = player.speed();
        let pos = tf.translation.truncate();

        let target_pos = move_with_collision(
            pos,
            dir,
            speed,
            delta,
            &blocked,
            &bomb_tiles,
            entity,
            &player.walking_off,
            player.bomb_pass,
        );

        tf.translation.x = target_pos.x;
        tf.translation.y = target_pos.y;

        // 离开自己放下的炸弹后，从豁免列表移除：
        // 用 bbox 重叠判断，避免玩家走到 tile 边界时（中心刚跨格、bbox 还压在炸弹上）
        // 因 walking_off 立即清空被自家炸弹卡住。
        if !player.walking_off.is_empty() {
            player.walking_off.retain(|bomb_e| {
                bomb_tiles.iter().any(|(e, c, r)| {
                    if *e != *bomb_e {
                        return false;
                    }
                    let cell_pos = tile_center(*c, *r);
                    aabb_overlap(
                        target_pos,
                        Vec2::splat(BM_PLAYER_SIZE - 4.0),
                        cell_pos,
                        Vec2::splat(BM_BOMB_SIZE - 6.0),
                    )
                })
            });
        }

        // 放炸弹
        if input.fire && player.place_cd <= 0.0 && player.bombs_alive < player.max_bombs {
            let (pc, pr) = world_to_tile(target_pos);
            if !occupied_bomb_cells.contains(&(pc, pr)) && !blocked[pc as usize][pr as usize] {
                place_requests.push((entity, pc, pr, player.bomb_range, player.remote));
                occupied_bomb_cells.insert((pc, pr));
                player.place_cd = BM_PLACE_CD;
            }
        }

        // 遥控引爆：只能引爆自己拥有的、最早放下的一颗遥控炸弹
        if input.jump && player.remote && player.detonate_cd <= 0.0 {
            if let Some((bomb_e, _, _, _, _)) = bomb_meta
                .iter()
                .find(|(_, _, _, owner, remote)| *remote && *owner == Some(entity))
            {
                detonate_requests.push(*bomb_e);
                player.detonate_cd = 0.18;
            }
        }
    }

    // 实际放置炸弹
    for (owner, c, r, range, remote) in place_requests {
        let bomb = spawn_bm_bomb(&mut commands, c, r, range, Some(owner), remote);
        if let Ok((_e, _, mut p)) = players.get_mut(owner) {
            p.bombs_alive += 1;
            p.walking_off.push(bomb);
        }
    }
    // 远程引爆：直接把对应炸弹标记为已触发
    for e in detonate_requests {
        if let Ok((_, mut bomb, _)) = bombs.get_mut(e) {
            bomb.triggered = true;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn move_with_collision(
    pos: Vec2,
    dir: Vec2,
    speed: f32,
    delta: f32,
    blocked: &[Vec<bool>],
    bombs: &[(Entity, i32, i32)],
    self_entity: Entity,
    walking_off: &[Entity],
    bomb_pass: bool,
) -> Vec2 {
    let half = (BM_PLAYER_SIZE - 4.0) * 0.5;
    let pmin = Vec2::new(
        BM_OFFSET_X - BM_PLAY_W * 0.5 + half + 1.0,
        BM_OFFSET_Y - BM_PLAY_H * 0.5 + half + 1.0,
    );
    let pmax = Vec2::new(
        BM_OFFSET_X + BM_PLAY_W * 0.5 - half - 1.0,
        BM_OFFSET_Y + BM_PLAY_H * 0.5 - half - 1.0,
    );

    let collide = |p: Vec2| -> bool {
        let half_v = Vec2::splat(half);
        // 网格阻挡：用四角分别求所属 tile，再对网格范围取 min/max
        let corners = [
            p + Vec2::new(-half_v.x, -half_v.y),
            p + Vec2::new(half_v.x, -half_v.y),
            p + Vec2::new(-half_v.x, half_v.y),
            p + Vec2::new(half_v.x, half_v.y),
        ];
        let mut cmin = i32::MAX;
        let mut cmax = i32::MIN;
        let mut rmin = i32::MAX;
        let mut rmax = i32::MIN;
        for cp in corners {
            let (cc, rr) = world_to_tile(cp);
            cmin = cmin.min(cc);
            cmax = cmax.max(cc);
            rmin = rmin.min(rr);
            rmax = rmax.max(rr);
        }
        for c in cmin..=cmax {
            for r in rmin..=rmax {
                if !(0..BM_COLS).contains(&c) || !(0..BM_ROWS).contains(&r) {
                    return true;
                }
                if blocked[c as usize][r as usize] {
                    let cell_pos = tile_center(c, r);
                    if aabb_overlap(p, Vec2::splat(half * 2.0), cell_pos, Vec2::splat(BM_TILE - 1.0)) {
                        return true;
                    }
                }
            }
        }
        // 炸弹阻挡（除非穿弹/或正在豁免）
        if !bomb_pass {
            for (bomb_e, c, r) in bombs {
                if walking_off.contains(bomb_e) {
                    continue;
                }
                let _ = self_entity;
                let cell_pos = tile_center(*c, *r);
                if aabb_overlap(
                    p,
                    Vec2::splat(half * 2.0),
                    cell_pos,
                    Vec2::splat(BM_BOMB_SIZE - 6.0),
                ) {
                    return true;
                }
            }
        }
        false
    };

    let mut new_pos = pos;
    let dx = dir.x * speed * delta;
    let dy = dir.y * speed * delta;
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    let move_x = abs_dx > 1e-3;
    let move_y = abs_dy > 1e-3;
    let cardinal_x = move_x && !move_y;
    let cardinal_y = move_y && !move_x;
    let snap_step = speed * delta;

    // 计算朝当前 tile 中心吸附（仅单轴移动时使用），返回偏移量
    let snap_to_row_center = |p: Vec2| -> f32 {
        let (_, row) = world_to_tile(p);
        let row_y = tile_center(0, row).y;
        let diff = row_y - p.y;
        if diff.abs() < 0.5 {
            0.0
        } else if diff.abs() < snap_step {
            diff
        } else {
            diff.signum() * snap_step
        }
    };
    let snap_to_col_center = |p: Vec2| -> f32 {
        let (col, _) = world_to_tile(p);
        let col_x = tile_center(col, 0).x;
        let diff = col_x - p.x;
        if diff.abs() < 0.5 {
            0.0
        } else if diff.abs() < snap_step {
            diff
        } else {
            diff.signum() * snap_step
        }
    };

    // X 方向
    if move_x {
        let snap_y = if cardinal_x { snap_to_row_center(new_pos) } else { 0.0 };
        let target_x = (new_pos.x + dx).clamp(pmin.x, pmax.x);
        let try_combo = Vec2::new(target_x, (new_pos.y + snap_y).clamp(pmin.y, pmax.y));
        if !collide(try_combo) {
            new_pos = try_combo;
        } else {
            let try_x = Vec2::new(target_x, new_pos.y);
            if !collide(try_x) {
                new_pos = try_x;
            } else if cardinal_x {
                // 撞墙时尝试更大幅度的角落贴边，最大半个 tile
                let (_, row) = world_to_tile(new_pos);
                let row_y = tile_center(0, row).y;
                let diff = row_y - new_pos.y;
                if diff.abs() > 0.5 && diff.abs() < BM_TILE * 0.5 {
                    let step = diff.signum() * snap_step;
                    let nudged_y = if step.abs() > diff.abs() {
                        row_y
                    } else {
                        new_pos.y + step
                    };
                    let candidate = Vec2::new(target_x, nudged_y).clamp(pmin, pmax);
                    if !collide(candidate) {
                        new_pos = candidate;
                    }
                }
            }
        }
    }

    // Y 方向
    if move_y {
        let snap_x = if cardinal_y { snap_to_col_center(new_pos) } else { 0.0 };
        let target_y = (new_pos.y + dy).clamp(pmin.y, pmax.y);
        let try_combo = Vec2::new((new_pos.x + snap_x).clamp(pmin.x, pmax.x), target_y);
        if !collide(try_combo) {
            new_pos = try_combo;
        } else {
            let try_y = Vec2::new(new_pos.x, target_y);
            if !collide(try_y) {
                new_pos = try_y;
            } else if cardinal_y {
                let (col, _) = world_to_tile(new_pos);
                let col_x = tile_center(col, 0).x;
                let diff = col_x - new_pos.x;
                if diff.abs() > 0.5 && diff.abs() < BM_TILE * 0.5 {
                    let step = diff.signum() * snap_step;
                    let nudged_x = if step.abs() > diff.abs() {
                        col_x
                    } else {
                        new_pos.x + step
                    };
                    let candidate = Vec2::new(nudged_x, target_y).clamp(pmin, pmax);
                    if !collide(candidate) {
                        new_pos = candidate;
                    }
                }
            }
        }
    }

    new_pos.clamp(pmin, pmax)
}
