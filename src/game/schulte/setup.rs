use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::FONT_BODY;
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, text_anchored};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::{GridCursor, GridLayout};

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{SchulteMode, SchulteStage, build_board};

const HUD_BORDER: Color = Color::srgb(0.36, 0.86, 0.70);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let size = size_for(level, age);
    let mode = mode_for(level, age);
    let layout = GridLayout::fit(size, size);
    let (cells, order) = build_board(size, mode);

    let gap = (layout.cell_size * CELL_GAP_RATIO).max(BORDER);
    let tile = layout.cell_size - gap;
    for row in 0..size {
        for col in 0..size {
            let index = (row * size + col) as usize;
            let center = snap2(layout.cell_center(col, row));
            spawn_cell(commands, font, center, tile, index, cells[index], mode);
        }
    }

    spawn_cursor(commands, snap2(layout.cell_center(0, 0)), tile);
    spawn_hud(commands, font, hud_root);

    let total_time = time_for(level, age);
    commands.insert_resource(SchulteStage {
        layout,
        mode,
        cells,
        order,
        found: vec![false; (size * size) as usize],
        progress: 0,
        cursor: GridCursor::new(0, 0),
        time_left: total_time,
        initial_time: total_time,
        streak: 0,
        best_streak: 0,
        mistakes: 0,
        wrong_cell: None,
        wrong_clock: 0.0,
        message: mode.hint().to_string(),
        message_clock: 2.6,
    });
}

/// 一个格子 = 描边底板 + 内层填充 + 数字。三者都带 `SchulteCell`，
/// 状态变化时按下标直接查询改色，不用走父子层级。
fn spawn_cell(
    commands: &mut Commands,
    font: &UiFont,
    center: Vec2,
    tile: f32,
    index: usize,
    cell: (u32, u8),
    mode: SchulteMode,
) {
    commands.spawn((
        Sprite::from_color(palette::CELL_BORDER, Vec2::splat(tile)),
        Transform::from_translation(center.extend(0.0)),
        SchulteCell { index },
        SchulteCellBorder,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(palette::CELL_FILL, Vec2::splat(tile - BORDER * 2.0)),
        Transform::from_translation(center.extend(0.1)),
        SchulteCell { index },
        SchulteCellFill,
        GameEntity,
    ));
    text_anchored(
        commands,
        font,
        &cell.0.to_string(),
        center,
        FONT_BODY,
        number_color(cell.1, mode),
        Anchor::CENTER,
        GameEntity,
    )
    .insert((SchulteCell { index }, SchulteCellText));
}

pub fn number_color(color_id: u8, mode: SchulteMode) -> Color {
    match (mode, color_id) {
        (SchulteMode::DualColor, 0) => palette::NUM_WARM,
        (SchulteMode::DualColor, _) => palette::NUM_COOL,
        _ => palette::NUM_PLAIN,
    }
}

/// 光标画成四条边，中间留空 —— 整块半透明底会把数字压暗。
fn spawn_cursor(commands: &mut Commands, center: Vec2, tile: f32) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(5.0)),
            Visibility::default(),
            SchulteCursor,
            GameEntity,
        ))
        .id();
    let thickness = BORDER * 2.0;
    let half = tile * 0.5;
    for (dx, dy, w, h) in [
        (0.0, half, tile, thickness),
        (0.0, -half, tile, thickness),
        (-half, 0.0, thickness, tile),
        (half, 0.0, thickness, tile),
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
        commands, font, hud_root, "舒尔特方格",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SchulteHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, SchulteScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SchulteTarget,
    );
}
