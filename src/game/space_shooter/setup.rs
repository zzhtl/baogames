use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::constants::{FONT_HEADING, FONT_SMALL};
use crate::common::render::{UiFont, attach_sprite_parts, rect};
use crate::common::sprite_def::SpriteDef;
use crate::game::hud::{hud_panel, hud_text};
use crate::game::model::{Collider, GameEntity, Lifetime};

use super::components::*;
use super::constants::*;
use super::resources::SpaceState;
use super::sprites::{
    ENEMY_BOMBER, ENEMY_BOSS, ENEMY_CARRIER, ENEMY_SCOUT, ENEMY_SNIPER, POWERUP_P, SHIP_PLAYER,
};
use super::waves::build_wave;

pub fn setup_stage(commands: &mut Commands, font: &UiFont, hud_root: Entity, level: u8) {
    paint_background(commands);
    spawn_starfield(commands);
    paint_frame(commands);
    spawn_player_ship(commands, Vec2::new(PLAYER_RESPAWN_X, PLAYER_RESPAWN_Y));
    spawn_hud(commands, font, hud_root);

    commands.insert_resource(SpaceState {
        power: 1,
        wave_idx: 0,
        wave_clock: 0.0,
        pending: build_wave(0, level),
        wave_in_progress: true,
        between_wave_clock: 0.0,
        boss_spawned: false,
        boss_defeated: false,
        boss_hp_max: 0,
        message: format!("第 {} 关 — 准备", level),
        message_clock: 2.5,
    });
}

fn paint_background(commands: &mut Commands) {
    // 太空背景：深蓝渐变
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, 0.0),
        Vec2::new(PLAY_W, PLAY_H),
        Color::srgb(0.02, 0.04, 0.09),
        GameEntity,
    );
    // 上下两条带让视觉更有层次
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 40.0),
        Vec2::new(PLAY_W, 80.0),
        Color::srgb(0.04, 0.06, 0.13),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM + 40.0),
        Vec2::new(PLAY_W, 80.0),
        Color::srgb(0.04, 0.06, 0.13),
        GameEntity,
    );
}

fn spawn_starfield(commands: &mut Commands) {
    let mut rng = thread_rng();
    for _ in 0..70 {
        let x = rng.gen_range(PLAY_LEFT + 4.0..PLAY_RIGHT - 4.0);
        let y = rng.gen_range(PLAY_BOTTOM..PLAY_TOP);
        let layer: u8 = rng.gen_range(0..3);
        let (size, brightness, speed) = match layer {
            0 => (1.5, 0.32, 50.0),
            1 => (2.0, 0.55, 90.0),
            _ => (2.6, 0.85, 140.0),
        };
        let mut cmd = rect(
            commands,
            Vec2::new(x, y),
            Vec2::splat(size),
            Color::srgb(brightness, brightness, brightness * 0.95),
            GameEntity,
        );
        cmd.insert(SpaceStar { speed });
        let mut t = Transform::from_translation(Vec3::new(x, y, Z_STAR));
        t.scale = Vec3::splat(1.0);
        cmd.insert(t);
    }
}

fn paint_frame(commands: &mut Commands) {
    let frame_thickness = 6.0;
    let outer_w = PLAY_W + frame_thickness * 2.0;
    let outer_h = PLAY_H + frame_thickness * 2.0;
    let color = Color::srgb(0.32, 0.38, 0.5);
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP + frame_thickness * 0.5),
        Vec2::new(outer_w, frame_thickness),
        color,
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM - frame_thickness * 0.5),
        Vec2::new(outer_w, frame_thickness),
        color,
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_LEFT - frame_thickness * 0.5, 0.0),
        Vec2::new(frame_thickness, outer_h),
        color,
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_RIGHT + frame_thickness * 0.5, 0.0),
        Vec2::new(frame_thickness, outer_h),
        color,
        GameEntity,
    )
    .insert(SpaceFrame);
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    let hud_x = PLAY_RIGHT + 110.0;
    hud_panel(
        commands,
        hud_root,
        Vec2::new(hud_x, 180.0),
        Vec2::new(180.0, 230.0),
        Color::srgb(0.06, 0.08, 0.14),
        Color::srgb(0.36, 0.48, 0.78),
    );
    hud_text(
        commands,
        font,
        hud_root,
        "太空射击",
        Vec2::new(hud_x, 270.0),
        FONT_HEADING,
        Color::srgb(0.78, 0.92, 1.0),
        (),
    );
    hud_text(
        commands,
        font,
        hud_root,
        "P1\nWASD 移动\nJ / 空格 射击\nEsc 暂停",
        Vec2::new(hud_x, 195.0),
        FONT_SMALL,
        Color::srgb(0.7, 0.84, 1.0),
        (),
    );

    hud_text(
        commands,
        font,
        hud_root,
        "",
        Vec2::new(hud_x, 110.0),
        16.0,
        Color::srgb(1.0, 0.94, 0.7),
        SpaceHud,
    );

    // 居中提示
    hud_text(
        commands,
        font,
        hud_root,
        "",
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 60.0),
        24.0,
        Color::srgb(1.0, 0.92, 0.5),
        SpaceMessageText,
    );
}

fn enemy_def(kind: EnemyKind) -> &'static SpriteDef {
    match kind {
        EnemyKind::Scout => &ENEMY_SCOUT,
        EnemyKind::Sniper => &ENEMY_SNIPER,
        EnemyKind::Bomber => &ENEMY_BOMBER,
        EnemyKind::Carrier => &ENEMY_CARRIER,
        EnemyKind::Boss => &ENEMY_BOSS,
    }
}

pub fn spawn_player_ship(commands: &mut Commands, pos: Vec2) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), SHIP_PLAYER.size),
            Transform::from_translation(pos.extend(Z_PLAYER)),
            GameEntity,
            SpaceShipPlayer {
                fire_cd_left: 0.0,
                invincible_left: PLAYER_INVINCIBLE,
                blink_phase: 0.0,
            },
            Collider {
                size: Vec2::new(14.0, 22.0),
            },
        ))
        .id();
    attach_sprite_parts(commands, parent, &SHIP_PLAYER, GameEntity);
}

pub fn spawn_enemy(
    commands: &mut Commands,
    kind: EnemyKind,
    pos: Vec2,
    drops_power: bool,
) -> Entity {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), kind.size()),
            Transform::from_translation(pos.extend(Z_ENEMY)),
            GameEntity,
            SpaceEnemy {
                kind,
                hp: kind.hp(),
                points: kind.points(),
                fire_cd_left: kind.initial_cd(),
                time_alive: 0.0,
                spawn_x: pos.x,
                drops_power,
            },
            Collider { size: kind.collider() },
        ))
        .id();
    attach_sprite_parts(commands, parent, enemy_def(kind), GameEntity);
    parent
}

pub fn spawn_bullet(
    commands: &mut Commands,
    pos: Vec2,
    vel: Vec2,
    from_player: bool,
    damage: i32,
    color: Color,
    size: Vec2,
) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(pos.extend(Z_BULLET)),
        GameEntity,
        SpaceBullet {
            vel,
            from_player,
            damage,
        },
        Collider {
            size: size * 0.85,
        },
    ));
}

pub fn spawn_powerup(commands: &mut Commands, pos: Vec2) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), POWERUP_P.size),
            Transform::from_translation(pos.extend(Z_POWERUP)),
            GameEntity,
            SpacePowerUp,
            Collider { size: Vec2::splat(22.0) },
        ))
        .id();
    attach_sprite_parts(commands, parent, &POWERUP_P, GameEntity);
}

pub fn spawn_explosion(commands: &mut Commands, pos: Vec2, big: bool) {
    let count = if big { 14 } else { 6 };
    let mut rng = thread_rng();
    let base_color = if big {
        Color::srgb(1.0, 0.65, 0.28)
    } else {
        Color::srgb(1.0, 0.78, 0.42)
    };
    commands.spawn((
        Sprite::from_color(base_color, Vec2::splat(if big { 60.0 } else { 28.0 })),
        Transform::from_translation(pos.extend(Z_PARTICLE)),
        GameEntity,
        Lifetime(Timer::new(Duration::from_millis(180), TimerMode::Once)),
    ));
    for _ in 0..count {
        let dx = rng.gen_range(-1.0..1.0);
        let dy = rng.gen_range(-1.0..1.0);
        let off = Vec2::new(dx, dy) * if big { 28.0 } else { 14.0 };
        let size = if big {
            rng.gen_range(8.0..16.0)
        } else {
            rng.gen_range(4.0..9.0)
        };
        commands.spawn((
            Sprite::from_color(
                Color::srgb(
                    1.0,
                    rng.gen_range(0.55..0.92),
                    rng.gen_range(0.18..0.45),
                ),
                Vec2::splat(size),
            ),
            Transform::from_translation((pos + off).extend(Z_PARTICLE)),
            GameEntity,
            Lifetime(Timer::new(
                Duration::from_millis(rng.gen_range(180..360)),
                TimerMode::Once,
            )),
        ));
    }
}
