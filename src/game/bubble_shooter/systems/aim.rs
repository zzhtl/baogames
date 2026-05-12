use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::AimDot;
use super::super::constants::*;
use super::super::grid::{aim_dir, cell_to_pos, cell_valid};
use super::super::resources::BubbleStage;

pub fn bubble_aim_dots_update(
    session: Res<GameSession>,
    stage: Res<BubbleStage>,
    mut dots: Query<(&AimDot, &mut Transform, &mut Sprite)>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    let visible = !stage.shot_active && !session.paused && !session.finished;
    let dir = aim_dir(stage.aim);
    let start = Vec2::new(CANNON_X, CANNON_Y) + dir * 36.0;
    let mut p = start;
    let mut v = dir;
    let step = 22.0;
    for i in 0..7 {
        // 简单步进 + 墙反射；遇到泡泡或顶部停在那个位置
        let mut traveled = 0.0;
        let segment = step;
        let mut hit = false;
        while traveled < segment {
            let remain = segment - traveled;
            let next_x = p.x + v.x * remain;
            if v.x < 0.0 && next_x - BUBBLE_R < PLAY_LEFT {
                let t = (PLAY_LEFT + BUBBLE_R - p.x) / v.x;
                p += v * t;
                v.x = -v.x;
                traveled += t;
                continue;
            }
            if v.x > 0.0 && next_x + BUBBLE_R > PLAY_RIGHT {
                let t = (PLAY_RIGHT - BUBBLE_R - p.x) / v.x;
                p += v * t;
                v.x = -v.x;
                traveled += t;
                continue;
            }
            p += v * remain;
            traveled = segment;
        }
        // 顶部检测
        let top_y = cell_to_pos(0, 0, stage.descend).y;
        if p.y >= top_y {
            hit = true;
        }
        // 网格碰撞检测
        if !hit {
            let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - p.y) / ROW_H - stage.descend as f32;
            let row0 = row_f.round() as i32;
            'outer: for r in (row0 - 1).max(0)..=(row0 + 1).min(ROWS_MAX as i32 - 1) {
                let offset = if r.rem_euclid(2) == 1 {
                    BUBBLE_R
                } else {
                    0.0
                };
                let col_f = (p.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
                let col0 = col_f.round() as i32;
                for c in (col0 - 1)..=(col0 + 1) {
                    if !cell_valid(c, r) {
                        continue;
                    }
                    if stage.grid[r as usize][c as usize].is_some() {
                        let center = cell_to_pos(c, r, stage.descend);
                        if (center - p).length_squared() < (BUBBLE_D - 1.0).powi(2) {
                            hit = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
        for (dot, mut t, mut sp) in &mut dots {
            if dot.idx == i {
                t.translation.x = p.x;
                t.translation.y = p.y;
                sp.color = if visible {
                    let alpha = if hit { 0.25 } else { 0.7 - i as f32 * 0.06 };
                    Color::srgba(0.92, 0.86, 1.0, alpha)
                } else {
                    Color::srgba(0.0, 0.0, 0.0, 0.0)
                };
            }
        }
        if hit {
            for (dot, mut t, mut sp) in &mut dots {
                if dot.idx > i {
                    t.translation.x = p.x;
                    t.translation.y = p.y;
                    sp.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                }
            }
            return;
        }
    }
}
