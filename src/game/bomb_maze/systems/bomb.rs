use bevy::prelude::*;

use crate::common::render::UiFont;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{BM_COLS, BM_ROWS};
use super::super::geometry::tile_center;
use super::super::setup::{spawn_bm_exit, spawn_bm_flame, spawn_bm_powerup};

pub fn bm_bomb_tick(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bombs: Query<(Entity, &mut BMBomb, &BMTilePos), With<BMBomb>>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<(Entity, &BMTilePos, &BMSoftWall)>,
    mut players: Query<&mut BMPlayer>,
    font: Res<UiFont>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }

    // 网格信息
    let mut hard = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            hard[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut soft_at: std::collections::HashMap<(i32, i32), (Entity, HiddenItem)> =
        std::collections::HashMap::new();
    for (e, tp, sw) in &soft_walls {
        soft_at.insert((tp.col, tp.row), (e, sw.hides));
    }
    let mut bomb_at: std::collections::HashMap<(i32, i32), Entity> =
        std::collections::HashMap::new();
    for (e, _, tp) in bombs.iter() {
        bomb_at.insert((tp.col, tp.row), e);
    }

    // 第一步：tick 计时 / 标记将要爆炸的炸弹
    let mut to_explode: Vec<Entity> = Vec::new();
    for (entity, mut bomb, _) in &mut bombs {
        bomb.fuse.tick(time.delta());
        if bomb.triggered || (!bomb.remote && bomb.fuse.just_finished()) {
            to_explode.push(entity);
        }
    }
    if to_explode.is_empty() {
        return;
    }

    // BFS 处理连环引爆
    let mut queue: std::collections::VecDeque<Entity> = to_explode.iter().copied().collect();
    let mut exploded: std::collections::HashSet<Entity> = std::collections::HashSet::new();
    let mut flame_cells: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut destroyed_softs: std::collections::HashSet<Entity> = std::collections::HashSet::new();
    let mut bomb_owners: Vec<Option<Entity>> = Vec::new();

    while let Some(entity) = queue.pop_front() {
        if !exploded.insert(entity) {
            continue;
        }
        let Ok((_, bomb, tp)) = bombs.get(entity) else {
            continue;
        };
        bomb_owners.push(bomb.owner);
        let (cx, cy) = (tp.col, tp.row);
        flame_cells.insert((cx, cy));
        for d in Dir4::all() {
            let dv = d.vec();
            for step in 1..=bomb.range {
                let nc = cx + dv.x as i32 * step;
                let nr = cy + dv.y as i32 * step;
                if !(0..BM_COLS).contains(&nc) || !(0..BM_ROWS).contains(&nr) {
                    break;
                }
                if hard[nc as usize][nr as usize] {
                    break;
                }
                flame_cells.insert((nc, nr));
                if let Some((soft_e, _)) = soft_at.get(&(nc, nr)) {
                    destroyed_softs.insert(*soft_e);
                    break; // 软砖吸火焰一格
                }
                if let Some(bomb_e) = bomb_at.get(&(nc, nr)) {
                    if !exploded.contains(bomb_e) {
                        queue.push_back(*bomb_e);
                    }
                    // 火焰会穿过被引爆的炸弹？经典版会停在炸弹格，这里也停
                    break;
                }
            }
        }
    }

    // 还原炸弹计数 + despawn 已爆炸炸弹
    for owner in bomb_owners.into_iter().flatten() {
        if let Ok(mut p) = players.get_mut(owner) {
            p.bombs_alive = p.bombs_alive.saturating_sub(1);
        }
    }
    for e in &exploded {
        commands.entity(*e).despawn();
    }

    // 软砖被毁：露出底下藏的东西
    for soft_e in &destroyed_softs {
        if let Some(((c, r), (_, hides))) = soft_at.iter().find(|(_, (e, _))| e == soft_e) {
            commands.entity(*soft_e).despawn();
            match hides {
                HiddenItem::Powerup(kind) => {
                    spawn_bm_powerup(&mut commands, *c, *r, *kind, &font);
                }
                HiddenItem::Exit => {
                    spawn_bm_exit(&mut commands, *c, *r);
                }
                HiddenItem::Nothing => {}
            }
        }
    }

    // 火焰
    for (c, r) in flame_cells {
        let pos = tile_center(c, r);
        spawn_bm_flame(&mut commands, pos, Color::srgb(1.0, 0.55, 0.18));
    }
}

pub fn bm_flame_tick(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut flames: Query<(Entity, &mut BMFlame, &mut Sprite)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    for (entity, mut flame, mut sprite) in &mut flames {
        flame.timer.tick(time.delta());
        let t = flame.timer.fraction();
        let alpha = (1.0 - t).clamp(0.0, 1.0);
        let mut c = Color::srgba(1.0, 0.55, 0.18, alpha);
        if t > 0.5 {
            c = Color::srgba(1.0, 0.85, 0.32, alpha);
        }
        sprite.color = c;
        if flame.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
