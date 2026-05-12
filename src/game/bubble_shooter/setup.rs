use bevy::prelude::*;
use rand::prelude::*;

use crate::common::render::{UiFont, panel, rect, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::grid::{cell_to_pos, cols_in_row, level_idx};
use super::resources::BubbleStage;

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    paint_field(commands);
    paint_frame(commands);

    let li = level_idx(level);
    let colors_count = COLORS_BY_LEVEL[li];
    let initial_rows = INITIAL_ROWS_BY_LEVEL[li];
    let max_shots = SHOTS_TO_DESCEND_BY_LEVEL[li];

    let mut grid: Vec<Vec<Option<u8>>> = vec![vec![None; COLS_EVEN as usize]; ROWS_MAX];
    let mut rng = thread_rng();
    for r in 0..initial_rows {
        let cols = cols_in_row(r as i32);
        for c in 0..cols {
            grid[r][c as usize] = Some(rng.gen_range(0..colors_count));
        }
    }

    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if let Some(color_id) = grid[r][c as usize] {
                spawn_grid_bubble(commands, c, r as i32, color_id, 0);
            }
        }
    }

    paint_cannon(commands);

    let current = rng.gen_range(0..colors_count);
    let next = rng.gen_range(0..colors_count);

    spawn_loaded_bubble(commands, current);
    spawn_next_preview(commands, font, next);
    spawn_aim_dots(commands);
    spawn_hud(commands, font);

    commands.insert_resource(BubbleStage {
        grid,
        descend: 0,
        aim: 0.0,
        current,
        next,
        shot_active: false,
        shots_left_for_descend: max_shots,
        max_shots_per_descend: max_shots,
        colors_count,
        message: format!("第 {} 关 - 三连同色消除", level),
        message_clock: 2.4,
        flash_clock: 0.0,
    });
}

fn paint_field(commands: &mut Commands) {
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, 0.0),
        Vec2::new(PLAY_W, PLAY_H),
        Color::srgb(0.07, 0.06, 0.13),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.1, 0.08, 0.18),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM + 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.1, 0.08, 0.18),
        GameEntity,
    );
    // 顶部"天花板"
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 4.0),
        Vec2::new(PLAY_W, 6.0),
        Color::srgb(0.62, 0.34, 0.62),
        GameEntity,
    );
    // 死亡线
    for i in 0..16 {
        let x = PLAY_LEFT + 6.0 + i as f32 * (PLAY_W - 12.0) / 16.0 + 8.0;
        rect(
            commands,
            Vec2::new(x, DEAD_Y),
            Vec2::new(14.0, 2.0),
            Color::srgb(0.85, 0.32, 0.42),
            GameEntity,
        );
    }
}

fn paint_frame(commands: &mut Commands) {
    let frame_thickness = 6.0;
    let outer_w = PLAY_W + frame_thickness * 2.0;
    let outer_h = PLAY_H + frame_thickness * 2.0;
    let frame_color = Color::srgb(0.46, 0.32, 0.6);
    for (pos, size) in [
        (
            Vec2::new(PLAY_OFFSET_X, PLAY_TOP + frame_thickness * 0.5),
            Vec2::new(outer_w, frame_thickness),
        ),
        (
            Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM - frame_thickness * 0.5),
            Vec2::new(outer_w, frame_thickness),
        ),
        (
            Vec2::new(PLAY_LEFT - frame_thickness * 0.5, 0.0),
            Vec2::new(frame_thickness, outer_h),
        ),
        (
            Vec2::new(PLAY_RIGHT + frame_thickness * 0.5, 0.0),
            Vec2::new(frame_thickness, outer_h),
        ),
    ] {
        commands.spawn((
            Sprite::from_color(frame_color, size),
            Transform::from_translation(pos.extend(Z_FRAME)),
            GameEntity,
        ));
    }
}

fn paint_cannon(commands: &mut Commands) {
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 18.0),
        Vec2::new(64.0, 18.0),
        Color::srgb(0.28, 0.18, 0.32),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 18.0),
        Vec2::new(56.0, 10.0),
        Color::srgb(0.7, 0.46, 0.78),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X - 28.0, CANNON_Y - 8.0),
        Vec2::new(8.0, 12.0),
        Color::srgb(0.42, 0.28, 0.5),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X + 28.0, CANNON_Y - 8.0),
        Vec2::new(8.0, 12.0),
        Color::srgb(0.42, 0.28, 0.5),
        GameEntity,
    );

    // 炮管 (旋转的)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.62, 0.4, 0.7), Vec2::new(14.0, 44.0)),
        Transform {
            translation: Vec3::new(CANNON_X, CANNON_Y + 22.0, Z_CANNON),
            rotation: Quat::IDENTITY,
            ..default()
        },
        CannonBarrel,
        GameEntity,
    ));
}

fn spawn_loaded_bubble(commands: &mut Commands, current: u8) {
    commands.spawn((
        Sprite::from_color(palette(current), Vec2::splat(BUBBLE_D - 6.0)),
        Transform::from_translation(Vec3::new(CANNON_X, CANNON_Y + 6.0, Z_CANNON - 0.1)),
        LoadedBubble,
        BubbleColor(current),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgba(1.0, 1.0, 1.0, 0.45), Vec2::new(8.0, 8.0)),
        Transform::from_translation(Vec3::new(
            CANNON_X - 7.0,
            CANNON_Y + 12.0,
            Z_CANNON - 0.05,
        )),
        LoadedBubble,
        GameEntity,
    ));
}

fn spawn_next_preview(commands: &mut Commands, font: &UiFont, next: u8) {
    let preview_x = CANNON_X - 92.0;
    let preview_y = CANNON_Y - 4.0;
    panel(
        commands,
        Vec2::new(preview_x, preview_y),
        Vec2::new(56.0, 56.0),
        Color::srgb(0.12, 0.1, 0.18),
        Color::srgb(0.5, 0.34, 0.6),
        GameEntity,
    );
    text(
        commands,
        font,
        "下一",
        Vec2::new(preview_x, preview_y + 38.0),
        12.0,
        Color::srgb(0.86, 0.78, 0.96),
        GameEntity,
    );
    commands.spawn((
        Sprite::from_color(palette(next), Vec2::splat(BUBBLE_D - 8.0)),
        Transform::from_translation(Vec3::new(preview_x, preview_y, Z_CANNON - 0.1)),
        NextBubbleSprite,
        BubbleColor(next),
        GameEntity,
    ));
}

fn spawn_aim_dots(commands: &mut Commands) {
    for i in 0..7 {
        commands.spawn((
            Sprite::from_color(Color::srgba(0.92, 0.86, 1.0, 0.65), Vec2::splat(4.0)),
            Transform::from_translation(Vec3::new(
                CANNON_X,
                CANNON_Y + 50.0 + i as f32 * 18.0,
                Z_CANNON - 0.2,
            )),
            AimDot { idx: i },
            GameEntity,
        ));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont) {
    let hud_x = PLAY_RIGHT + 110.0;
    panel(
        commands,
        Vec2::new(hud_x, 180.0),
        Vec2::new(180.0, 230.0),
        Color::srgb(0.1, 0.08, 0.16),
        Color::srgb(0.7, 0.4, 0.78),
        GameEntity,
    );
    text(
        commands,
        font,
        "泡泡龙",
        Vec2::new(hud_x, 270.0),
        22.0,
        Color::srgb(1.0, 0.88, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        "P1\n← / → 或 A/D 瞄准\nSpace / J 发射\nEsc 暂停",
        Vec2::new(hud_x, 200.0),
        14.0,
        Color::srgb(0.92, 0.78, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        "",
        Vec2::new(hud_x, 110.0),
        16.0,
        Color::srgb(1.0, 0.94, 0.7),
        GameEntity,
    )
    .insert(BubbleHud);

    // 中央提示
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(24.0).with_font(font.0.clone()),
        TextColor(Color::srgb(1.0, 0.92, 0.5)),
        Transform::from_translation(Vec3::new(PLAY_OFFSET_X, PLAY_TOP - 56.0, Z_HUD)),
        BubbleMessage,
        GameEntity,
    ));
}

pub fn spawn_grid_bubble(
    commands: &mut Commands,
    col: i32,
    row: i32,
    color_id: u8,
    descend: usize,
) {
    let pos = cell_to_pos(col, row, descend);
    let body_color = palette(color_id);
    // 主体
    commands.spawn((
        Sprite::from_color(body_color, Vec2::splat(BUBBLE_D - 4.0)),
        Transform::from_translation(pos.extend(Z_BUBBLE)),
        GridBubble,
        BubbleCell { col, row },
        BubbleColor(color_id),
        GameEntity,
    ));
    // 暗色描边内圈
    commands.spawn((
        Sprite::from_color(palette_dark(color_id), Vec2::splat(BUBBLE_D - 14.0)),
        Transform::from_translation(Vec3::new(pos.x, pos.y - 5.0, Z_BUBBLE + 0.05)),
        GridBubble,
        BubbleCell { col, row },
        GameEntity,
    ));
    // 高光
    commands.spawn((
        Sprite::from_color(Color::srgba(1.0, 1.0, 1.0, 0.55), Vec2::new(7.0, 7.0)),
        Transform::from_translation(Vec3::new(pos.x - 7.0, pos.y + 6.0, Z_BUBBLE + 0.1)),
        GridBubble,
        BubbleCell { col, row },
        GameEntity,
    ));
}
