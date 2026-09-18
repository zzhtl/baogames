use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::FontHinting;

use crate::common::constants::{FONT_BODY, Z_TEXT};
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, pixel_font};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::{GridCursor, GridLayout};

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{LinkupStage, build_tiles};

const HUD_BORDER: Color = Color::srgb(0.36, 0.76, 0.96);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let (cols, rows) = grid_for(level, age);
    let tiles = build_tiles(cols, rows, patterns_for(level, age), &mut rand::thread_rng());
    let layout = GridLayout::fit(cols, rows);
    let gap = (layout.cell_size * TILE_GAP_RATIO).max(BORDER);
    let tile = layout.cell_size - gap;

    for row in 0..rows {
        for col in 0..cols {
            let index = (row * cols + col) as usize;
            spawn_tile(commands, font, snap2(layout.cell_center(col, row)), tile, index, tiles[index]);
        }
    }
    spawn_cursor(commands, snap2(layout.cell_center(0, 0)), tile);
    spawn_hud(commands, font, hud_root);

    let pairs_left = tiles.iter().filter(|tile| tile.is_some()).count() / 2;
    commands.insert_resource(LinkupStage {
        layout,
        cols,
        rows,
        tiles,
        gravity: gravity_for(level, age),
        cursor: GridCursor::new(0, 0),
        selected: None,
        path: Vec::new(),
        path_clock: 0.0,
        pairs_left,
        streak: 0,
        best_streak: 0,
        shuffles: 0,
        time_left: time_for(level, age),
        message: "拐弯不超过两次".to_string(),
        message_clock: 2.4,
    });
}

fn spawn_tile(
    commands: &mut Commands,
    font: &UiFont,
    center: Vec2,
    tile: f32,
    index: usize,
    pattern: Option<u8>,
) {
    commands.spawn((
        Sprite::from_color(palette::TILE_EDGE, Vec2::splat(tile)),
        Transform::from_translation(center.extend(0.0)),
        LinkTile { index },
        LinkTileEdge,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(palette::TILE_FACE, Vec2::splat(tile - BORDER * 2.0)),
        Transform::from_translation(center.extend(0.1)),
        LinkTile { index },
        LinkTileFace,
        GameEntity,
    ));
    commands.spawn((
        Text2d::new(pattern_char(pattern)),
        pixel_font(font, FONT_BODY),
        FontHinting::Enabled,
        Anchor::CENTER,
        TextColor(pattern_color(pattern)),
        Transform::from_translation(center.extend(Z_TEXT)),
        LinkTile { index },
        LinkTileText,
        GameEntity,
    ));
}

pub fn pattern_char(pattern: Option<u8>) -> String {
    pattern
        .map(|id| palette::PATTERN_CHARS[id as usize % palette::PATTERN_CHARS.len()].to_string())
        .unwrap_or_default()
}

pub fn pattern_color(pattern: Option<u8>) -> Color {
    pattern
        .map(|id| palette::PATTERN_COLORS[id as usize % palette::PATTERN_COLORS.len()])
        .unwrap_or(Color::NONE)
}

fn spawn_cursor(commands: &mut Commands, center: Vec2, tile: f32) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(6.0)),
            Visibility::default(),
            LinkCursor,
            GameEntity,
        ))
        .id();
    let thickness = BORDER;
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
        commands, font, hud_root, "连连看",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, LinkupHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, LinkupScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, LinkupMessage,
    );
}
