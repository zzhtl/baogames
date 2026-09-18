use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::FONT_BODY;
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, rect};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;
use crate::game::puzzle::grid::{GridCursor, GridLayout};

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::SpotDiffStage;
use super::scene::{Motif, apply_differences, build_scene, diff_indices};

const HUD_BORDER: Color = Color::srgb(0.99, 0.56, 0.38);

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let (cols, rows) = grid_for(level, age);
    let shapes = palette::MOTIF_SHAPES.len();
    let colors = palette::MOTIF_COLORS.len();
    let mut rng = rand::thread_rng();
    let left = build_scene((cols * rows) as usize, shapes, colors, &mut rng);
    let (right, _) = apply_differences(
        &left,
        diffs_for(level, age),
        subtle_for(level, age),
        shapes,
        colors,
        &mut rng,
    );
    // 以实际差异为准：万一哪次改动被同值覆盖，进度条也不会对不上。
    let diffs = diff_indices(&left, &right);

    let layouts = panel_layouts(cols, rows);
    for (side, layout) in layouts.iter().enumerate() {
        spawn_panel_frame(commands, layout);
        let scene = if side == 0 { &left } else { &right };
        for row in 0..rows {
            for col in 0..cols {
                let index = (row * cols + col) as usize;
                let center = snap2(layout.cell_center(col, row));
                spawn_motif(commands, center, layout.cell_size, scene[index]);
                spawn_found_mark(commands, center, layout.cell_size, index);
            }
        }
        spawn_cursor(commands, snap2(layout.cell_center(0, 0)), layout.cell_size, side);
    }
    spawn_hud(commands, font, hud_root);

    commands.insert_resource(SpotDiffStage {
        layouts,
        cols,
        rows,
        found: vec![false; diffs.len()],
        diffs,
        cursor: GridCursor::new(0, 0),
        mistakes: 0,
        time_left: time_for(level, age),
        message: "找出不同的格子".to_string(),
        message_clock: 2.4,
    });
}

/// 左右两张图共用同一套自适应格宽，再各自平移到半屏。
pub fn panel_layouts(cols: i32, rows: i32) -> [GridLayout; 2] {
    let base = GridLayout::fit_in(cols, rows, PANEL_W, PANEL_H, PANEL_CENTER_Y);
    // 格宽已经对齐到偶数画布像素，中缝取偶数像素，平移量就还是整像素。
    let offset = (base.total_size().x + PANEL_GAP) * 0.5;
    [
        base.shifted(Vec2::new(-offset, 0.0)),
        base.shifted(Vec2::new(offset, 0.0)),
    ]
}

fn spawn_panel_frame(commands: &mut Commands, layout: &GridLayout) {
    let size = layout.total_size() + Vec2::splat(BORDER * 4.0);
    let center = Vec2::new(
        layout.origin.x + (layout.cols - 1) as f32 * layout.cell_size * 0.5,
        layout.origin.y - (layout.rows - 1) as f32 * layout.cell_size * 0.5,
    );
    rect(commands, snap2(center), size, palette::PANEL_EDGE, GameEntity);
    rect(
        commands,
        snap2(center),
        size - Vec2::splat(BORDER * 2.0),
        palette::PANEL_BACK,
        GameEntity,
    );
}

/// 图元在开局就画定，整局不再变 —— 两张图本身是静态的。
fn spawn_motif(commands: &mut Commands, center: Vec2, cell: f32, motif: Motif) {
    let color = motif_color(motif);
    let shape = palette::MOTIF_SHAPES[motif.shape as usize % palette::MOTIF_SHAPES.len()];
    for (dx, dy, w, h) in shape.iter().copied() {
        commands.spawn((
            Sprite::from_color(color, Vec2::new(w * cell, h * cell)),
            Transform::from_translation(
                snap2(center + Vec2::new(dx * cell, dy * cell)).extend(0.2),
            ),
            GameEntity,
        ));
    }
}

pub fn motif_color(motif: Motif) -> Color {
    let base = palette::MOTIF_COLORS[motif.color as usize % palette::MOTIF_COLORS.len()];
    let scale = palette::SHADE_SCALE[motif.shade as usize % palette::SHADE_SCALE.len()];
    let linear = base.to_linear();
    Color::linear_rgb(linear.red * scale, linear.green * scale, linear.blue * scale)
}

/// 找到之后亮起来的方框，平时藏着。
fn spawn_found_mark(commands: &mut Commands, center: Vec2, cell: f32, index: usize) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(3.0)),
            Visibility::Hidden,
            FoundMark { index },
            GameEntity,
        ))
        .id();
    frame_bars(commands, parent, cell * 0.9, BORDER, palette::FOUND);
}

fn spawn_cursor(commands: &mut Commands, center: Vec2, cell: f32, side: usize) {
    let parent = commands
        .spawn((
            Transform::from_translation(center.extend(5.0)),
            Visibility::default(),
            SpotCursor { side },
            GameEntity,
        ))
        .id();
    frame_bars(commands, parent, cell, BORDER, palette::CURSOR);
}

/// 四条边框，中间留空 —— 整块半透明底会把图元压暗，反而更难比对。
fn frame_bars(commands: &mut Commands, parent: Entity, size: f32, thickness: f32, color: Color) {
    let half = size * 0.5;
    for (dx, dy, w, h) in [
        (0.0, half, size, thickness),
        (0.0, -half, size, thickness),
        (-half, 0.0, thickness, size),
        (half, 0.0, thickness, size),
    ] {
        commands.spawn((
            Sprite::from_color(color, Vec2::new(w, h)),
            Transform::from_xyz(dx, dy, 0.0),
            GameEntity,
            ChildOf(parent),
        ));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "找不同",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SpotDiffHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, SpotDiffScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SpotDiffMessage,
    );
}
