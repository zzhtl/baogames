use bevy::prelude::*;

use crate::common::render::{UiFont, panel, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::resources::{SokobanStage, Tile};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    let idx = (level.clamp(1, 10) - 1) as usize;
    let lines = LEVELS[idx];
    let (cols, rows, tiles, boxes, player) = parse_level(lines);

    let cell_size = (BOARD_W / cols as f32).min(BOARD_H / rows as f32);
    let total_w = cell_size * cols as f32;
    let total_h = cell_size * rows as f32;
    let origin = Vec2::new(
        -total_w * 0.5 + cell_size * 0.5,
        total_h * 0.5 - cell_size * 0.5 + BOARD_CENTER_Y,
    );

    for r in 0..rows as i32 {
        for c in 0..cols as i32 {
            let tile = tiles[(r as u32 * cols + c as u32) as usize];
            let center = Vec2::new(
                origin.x + c as f32 * cell_size,
                origin.y - r as f32 * cell_size,
            );
            spawn_tile(commands, center, cell_size, tile);
        }
    }

    for (i, &(c, r)) in boxes.iter().enumerate() {
        let center = Vec2::new(
            origin.x + c as f32 * cell_size,
            origin.y - r as f32 * cell_size,
        );
        let on_goal = tiles[(r as u32 * cols + c as u32) as usize] == Tile::Goal;
        spawn_box(commands, center, cell_size, i, on_goal);
    }

    let player_center = Vec2::new(
        origin.x + player.0 as f32 * cell_size,
        origin.y - player.1 as f32 * cell_size,
    );
    spawn_player(commands, player_center, cell_size);

    spawn_hud(commands, font, level);

    let total_time = LEVEL_TIME[idx];
    commands.insert_resource(SokobanStage {
        cols,
        rows,
        cell_size,
        origin,
        tiles,
        initial_boxes: boxes.clone(),
        initial_player: player,
        initial_time: total_time,
        boxes,
        player,
        moves: 0,
        pushes: 0,
        time_left: total_time,
        move_cd: 0.0,
        message: format!("第 {} 关 - 把所有箱子推到目标点！", level),
        message_clock: 2.4,
    });
}

/// 解析关卡字符串。抽出来便于单元测试。
pub fn parse_level(
    lines: &[&str],
) -> (u32, u32, Vec<Tile>, Vec<(i32, i32)>, (i32, i32)) {
    let rows = lines.len() as u32;
    let cols = lines
        .iter()
        .map(|s| s.chars().count() as u32)
        .max()
        .unwrap_or(0);
    let mut tiles = vec![Tile::Floor; (cols * rows) as usize];
    let mut boxes: Vec<(i32, i32)> = Vec::new();
    let mut player: (i32, i32) = (0, 0);
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let i = r * cols as usize + c;
            match ch {
                '#' => tiles[i] = Tile::Wall,
                '.' => tiles[i] = Tile::Goal,
                '$' => {
                    tiles[i] = Tile::Floor;
                    boxes.push((c as i32, r as i32));
                }
                '*' => {
                    tiles[i] = Tile::Goal;
                    boxes.push((c as i32, r as i32));
                }
                '@' => {
                    tiles[i] = Tile::Floor;
                    player = (c as i32, r as i32);
                }
                '+' => {
                    tiles[i] = Tile::Goal;
                    player = (c as i32, r as i32);
                }
                _ => tiles[i] = Tile::Floor,
            }
        }
        // 行尾不足补齐为 Floor（vec! 已默认 Floor，无需处理）
    }
    (cols, rows, tiles, boxes, player)
}

fn soko_rect(commands: &mut Commands, pos: Vec2, size: Vec2, color: Color, z: f32) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(pos.extend(z)),
        SokoTileSprite,
        GameEntity,
    ));
}

fn spawn_tile(commands: &mut Commands, center: Vec2, size: f32, tile: Tile) {
    match tile {
        Tile::Wall => {
            // 砖墙：暗边 + 砖面 + 错位砖缝 + 顶高光
            soko_rect(commands, center, Vec2::splat(size), COLOR_WALL_BORDER, 0.0);
            soko_rect(commands, center, Vec2::splat(size - 4.0), COLOR_WALL_INNER, 0.05);
            let m = 1.6;
            soko_rect(commands, center + Vec2::new(0.0, size * 0.17), Vec2::new(size - 4.0, m), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(0.0, -size * 0.17), Vec2::new(size - 4.0, m), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(0.0, size * 0.33), Vec2::new(m, size * 0.28), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(-size * 0.25, 0.0), Vec2::new(m, size * 0.28), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(size * 0.25, 0.0), Vec2::new(m, size * 0.28), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(0.0, -size * 0.33), Vec2::new(m, size * 0.28), COLOR_WALL_MORTAR, 0.06);
            soko_rect(commands, center + Vec2::new(0.0, size * 0.42), Vec2::new(size - 6.0, 2.0), COLOR_WALL_HI, 0.07);
        }
        Tile::Floor => {
            soko_rect(commands, center, Vec2::splat(size), COLOR_FLOOR_EDGE, 0.0);
            soko_rect(commands, center, Vec2::splat(size - 3.0), COLOR_FLOOR, 0.02);
        }
        Tile::Goal => {
            soko_rect(commands, center, Vec2::splat(size), COLOR_FLOOR_EDGE, 0.0);
            soko_rect(commands, center, Vec2::splat(size - 3.0), COLOR_FLOOR, 0.02);
            // 靶心：外环 + 内圈 + 发光中心
            let d = size * 0.5;
            soko_rect(commands, center, Vec2::splat(d + 4.0), COLOR_GOAL_BORDER, 0.1);
            soko_rect(commands, center, Vec2::splat(d), COLOR_GOAL_INNER, 0.12);
            soko_rect(commands, center, Vec2::splat(d * 0.45), COLOR_GOAL_GLOW, 0.14);
        }
    }
}

fn spawn_box(commands: &mut Commands, center: Vec2, size: f32, index: usize, on_goal: bool) {
    let box_size = size * 0.86;
    let (border, inner) = if on_goal {
        (COLOR_BOX_DONE_BORDER, COLOR_BOX_DONE_INNER)
    } else {
        (COLOR_BOX_BORDER, COLOR_BOX_INNER)
    };
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(box_size)),
            Transform::from_translation(center.extend(0.5)),
            SokoBox { index },
            GameEntity,
        ))
        .id();
    commands.spawn((
        Sprite::from_color(border, Vec2::splat(box_size)),
        Transform::from_xyz(0.0, 0.0, 0.6),
        SokoBoxBorder,
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Sprite::from_color(inner, Vec2::splat(box_size - 8.0)),
        Transform::from_xyz(0.0, 0.0, 0.65),
        SokoBoxInner,
        GameEntity,
        ChildOf(parent),
    ));
    // 木箱：横向木纹 + 四角金属包边（都随 border 变色，完成时整体变绿）
    let iw = box_size - 8.0;
    for dy in [iw * 0.30, -iw * 0.30] {
        commands.spawn((
            Sprite::from_color(border, Vec2::new(iw, 2.5)),
            Transform::from_xyz(0.0, dy, 0.7),
            SokoBoxBorder,
            GameEntity,
            ChildOf(parent),
        ));
    }
    let corner = box_size * 0.20;
    let off = box_size * 0.5 - corner * 0.5 - 2.0;
    for (sx, sy) in [(-1.0, 1.0), (1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)] {
        commands.spawn((
            Sprite::from_color(border, Vec2::splat(corner)),
            Transform::from_xyz(sx * off, sy * off, 0.72),
            SokoBoxBorder,
            GameEntity,
            ChildOf(parent),
        ));
    }
}

fn spawn_player(commands: &mut Commands, center: Vec2, size: f32) {
    let p_size = size * 0.78;
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(p_size)),
            Transform::from_translation(center.extend(1.0)),
            SokoPlayer,
            GameEntity,
        ))
        .id();
    commands.spawn((
        Sprite::from_color(COLOR_PLAYER_BORDER, Vec2::splat(p_size)),
        Transform::from_xyz(0.0, 0.0, 1.1),
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Sprite::from_color(COLOR_PLAYER_INNER, Vec2::splat(p_size - 6.0)),
        Transform::from_xyz(0.0, 0.0, 1.15),
        GameEntity,
        ChildOf(parent),
    ));
    // 眼睛
    let eye = p_size * 0.16;
    let dx = p_size * 0.22;
    let dy = p_size * 0.1;
    for sx in [-1.0, 1.0] {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.05, 0.07, 0.12), Vec2::splat(eye)),
            Transform::from_xyz(sx * dx, dy, 1.2),
            GameEntity,
            ChildOf(parent),
        ));
    }
    // 微笑
    commands.spawn((
        Sprite::from_color(COLOR_PLAYER_FACE, Vec2::new(p_size * 0.42, 3.0)),
        Transform::from_xyz(0.0, -p_size * 0.18, 1.2),
        GameEntity,
        ChildOf(parent),
    ));
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    let hud_y = 230.0;
    panel(
        commands,
        Vec2::new(0.0, hud_y),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.10, 0.13, 0.20),
        Color::srgb(0.86, 0.62, 0.32),
        GameEntity,
    );
    text(
        commands,
        font,
        &format!("推箱子 · 第 {} 关", level),
        Vec2::new(-350.0, hud_y),
        20.0,
        Color::srgb(1.0, 0.92, 0.72),
        GameEntity,
    );
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(15.0).with_font(font.0.clone()),
        TextColor(Color::srgb(0.92, 0.96, 1.0)),
        Transform::from_translation(Vec3::new(160.0, hud_y, 10.0)),
        SokobanHud,
        GameEntity,
    ));

    panel(
        commands,
        Vec2::new(0.0, -230.0),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.08, 0.10, 0.15),
        Color::srgb(0.96, 0.72, 0.32),
        GameEntity,
    );
    text(
        commands,
        font,
        "方向键 / WASD 推箱子，R 重来；Esc 暂停，Backspace 返回",
        Vec2::new(-330.0, -230.0),
        13.0,
        Color::srgb(0.86, 0.92, 1.0),
        GameEntity,
    );
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(18.0).with_font(font.0.clone()),
        TextColor(Color::srgb(1.0, 0.86, 0.42)),
        Transform::from_translation(Vec3::new(280.0, -230.0, 10.0)),
        SokobanMessage,
        GameEntity,
    ));
}
