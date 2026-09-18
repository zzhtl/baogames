use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::FONT_BODY;
use crate::common::px::{px, snap2};
use crate::common::render::UiFont;
use crate::common::theme::{ACCENT, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::GridLayout;

use super::components::*;
use super::constants::*;
use super::generator::{cell_to_tile, generate, pick_key_tile, tile_dims};
use super::palette;
use super::resources::MazeStage;

const HUD_BORDER: Color = Color::srgb(0.45, 0.82, 0.38);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let (cells_w, cells_h) = cells_for(level, age);
    let (w, h) = tile_dims(cells_w, cells_h);
    let walls = generate(cells_w, cells_h, &mut rand::thread_rng());
    let start = cell_to_tile(0, 0);
    let exit = cell_to_tile(cells_w - 1, cells_h - 1);
    let key = needs_key(level)
        .then(|| pick_key_tile(&walls, w, h, start, exit))
        .flatten();

    let layout = GridLayout::fit(w as i32, h as i32);
    let cell = layout.cell_size;

    for row in 0..h as i32 {
        for col in 0..w as i32 {
            let center = snap2(layout.cell_center(col, row));
            let wall = walls[(row as usize) * w + col as usize];
            let mut entity = commands.spawn((
                Sprite::from_color(
                    if wall { palette::WALL } else { palette::FLOOR },
                    Vec2::splat(cell),
                ),
                Transform::from_translation(center.extend(0.0)),
                MazeTile { col, row },
                GameEntity,
            ));
            if !wall {
                entity.insert(MazeFloor);
            }
        }
    }

    spawn_marker(commands, &layout, exit, cell * 0.6, 0.4, palette::EXIT_LOCKED, MazeExit);
    if let Some(key_tile) = key {
        spawn_marker(commands, &layout, key_tile, cell * 0.45, 0.4, palette::KEY, MazeKey);
    }
    spawn_player(commands, &layout, start, cell);
    spawn_hud(commands, font, hud_root);

    let total_time = time_for(level, age);
    let mut visited = vec![false; w * h];
    visited[start.1 * w + start.0] = true;
    commands.insert_resource(MazeStage {
        layout,
        cols: w as i32,
        rows: h as i32,
        walls,
        visited,
        player: (start.0 as i32, start.1 as i32),
        start: (start.0 as i32, start.1 as i32),
        exit: (exit.0 as i32, exit.1 as i32),
        key: key.map(|(x, y)| (x as i32, y as i32)),
        has_key: false,
        fog_radius: fog_radius(level, age),
        steps: 0,
        backtracks: 0,
        move_cd: 0.0,
        time_left: total_time,
        message: if needs_key(level) {
            "先找钥匙再出口".to_string()
        } else {
            "走到右下角出口".to_string()
        },
        message_clock: 2.4,
    });
}

fn spawn_marker<M: Component>(
    commands: &mut Commands,
    layout: &GridLayout,
    tile: (usize, usize),
    size: f32,
    z: f32,
    color: Color,
    marker: M,
) {
    let center = snap2(layout.cell_center(tile.0 as i32, tile.1 as i32));
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(size)),
        Transform::from_translation(center.extend(z)),
        MazeTile { col: tile.0 as i32, row: tile.1 as i32 },
        marker,
        GameEntity,
    ));
}

/// 玩家画成两层方块：外圈暗、内芯亮，8 像素格子下也分得出朝向感。
fn spawn_player(commands: &mut Commands, layout: &GridLayout, tile: (usize, usize), cell: f32) {
    let center = snap2(layout.cell_center(tile.0 as i32, tile.1 as i32));
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(1.0)),
            Visibility::default(),
            MazePlayer,
            GameEntity,
        ))
        .id();
    commands.spawn((
        Sprite::from_color(palette::PLAYER_DARK, Vec2::splat(cell * 0.75)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Sprite::from_color(palette::PLAYER, Vec2::splat(cell * 0.45)),
        Transform::from_xyz(0.0, 0.0, 0.1),
        GameEntity,
        ChildOf(parent),
    ));
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "迷宫探险",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, MazeHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, MazeScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, MazeMessage,
    );
}
