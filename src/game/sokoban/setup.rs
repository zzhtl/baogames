use bevy::prelude::*;

use crate::common::constants::{FONT_HEADING, FONT_SMALL};
use crate::common::render::{UiFont, attach_sprite_parts};
use crate::game::hud::{hud_panel, hud_text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::resources::{SokobanStage, Tile};
use super::sprites::{
    BOX_DONE, BOX_WOOD, FLOOR_TILE, GOAL_TILE, SOKO_PLAYER, TILE_CANONICAL, WALL_TILE,
};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, hud_root: Entity, level: u8) {
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

    spawn_hud(commands, font, hud_root, level);

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

/// 地块组节点：透明占位 + 整体缩放 + SpriteDef 子块（与离线预览同源）。
fn spawn_tile(commands: &mut Commands, center: Vec2, size: f32, tile: Tile) {
    let def = match tile {
        Tile::Wall => &WALL_TILE,
        Tile::Floor => &FLOOR_TILE,
        Tile::Goal => &GOAL_TILE,
    };
    let group = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(TILE_CANONICAL)),
            Transform::from_translation(center.extend(0.0))
                .with_scale(Vec3::splat(size / TILE_CANONICAL)),
            SokoTileSprite,
            GameEntity,
        ))
        .id();
    attach_sprite_parts(commands, group, def, GameEntity);
}

fn spawn_box(commands: &mut Commands, center: Vec2, size: f32, index: usize, on_goal: bool) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(size)),
            Transform::from_translation(center.extend(0.5))
                .with_scale(Vec3::splat(size / TILE_CANONICAL)),
            SokoBox { index },
            GameEntity,
        ))
        .id();
    // 普通 / 完成两个互斥显示的组
    let normal = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(TILE_CANONICAL)),
            Transform::from_xyz(0.0, 0.0, 0.1),
            if on_goal {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            },
            SokoBoxNormal,
            GameEntity,
            ChildOf(parent),
        ))
        .id();
    attach_sprite_parts(commands, normal, &BOX_WOOD, GameEntity);
    let done = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(TILE_CANONICAL)),
            Transform::from_xyz(0.0, 0.0, 0.1),
            if on_goal {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
            SokoBoxDone,
            GameEntity,
            ChildOf(parent),
        ))
        .id();
    attach_sprite_parts(commands, done, &BOX_DONE, GameEntity);
}

fn spawn_player(commands: &mut Commands, center: Vec2, size: f32) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(size)),
            Transform::from_translation(center.extend(1.0))
                .with_scale(Vec3::splat(size / TILE_CANONICAL)),
            SokoPlayer,
            GameEntity,
        ))
        .id();
    attach_sprite_parts(commands, parent, &SOKO_PLAYER, GameEntity);
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity, level: u8) {
    let hud_y = 230.0;
    hud_panel(
        commands,
        hud_root,
        Vec2::new(0.0, hud_y),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.10, 0.13, 0.20),
        Color::srgb(0.86, 0.62, 0.32),
    );
    hud_text(
        commands,
        font,
        hud_root,
        &format!("推箱子 · 第 {} 关", level),
        Vec2::new(-350.0, hud_y),
        FONT_HEADING,
        Color::srgb(1.0, 0.92, 0.72),
        (),
    );
    hud_text(
        commands,
        font,
        hud_root,
        "",
        Vec2::new(160.0, hud_y),
        15.0,
        Color::srgb(0.92, 0.96, 1.0),
        SokobanHud,
    );

    hud_panel(
        commands,
        hud_root,
        Vec2::new(0.0, -230.0),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.08, 0.10, 0.15),
        Color::srgb(0.96, 0.72, 0.32),
    );
    hud_text(
        commands,
        font,
        hud_root,
        "方向键 / WASD 推箱子，R 重来 · Esc 暂停 · Backspace 返回菜单",
        Vec2::new(-330.0, -230.0),
        FONT_SMALL,
        Color::srgb(0.86, 0.92, 1.0),
        (),
    );
    hud_text(
        commands,
        font,
        hud_root,
        "",
        Vec2::new(280.0, -230.0),
        18.0,
        Color::srgb(1.0, 0.86, 0.42),
        SokobanMessage,
    );
}
