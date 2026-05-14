use bevy::prelude::*;
use rand::prelude::*;

use crate::common::render::{UiFont, panel, rect, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::grid::{cell_to_pos, cols_in_row, level_idx};
use super::resources::{BubbleAssets, BubbleStage};

pub fn setup_stage(commands: &mut Commands, assets: &BubbleAssets, font: &UiFont, level: u8) {
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
                spawn_grid_bubble(commands, assets, c, r as i32, color_id, 0);
            }
        }
    }

    paint_cannon(commands);

    let current = rng.gen_range(0..colors_count);
    let next = rng.gen_range(0..colors_count);

    spawn_loaded_bubble(commands, assets, current, 0.0);
    spawn_next_preview(commands, assets, font, next);
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
    // 主背景：柔和的奶油糖果色
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, 0.0),
        Vec2::new(PLAY_W, PLAY_H),
        Color::srgb(0.98, 0.94, 0.88),
        GameEntity,
    );
    // 顶部装饰条
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.85, 0.92, 1.0),
        GameEntity,
    );
    // 底部装饰条（草地色，让画面活泼一点）
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM + 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.78, 0.92, 0.78),
        GameEntity,
    );
    // 顶部"天花板"：更柔的棕红
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 4.0),
        Vec2::new(PLAY_W, 8.0),
        Color::srgb(0.86, 0.52, 0.72),
        GameEntity,
    );
    // 死亡线：警示红虚线
    for i in 0..16 {
        let x = PLAY_LEFT + 6.0 + i as f32 * (PLAY_W - 12.0) / 16.0 + 8.0;
        rect(
            commands,
            Vec2::new(x, DEAD_Y),
            Vec2::new(14.0, 3.0),
            Color::srgb(0.96, 0.42, 0.45),
            GameEntity,
        );
    }
}

fn paint_frame(commands: &mut Commands) {
    let frame_thickness = 8.0;
    let outer_w = PLAY_W + frame_thickness * 2.0;
    let outer_h = PLAY_H + frame_thickness * 2.0;
    let frame_color = Color::srgb(0.96, 0.66, 0.84);
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
    // 圆滚滚的炮台底座
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 20.0),
        Vec2::new(72.0, 22.0),
        Color::srgb(0.6, 0.4, 0.8),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 20.0),
        Vec2::new(64.0, 14.0),
        Color::srgb(0.92, 0.78, 1.0),
        GameEntity,
    );
    // 两个圆轮子
    rect(
        commands,
        Vec2::new(CANNON_X - 32.0, CANNON_Y - 10.0),
        Vec2::new(10.0, 14.0),
        Color::srgb(0.5, 0.32, 0.66),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X + 32.0, CANNON_Y - 10.0),
        Vec2::new(10.0, 14.0),
        Color::srgb(0.5, 0.32, 0.66),
        GameEntity,
    );

    // 炮管 (旋转的)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.58, 0.94), Vec2::new(16.0, 46.0)),
        Transform {
            translation: Vec3::new(CANNON_X, CANNON_Y + 22.0, Z_CANNON),
            rotation: Quat::IDENTITY,
            ..default()
        },
        CannonBarrel,
        GameEntity,
    ));
}

pub fn spawn_loaded_bubble(
    commands: &mut Commands,
    assets: &BubbleAssets,
    current: u8,
    aim: f32,
) {
    let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + super::grid::aim_dir(aim) * 6.0;
    commands
        .spawn((
            Sprite {
                image: assets.ball.clone(),
                color: palette(current),
                custom_size: Some(Vec2::splat(BUBBLE_D - 4.0)),
                ..default()
            },
            Transform::from_translation(loaded_pos.extend(Z_CANNON - 0.1)),
            LoadedBubble,
            BubbleColor(current),
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: assets.highlight.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.85),
                    custom_size: Some(Vec2::splat(12.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(-6.0, 6.0, 0.05)),
            ));
        });
}

fn spawn_next_preview(commands: &mut Commands, assets: &BubbleAssets, font: &UiFont, next: u8) {
    let preview_x = CANNON_X - 92.0;
    let preview_y = CANNON_Y - 4.0;
    panel(
        commands,
        Vec2::new(preview_x, preview_y),
        Vec2::new(56.0, 56.0),
        Color::srgb(0.99, 0.92, 0.78),
        Color::srgb(0.96, 0.66, 0.84),
        GameEntity,
    );
    text(
        commands,
        font,
        "下一",
        Vec2::new(preview_x, preview_y + 38.0),
        12.0,
        Color::srgb(0.6, 0.32, 0.5),
        GameEntity,
    );
    commands
        .spawn((
            Sprite {
                image: assets.ball.clone(),
                color: palette(next),
                custom_size: Some(Vec2::splat(BUBBLE_D - 6.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(preview_x, preview_y, Z_CANNON - 0.1)),
            NextBubbleSprite,
            BubbleColor(next),
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: assets.highlight.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(-5.0, 5.0, 0.05)),
            ));
        });
}

fn spawn_aim_dots(commands: &mut Commands) {
    for i in 0..7 {
        commands.spawn((
            Sprite::from_color(Color::srgba(0.4, 0.32, 0.52, 0.7), Vec2::splat(5.0)),
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
        Color::srgb(1.0, 0.94, 0.88),
        Color::srgb(0.96, 0.66, 0.84),
        GameEntity,
    );
    text(
        commands,
        font,
        "泡泡龙",
        Vec2::new(hud_x, 270.0),
        24.0,
        Color::srgb(0.94, 0.38, 0.62),
        GameEntity,
    );
    text(
        commands,
        font,
        "P1\n← / → 或 A/D 瞄准\nSpace / J 发射\nEsc 暂停",
        Vec2::new(hud_x, 200.0),
        14.0,
        Color::srgb(0.45, 0.32, 0.5),
        GameEntity,
    );
    text(
        commands,
        font,
        "",
        Vec2::new(hud_x, 110.0),
        16.0,
        Color::srgb(0.78, 0.42, 0.18),
        GameEntity,
    )
    .insert(BubbleHud);

    // 中央提示
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(24.0).with_font(font.0.clone()),
        TextColor(Color::srgb(0.94, 0.46, 0.32)),
        Transform::from_translation(Vec3::new(PLAY_OFFSET_X, PLAY_TOP - 56.0, Z_HUD)),
        BubbleMessage,
        GameEntity,
    ));
}

/// 生成一颗网格泡泡，返回主体实体；高光作为子实体随父亲一起变换与销毁。
pub fn spawn_grid_bubble(
    commands: &mut Commands,
    assets: &BubbleAssets,
    col: i32,
    row: i32,
    color_id: u8,
    descend: usize,
) -> Entity {
    let pos = cell_to_pos(col, row, descend);
    commands
        .spawn((
            Sprite {
                image: assets.ball.clone(),
                color: palette(color_id),
                custom_size: Some(Vec2::splat(BUBBLE_D - 2.0)),
                ..default()
            },
            Transform::from_translation(pos.extend(Z_BUBBLE)),
            GridBubble,
            BubbleCell { col, row },
            BubbleColor(color_id),
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: assets.highlight.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                    custom_size: Some(Vec2::splat(12.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(-5.5, 5.5, 0.05)),
            ));
        })
        .id()
}

/// 生成飞行中的泡泡（不进网格），返回主体实体。
pub fn spawn_flying_bubble(
    commands: &mut Commands,
    assets: &BubbleAssets,
    pos: Vec2,
    vel: Vec2,
    color_id: u8,
) {
    commands
        .spawn((
            Sprite {
                image: assets.ball.clone(),
                color: palette(color_id),
                custom_size: Some(Vec2::splat(BUBBLE_D - 4.0)),
                ..default()
            },
            Transform::from_translation(pos.extend(Z_FLYING)),
            FlyingBubble { vel },
            BubbleColor(color_id),
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: assets.highlight.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.85),
                    custom_size: Some(Vec2::splat(11.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(-5.0, 5.0, 0.05)),
            ));
        });
}
