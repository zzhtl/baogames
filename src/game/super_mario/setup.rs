use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::common::render::{UiFont, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::geometry::tile_center;
use super::levels::{build_level_1_1, build_level_1_2, build_level_1_3, build_level_1_4};
use super::palette::*;
use super::resources::MarioStage;
use super::setup_actors::{
    spawn_axe, spawn_bowser, spawn_goomba, spawn_koopa, spawn_player,
};
use super::setup_terrain::{
    spawn_background_decor, spawn_brick, spawn_castle, spawn_dark_block, spawn_flag,
    spawn_ground, spawn_high_sky_decor, spawn_lava, spawn_moving_platform, spawn_pipe_tile,
    spawn_question, spawn_stone,
};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    let bg_color = match level {
        2 => Color::srgb(0.02, 0.02, 0.03),
        3 => Color::srgb(0.13, 0.20, 0.42),
        4 => Color::srgb(0.10, 0.04, 0.04),
        _ => COLOR_SKY,
    };
    commands.spawn((
        Sprite::from_color(bg_color, Vec2::new(ARENA_W * 1.4, ARENA_H * 1.4)),
        Transform::from_translation(Vec3::new(0.0, 0.0, Z_BG_FAR)),
        MarioBackground,
        GameEntity,
    ));

    if level == 1 {
        spawn_background_decor(commands);
    } else if level == 3 {
        spawn_high_sky_decor(commands);
    }

    let mut player_spawn = Vec2::new(2.0 * TILE, FLOOR_Y + TILE);
    let mut goomba_spawns: Vec<Vec2> = Vec::new();
    let mut koopa_green_spawns: Vec<Vec2> = Vec::new();
    let mut koopa_red_spawns: Vec<Vec2> = Vec::new();
    let mut bowser_spawn: Option<Vec2> = None;
    let mut flag_col = 0i32;
    let mut axe_pos: Option<Vec2> = None;
    let map = match level {
        2 => build_level_1_2(),
        3 => build_level_1_3(),
        4 => build_level_1_4(),
        _ => build_level_1_1(),
    };
    let h = map.len() as i32;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            let col = col_idx as i32;
            let row_from_bottom = h - 1 - row_idx as i32;
            let pos = tile_center(col, row_from_bottom);
            match ch {
                'G' => spawn_ground(commands, pos),
                'B' => spawn_brick(commands, pos),
                'Q' => spawn_question(commands, pos, QuestionContent::Coin),
                'U' => spawn_question(commands, pos, QuestionContent::Mushroom),
                '*' => spawn_question(commands, pos, QuestionContent::Star),
                '1' => spawn_question(commands, pos, QuestionContent::OneUp),
                'S' => spawn_stone(commands, pos),
                'D' => spawn_dark_block(commands, pos),
                'p' => spawn_pipe_tile(commands, pos),
                'l' => spawn_lava(commands, pos),
                'L' => spawn_moving_platform(commands, pos, true),
                'H' => spawn_moving_platform(commands, pos, false),
                'F' => {
                    flag_col = col;
                }
                'A' => {
                    axe_pos = Some(pos);
                }
                'M' => {
                    player_spawn = Vec2::new(pos.x, pos.y + (PLAYER_H - TILE) * 0.5);
                }
                'g' => {
                    goomba_spawns.push(Vec2::new(pos.x, pos.y - (TILE - GOOMBA_H) * 0.5));
                }
                'k' => {
                    koopa_green_spawns
                        .push(Vec2::new(pos.x, pos.y - (TILE - KOOPA_H) * 0.5 + 6.0));
                }
                'r' => {
                    koopa_red_spawns
                        .push(Vec2::new(pos.x, pos.y - (TILE - KOOPA_H) * 0.5 + 6.0));
                }
                'b' => {
                    bowser_spawn = Some(Vec2::new(pos.x, pos.y));
                }
                'C' => spawn_castle(commands, pos),
                _ => {}
            }
        }
    }

    if flag_col != 0 {
        spawn_flag(commands, flag_col);
    }
    if let Some(ap) = axe_pos {
        spawn_axe(commands, ap);
    }

    spawn_player(commands, player_spawn);

    for p in goomba_spawns {
        spawn_goomba(commands, p);
    }
    for p in koopa_green_spawns {
        spawn_koopa(commands, p, KoopaKind::Green);
    }
    for p in koopa_red_spawns {
        spawn_koopa(commands, p, KoopaKind::Red);
    }
    if let Some(p) = bowser_spawn {
        spawn_bowser(commands, p);
    }

    spawn_hud(commands, font, level);

    commands.insert_resource(MarioStage {
        time_left: LEVEL_TIME,
        coins: 0,
        level,
        finish_timer: 0.0,
        player_spawn,
    });
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    let y = ARENA_H * 0.5 - 24.0;
    text(
        commands,
        font,
        "MARIO",
        Vec2::new(-ARENA_W * 0.5 + 70.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let score_e = text(
        commands,
        font,
        "000000",
        Vec2::new(-ARENA_W * 0.5 + 70.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(score_e).insert(MarioHud { kind: HudKind::Score });

    text(
        commands,
        font,
        "★",
        Vec2::new(-100.0, y - 22.0),
        20.0,
        COLOR_COIN,
        GameEntity,
    );
    let coin_e = text(
        commands,
        font,
        "x00",
        Vec2::new(-60.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(coin_e).insert(MarioHud { kind: HudKind::Coins });

    text(
        commands,
        font,
        "WORLD",
        Vec2::new(120.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let world_e = text(
        commands,
        font,
        &format!("1-{}", level.max(1)),
        Vec2::new(120.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(world_e).insert(MarioHud { kind: HudKind::World });

    text(
        commands,
        font,
        "TIME",
        Vec2::new(ARENA_W * 0.5 - 80.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let time_e = text(
        commands,
        font,
        "300",
        Vec2::new(ARENA_W * 0.5 - 80.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(time_e).insert(MarioHud { kind: HudKind::Time });

    let status_e = text(
        commands,
        font,
        "",
        Vec2::new(0.0, 0.0),
        28.0,
        Color::srgb(1.0, 0.95, 0.4),
        GameEntity,
    )
    .id();
    commands.entity(status_e).insert(MarioHud { kind: HudKind::Status });
}
