use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::{FONT_BODY, Z_TEXT};
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, pixel_font, rect};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::GridLayout;
use bevy::text::FontHinting;

use super::components::*;
use super::constants::*;
use super::logic::{manhattan_lower_bound, shuffle};
use super::palette;
use super::resources::SlidingStage;

const HUD_BORDER: Color = Color::srgb(0.82, 0.86, 0.34);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let size = size_for(level, age);
    let board = shuffle(size, shuffle_for(level, age), &mut rand::thread_rng());
    let layout = GridLayout::fit(size as i32, size as i32);
    let gap = (layout.cell_size * TILE_GAP_RATIO).max(BORDER);
    let tile = layout.cell_size - gap;

    // 棋盘底板：空格所在的凹槽要能看出来。
    let total = layout.total_size() + Vec2::splat(gap);
    let center = Vec2::new(
        layout.origin.x + (layout.cols - 1) as f32 * layout.cell_size * 0.5,
        layout.origin.y - (layout.rows - 1) as f32 * layout.cell_size * 0.5,
    );
    rect(commands, snap2(center), total, palette::BOARD_BACK, GameEntity);

    for (index, &value) in board.iter().enumerate() {
        if value == 0 {
            continue;
        }
        let (col, row) = ((index % size) as i32, (index / size) as i32);
        spawn_tile(commands, font, snap2(layout.cell_center(col, row)), tile, value);
    }
    spawn_hud(commands, font, hud_root);

    let total_time = time_for(level, age);
    commands.insert_resource(SlidingStage {
        size,
        layout,
        lower_bound: manhattan_lower_bound(&board, size),
        initial_board: board.clone(),
        board,
        moves: 0,
        history: Vec::new(),
        time_left: total_time,
        message: "方向键=块的去向".to_string(),
        message_clock: 2.6,
    });
}

fn spawn_tile(commands: &mut Commands, font: &UiFont, center: Vec2, tile: f32, value: u8) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(1.0)),
            Visibility::default(),
            SlideTile { value },
            GameEntity,
        ))
        .id();
    commands.spawn((
        Sprite::from_color(palette::TILE_EDGE, Vec2::splat(tile)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SlideTile { value },
        SlideTileEdge,
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Sprite::from_color(palette::TILE_FACE, Vec2::splat(tile - BORDER * 2.0)),
        Transform::from_xyz(0.0, 0.0, 0.1),
        SlideTile { value },
        SlideTileFace,
        GameEntity,
        ChildOf(parent),
    ));
    // 数字挂在**没有缩放**的父节点下，字号直接用实机值 —— 被 Transform 拉伸过的
    // Text2d 会糊（见 memory_match/setup.rs 的同类注释）。
    commands.spawn((
        Text2d::new(value.to_string()),
        pixel_font(font, FONT_BODY),
        FontHinting::Enabled,
        Anchor::CENTER,
        TextColor(palette::TILE_TEXT),
        Transform::from_xyz(0.0, 0.0, Z_TEXT),
        GameEntity,
        ChildOf(parent),
    ));
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "数字华容道",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SlidingHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, SlidingScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SlidingMessage,
    );
}
