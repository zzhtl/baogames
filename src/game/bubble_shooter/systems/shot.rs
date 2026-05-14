use bevy::prelude::*;
use rand::prelude::*;

use crate::game::model::{GameKind, GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::grid::{
    available_colors, cell_to_pos, cell_valid, cols_in_row, flood_same_color, floating_cells,
    grid_is_empty, snap_nearest_empty,
};
use super::super::resources::{BubbleAssets, BubbleStage};
use super::super::setup::{spawn_grid_bubble, spawn_loaded_bubble};

pub fn bubble_shot_update(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<BubbleStage>,
    assets: Res<BubbleAssets>,
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
        if finish_bubble(&mut session, &mut save, false) {
            save.store();
        }
        return;
    };
    let color_id = color.0;
    stage.grid[row as usize][col as usize] = Some(color_id);
    let placed_entity = spawn_grid_bubble(&mut commands, &assets, col, row, color_id, stage.descend);

    let (popped, fell) = resolve_pops(
        &mut commands,
        &mut stage,
        &grid_q,
        (col, row),
        color_id,
        placed_entity,
    );

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
        if finish_bubble(&mut session, &mut save, false) {
            save.store();
        }
        return;
    }

    advance_loaded(&mut commands, &assets, &mut stage);

    // 胜利判定
    if grid_is_empty(&stage.grid) && finish_bubble(&mut session, &mut save, true) {
        save.store();
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
/// `placed_entity` 是刚刚 spawn 的新泡泡实体；因为它还没进 `grid_q`，需要手动检查。
fn resolve_pops(
    commands: &mut Commands,
    stage: &mut BubbleStage,
    grid_q: &Query<(Entity, &BubbleCell), With<GridBubble>>,
    placed: (i32, i32),
    color_id: u8,
    placed_entity: Entity,
) -> (u32, u32) {
    let group = flood_same_color(placed, color_id, &stage.grid);
    if group.len() < 3 {
        return (0, 0);
    }
    for (gc, gr) in &group {
        stage.grid[*gr as usize][*gc as usize] = None;
    }
    let popped = group.len() as u32;
    // 已有实体（不含 placed_entity）
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
    // 新 spawn 的 placed_entity 不在 grid_q 里，单独处理
    if group.iter().any(|(gc, gr)| *gc == placed.0 && *gr == placed.1) {
        commands
            .entity(placed_entity)
            .remove::<GridBubble>()
            .insert(PoppingBubble { life: POP_LIFETIME });
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

pub(super) fn apply_score(session: &mut GameSession, popped: u32, fell: u32) {
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

pub(super) fn touches_dead_line(stage: &BubbleStage) -> bool {
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

fn advance_loaded(commands: &mut Commands, assets: &BubbleAssets, stage: &mut BubbleStage) {
    let mut rng = thread_rng();
    let avail = available_colors(&stage.grid);
    let new_current = stage.next;
    if avail.is_empty() {
        // 网格已空（即将判胜利），保持原状即可
        stage.current = new_current;
    } else {
        // 优先保留 next 的颜色作为新 current；若该颜色已被消光，从剩余可用色中随机
        if avail.iter().any(|c| *c == new_current) {
            stage.current = new_current;
        } else {
            stage.current = avail[rng.gen_range(0..avail.len())];
        }
        stage.next = avail[rng.gen_range(0..avail.len())];
    }
    stage.shot_active = false;

    spawn_loaded_bubble(commands, assets, stage.current, stage.aim);
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
    // 网格泡泡只有一个父实体，子实体（高光）跟随平移
    for (e, cell) in grid_q {
        let new_pos = cell_to_pos(cell.col, cell.row, stage.descend);
        commands.entity(e).insert(Transform::from_translation(
            new_pos.extend(Z_BUBBLE),
        ));
    }
}

/// 返回 true 表示 `save` 已被更新、需要持久化到磁盘。
/// IO 副作用从这里抽出便于单测，避免污染真实存档文件。
fn finish_bubble(session: &mut GameSession, save: &mut SaveData, won: bool) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = won;
    let idx = GameKind::BubbleBobble.index();
    if won {
        session.score += 1000;
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else {
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        session.status = "撞到死亡线…Enter 重试，Esc 返回".to_string();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::BubbleBobble,
            level: 1,
            score: 0,
            lives: 3,
            paused: false,
            finished: false,
            won: false,
            status: String::new(),
        }
    }

    fn fresh_stage() -> BubbleStage {
        BubbleStage {
            grid: vec![vec![None; COLS_EVEN as usize]; ROWS_MAX],
            descend: 0,
            aim: 0.0,
            current: 0,
            next: 0,
            shot_active: false,
            shots_left_for_descend: 12,
            max_shots_per_descend: 12,
            colors_count: 3,
            message: String::new(),
            message_clock: 0.0,
            flash_clock: 0.0,
        }
    }

    #[test]
    fn apply_score_triple_match_gives_150() {
        let mut s = fresh_session();
        apply_score(&mut s, 3, 0);
        assert_eq!(s.score, 150);
    }

    #[test]
    fn apply_score_no_pops_no_falls_keeps_score() {
        let mut s = fresh_session();
        s.score = 42;
        apply_score(&mut s, 0, 0);
        assert_eq!(s.score, 42);
    }

    #[test]
    fn apply_score_large_chain_gets_bonus() {
        // popped>=5 时每超过 4 的颗数额外 +30
        let mut s = fresh_session();
        apply_score(&mut s, 5, 0);
        // 5*50 + (5-4)*30
        assert_eq!(s.score, 280);

        let mut s = fresh_session();
        apply_score(&mut s, 7, 0);
        // 7*50 + 3*30
        assert_eq!(s.score, 440);
    }

    #[test]
    fn apply_score_falling_bubbles_score_100_each() {
        let mut s = fresh_session();
        apply_score(&mut s, 0, 4);
        assert_eq!(s.score, 400);
    }

    #[test]
    fn apply_score_combined_pops_and_falls() {
        let mut s = fresh_session();
        apply_score(&mut s, 3, 2);
        // 150 + 200
        assert_eq!(s.score, 350);
    }

    #[test]
    fn touches_dead_line_empty_grid_is_false() {
        let stage = fresh_stage();
        assert!(!touches_dead_line(&stage));
    }

    #[test]
    fn touches_dead_line_top_row_only_is_false() {
        let mut stage = fresh_stage();
        stage.grid[0][0] = Some(0);
        assert!(!touches_dead_line(&stage));
    }

    #[test]
    fn touches_dead_line_triggers_after_enough_descend() {
        let mut stage = fresh_stage();
        // 底部行有泡泡，且把 descend 推到足够大
        stage.grid[ROWS_MAX - 1][0] = Some(0);
        let mut crossed = false;
        for d in 0..40 {
            stage.descend = d;
            if touches_dead_line(&stage) {
                crossed = true;
                break;
            }
        }
        assert!(crossed, "随着 descend 增大，最底行必然越过死亡线");
    }

    #[test]
    fn finish_bubble_win_updates_save_and_returns_true() {
        let mut session = fresh_session();
        session.score = 200;
        let mut save = SaveData::default();
        let dirty = finish_bubble(&mut session, &mut save, true);
        assert!(dirty);
        assert!(session.finished);
        assert!(session.won);
        // 胜利会 +1000 奖励
        assert_eq!(session.score, 1200);
        assert_eq!(save.high_scores[GameKind::BubbleBobble.index()], 1200);
        assert!(save.unlocked_levels[GameKind::BubbleBobble.index()] >= 2);
    }

    #[test]
    fn finish_bubble_loss_records_high_score_but_does_not_unlock() {
        let mut session = fresh_session();
        session.score = 300;
        let mut save = SaveData::default();
        let level_before = save.unlocked_levels[GameKind::BubbleBobble.index()];
        let dirty = finish_bubble(&mut session, &mut save, false);
        assert!(dirty);
        assert!(session.finished);
        assert!(!session.won);
        assert_eq!(save.high_scores[GameKind::BubbleBobble.index()], 300);
        assert_eq!(save.unlocked_levels[GameKind::BubbleBobble.index()], level_before);
    }

    #[test]
    fn finish_bubble_idempotent_after_first_call() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let first = finish_bubble(&mut session, &mut save, true);
        let second = finish_bubble(&mut session, &mut save, false);
        assert!(first);
        // 第二次调用必须返回 false，避免被重复扣分或覆盖结果
        assert!(!second);
        assert!(session.won, "won 不应被后续调用翻转");
    }

    #[test]
    fn finish_bubble_caps_unlocked_level_at_ten() {
        let mut session = fresh_session();
        session.level = 10;
        let mut save = SaveData::default();
        finish_bubble(&mut session, &mut save, true);
        assert_eq!(save.unlocked_levels[GameKind::BubbleBobble.index()], 10);
    }
}
