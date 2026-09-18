use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::FONT_BODY;
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, rect, text_anchored};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::{GridCursor, GridLayout};

use super::components::*;
use super::constants::*;
use super::generator::{carve, conflicts, full_solution};
use super::palette;
use super::resources::SudokuStage;

const HUD_BORDER: Color = Color::srgb(0.46, 0.55, 0.95);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let spec = spec_for(level, age);
    let mut rng = rand::thread_rng();
    let (board, _) = carve(&full_solution(spec, &mut rng), spec, holes_for(level, age), &mut rng);
    let given: Vec<bool> = board.iter().map(|&value| value != 0).collect();

    let size = spec.size as i32;
    let layout = GridLayout::fit_in(size, size, BOARD_W, BOARD_H, BOARD_CENTER_Y);
    let cell = layout.cell_size;

    for row in 0..size {
        for col in 0..size {
            let index = spec.index(col as usize, row as usize);
            let center = snap2(layout.cell_center(col, row));
            rect(commands, center, Vec2::splat(cell), palette::CELL_LINE, GameEntity);
            commands.spawn((
                Sprite::from_color(
                    if given[index] { palette::GIVEN_FILL } else { palette::CELL_FILL },
                    Vec2::splat(cell - BORDER * 2.0),
                ),
                Transform::from_translation(center.extend(0.1)),
                SudokuCell { index },
                SudokuCellFill,
                GameEntity,
            ));
            text_anchored(
                commands, font,
                &digit_label(board[index]),
                center, FONT_BODY,
                if given[index] { palette::GIVEN_TEXT } else { palette::FILLED_TEXT },
                Anchor::CENTER, GameEntity,
            )
            .insert((SudokuCell { index }, SudokuCellText));
        }
    }
    spawn_box_lines(commands, &layout, spec.box_w as i32, spec.box_h as i32);
    spawn_cursor(commands, snap2(layout.cell_center(0, 0)), cell);
    spawn_hud(commands, font, hud_root);

    let total_time = time_for(level, age);
    let mut stage = SudokuStage {
        spec,
        layout,
        conflicts: conflicts(&board, spec),
        initial_board: board.clone(),
        board,
        given,
        cursor: GridCursor::new(0, 0),
        time_left: total_time,
        initial_time: total_time,
        mistakes: 0,
        message: "动作一+1 动作二-1".to_string(),
        message_clock: 2.6,
    };
    stage.refresh_conflicts();
    commands.insert_resource(stage);
}

pub fn digit_label(value: u8) -> String {
    if value == 0 {
        String::new()
    } else {
        value.to_string()
    }
}

/// 宫线画在格线之上：9×9 只有 16 像素格，不加粗分不出宫。
fn spawn_box_lines(commands: &mut Commands, layout: &GridLayout, box_w: i32, box_h: i32) {
    let total = layout.total_size();
    let center = Vec2::new(
        layout.origin.x + (layout.cols - 1) as f32 * layout.cell_size * 0.5,
        layout.origin.y - (layout.rows - 1) as f32 * layout.cell_size * 0.5,
    );
    let thickness = BORDER;
    let mut line = |pos: Vec2, size: Vec2| {
        commands.spawn((
            Sprite::from_color(palette::BOX_LINE, size),
            Transform::from_translation(snap2(pos).extend(0.4)),
            GameEntity,
        ));
    };
    for col in (0..=layout.cols).step_by(box_w.max(1) as usize) {
        let x = layout.origin.x - layout.cell_size * 0.5 + col as f32 * layout.cell_size;
        line(Vec2::new(x, center.y), Vec2::new(thickness, total.y + thickness));
    }
    for row in (0..=layout.rows).step_by(box_h.max(1) as usize) {
        let y = layout.origin.y + layout.cell_size * 0.5 - row as f32 * layout.cell_size;
        line(Vec2::new(center.x, y), Vec2::new(total.x + thickness, thickness));
    }
}

fn spawn_cursor(commands: &mut Commands, center: Vec2, cell: f32) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(5.0)),
            Visibility::default(),
            SudokuCursor,
            GameEntity,
        ))
        .id();
    let thickness = BORDER;
    let half = cell * 0.5;
    for (dx, dy, w, h) in [
        (0.0, half, cell, thickness),
        (0.0, -half, cell, thickness),
        (-half, 0.0, thickness, cell),
        (half, 0.0, thickness, cell),
    ] {
        commands.spawn((
            Sprite::from_color(palette::CURSOR, Vec2::new(w, h)),
            Transform::from_xyz(dx, dy, 0.0),
            GameEntity,
            ChildOf(parent),
        ));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "儿童数独",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SudokuHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, SudokuScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SudokuMessage,
    );
}
