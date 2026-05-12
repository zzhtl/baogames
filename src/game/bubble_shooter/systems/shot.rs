use bevy::prelude::*;
use rand::prelude::*;

use crate::game::model::{GameEntity, GameKind, GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::grid::{
    aim_dir, available_colors, cell_to_pos, cell_valid, cols_in_row, flood_same_color,
    floating_cells, grid_is_empty, snap_nearest_empty,
};
use super::super::resources::BubbleStage;
use super::super::setup::spawn_grid_bubble;

pub fn bubble_shot_update(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<BubbleStage>,
    mut shot_q: Query<(Entity, &mut Transform, &mut FlyingBubble, &BubbleColor)>,
    grid_q: Query<(Entity, &BubbleCell), With<GridBubble>>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();

    let Ok((shot_e, mut tr, mut fly, color)) = shot_q.single_mut() else {
        return;
    };

    let settled = step_flying_bubble(&mut tr, &mut fly, dt, &stage);

    let Some(pos) = settled else { return };
    commands.entity(shot_e).despawn();

    let placed = snap_nearest_empty(pos, stage.descend, &stage.grid);
    let Some((col, row)) = placed else {
        // 没找到可用位置（可能溢出），强制结束
        stage.shot_active = false;
        finish_bubble(&mut session, &mut save, false);
        return;
    };
    let color_id = color.0;
    stage.grid[row as usize][col as usize] = Some(color_id);
    spawn_grid_bubble(&mut commands, col, row, color_id, stage.descend);

    let (popped, fell) = resolve_pops(&mut commands, &mut stage, &grid_q, (col, row), color_id);

    apply_score(&mut session, popped, fell);

    if popped == 0 {
        stage.shots_left_for_descend -= 1;
        if stage.shots_left_for_descend <= 0 {
            descend_grid(&mut commands, &mut stage, &grid_q);
        }
    } else {
        stage.shots_left_for_descend = stage.max_shots_per_descend;
    }

    // 检查死亡线
    if !grid_is_empty(&stage.grid) && touches_dead_line(&stage) {
        stage.shot_active = false;
        finish_bubble(&mut session, &mut save, false);
        return;
    }

    advance_loaded(&mut commands, &mut stage);

    // 胜利判定
    if grid_is_empty(&stage.grid) {
        finish_bubble(&mut session, &mut save, true);
    }
}

/// 飞行子弹的步进/反射/碰撞。返回落点（撞墙/撞泡/撞顶）。None 表示尚在飞行。
fn step_flying_bubble(
    tr: &mut Transform,
    fly: &mut FlyingBubble,
    dt: f32,
    stage: &BubbleStage,
) -> Option<Vec2> {
    let mut remaining = dt;
    let max_step = BUBBLE_R * 0.6;
    let mut settled: Option<Vec2> = None;
    while remaining > 0.0 {
        let speed = fly.vel.length().max(1.0);
        let step_dt = (max_step / speed).min(remaining);
        let next = Vec2::new(
            tr.translation.x + fly.vel.x * step_dt,
            tr.translation.y + fly.vel.y * step_dt,
        );
        let mut new_pos = next;
        if new_pos.x - BUBBLE_R < PLAY_LEFT {
            new_pos.x = PLAY_LEFT + BUBBLE_R;
            fly.vel.x = fly.vel.x.abs();
        } else if new_pos.x + BUBBLE_R > PLAY_RIGHT {
            new_pos.x = PLAY_RIGHT - BUBBLE_R;
            fly.vel.x = -fly.vel.x.abs();
        }
        let top_y = cell_to_pos(0, 0, stage.descend).y;
        if new_pos.y >= top_y {
            settled = Some(Vec2::new(new_pos.x, top_y));
            tr.translation.x = new_pos.x;
            tr.translation.y = top_y;
            break;
        }
        let mut hit = false;
        let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - new_pos.y) / ROW_H - stage.descend as f32;
        let row0 = row_f.round() as i32;
        'outer: for r in (row0 - 1).max(0)..=(row0 + 1).min(ROWS_MAX as i32 - 1) {
            let offset = if r.rem_euclid(2) == 1 {
                BUBBLE_R
            } else {
                0.0
            };
            let col_f = (new_pos.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
            let col0 = col_f.round() as i32;
            for c in (col0 - 1)..=(col0 + 1) {
                if !cell_valid(c, r) {
                    continue;
                }
                if stage.grid[r as usize][c as usize].is_some() {
                    let center = cell_to_pos(c, r, stage.descend);
                    if (center - new_pos).length_squared() < (BUBBLE_D - 1.0).powi(2) {
                        hit = true;
                        break 'outer;
                    }
                }
            }
        }
        tr.translation.x = new_pos.x;
        tr.translation.y = new_pos.y;
        if hit {
            settled = Some(new_pos);
            break;
        }
        remaining -= step_dt;
    }
    settled
}

/// 处理同色聚集 + 浮空脱落。返回 (popped, fell) 数量。
fn resolve_pops(
    commands: &mut Commands,
    stage: &mut BubbleStage,
    grid_q: &Query<(Entity, &BubbleCell), With<GridBubble>>,
    placed: (i32, i32),
    color_id: u8,
) -> (u32, u32) {
    let group = flood_same_color(placed, color_id, &stage.grid);
    if group.len() < 3 {
        return (0, 0);
    }
    for (gc, gr) in &group {
        stage.grid[*gr as usize][*gc as usize] = None;
    }
    let popped = group.len() as u32;
    for (e, cell) in grid_q {
        if group
            .iter()
            .any(|(gc, gr)| *gc == cell.col && *gr == cell.row)
        {
            commands
                .entity(e)
                .remove::<GridBubble>()
                .insert(PoppingBubble { life: POP_LIFETIME });
        }
    }
    // 检查浮空泡泡
    let floating = floating_cells(&stage.grid);
    for (fc, fr) in &floating {
        stage.grid[*fr as usize][*fc as usize] = None;
    }
    let fell = floating.len() as u32;
    for (e, cell) in grid_q {
        if floating
            .iter()
            .any(|(fc, fr)| *fc == cell.col && *fr == cell.row)
        {
            commands
                .entity(e)
                .remove::<GridBubble>()
                .insert(FallingBubble { vy: 30.0 });
        }
    }
    (popped, fell)
}

fn apply_score(session: &mut GameSession, popped: u32, fell: u32) {
    if popped > 0 {
        session.score += popped * 50;
        if popped >= 5 {
            session.score += (popped - 4) * 30;
        }
    }
    if fell > 0 {
        session.score += fell * 100;
    }
}

fn touches_dead_line(stage: &BubbleStage) -> bool {
    for r in (0..ROWS_MAX).rev() {
        for c in 0..cols_in_row(r as i32) {
            if stage.grid[r][c as usize].is_some() {
                let p = cell_to_pos(c, r as i32, stage.descend);
                if p.y - BUBBLE_R <= DEAD_Y {
                    return true;
                }
            }
        }
    }
    false
}

fn advance_loaded(commands: &mut Commands, stage: &mut BubbleStage) {
    let mut rng = thread_rng();
    let new_current = stage.next;
    let avail = available_colors(&stage.grid);
    let new_next = if avail.is_empty() {
        stage.next
    } else {
        let cur_ok = avail.iter().any(|c| *c == new_current);
        let cur = if cur_ok {
            new_current
        } else {
            avail[rng.gen_range(0..avail.len())]
        };
        stage.current = cur;
        avail[rng.gen_range(0..avail.len())]
    };
    if avail.is_empty() {
        stage.current = new_current;
    }
    stage.next = new_next;
    stage.shot_active = false;

    let dir = aim_dir(stage.aim);
    let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + dir * 6.0;
    commands.spawn((
        Sprite::from_color(palette(stage.current), Vec2::splat(BUBBLE_D - 6.0)),
        Transform::from_translation(loaded_pos.extend(Z_CANNON - 0.1)),
        LoadedBubble,
        BubbleColor(stage.current),
        GameEntity,
    ));
    let highlight_local = dir.perp() * -7.0 + dir * 6.0;
    commands.spawn((
        Sprite::from_color(Color::srgba(1.0, 1.0, 1.0, 0.45), Vec2::new(8.0, 8.0)),
        Transform::from_translation((loaded_pos + highlight_local).extend(Z_CANNON - 0.05)),
        LoadedBubble,
        GameEntity,
    ));
}

fn descend_grid(
    commands: &mut Commands,
    stage: &mut BubbleStage,
    grid_q: &Query<(Entity, &BubbleCell), With<GridBubble>>,
) {
    stage.descend += 1;
    stage.shots_left_for_descend = stage.max_shots_per_descend;
    stage.message = "天花板下移！".to_string();
    stage.message_clock = 1.4;
    stage.flash_clock = 0.4;
    // 所有网格泡泡精灵下移 ROW_H
    for (e, cell) in grid_q {
        let new_pos = cell_to_pos(cell.col, cell.row, stage.descend);
        commands.entity(e).insert(Transform::from_translation(
            new_pos.extend(Z_BUBBLE),
        ));
    }
}

fn finish_bubble(session: &mut GameSession, save: &mut SaveData, won: bool) {
    if session.finished {
        return;
    }
    session.finished = true;
    session.won = won;
    let idx = GameKind::BubbleBobble.index();
    if won {
        session.score += 1000;
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else {
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.store();
        session.status = "撞到死亡线…Enter 重试，Esc 返回".to_string();
    }
}
