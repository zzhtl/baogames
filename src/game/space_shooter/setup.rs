use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::constants::{FONT_BODY};
use crate::common::render::{UiFont, attach_sprite_parts, rect};
use crate::common::sprite_def::SpriteDef;
use bevy::sprite::Anchor;
use crate::common::px::{px, snap};
use crate::game::hud::{hud_panel, hud_text, hud_text_anchored};
use crate::game::model::{Collider, GameEntity, Lifetime};

use super::components::*;
use super::constants::*;
use super::resources::{SpaceControls, SpaceState};
use super::sprites::{
    ENEMY_BOMBER, ENEMY_BOSS, ENEMY_CARRIER, ENEMY_SCOUT, ENEMY_SNIPER, POWERUP_P, SHIP_PLAYER,
};
use super::waves::build_wave;

pub fn setup_stage(commands: &mut Commands, font: &UiFont, hud_root: Entity, level: u8) {
    commands.insert_resource(SpaceControls::default());
    paint_background(commands);
    spawn_starfield(commands);
    paint_frame(commands);
    spawn_player_ship(commands, Vec2::new(PLAYER_RESPAWN_X, PLAYER_RESPAWN_Y));
    spawn_hud(commands, font, hud_root);

    commands.insert_resource(SpaceState {
        power: 1,
        rolls: 3,
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
    // 层级必须夹在关卡底板(-10)和星空(-0.5)之间：
    // 画在 z=0（rect 的默认层）会把整片星空盖掉，画在 -10 又会和关卡底板抢同一层。
    let mut band = |y: f32, h: f32, color: Color, z: f32| {
        commands.spawn((
            Sprite::from_color(color, Vec2::new(PLAY_W, h)),
            Transform::from_translation(Vec3::new(PLAY_OFFSET_X, y, z)),
            GameEntity,
        ));
    };
    band(0.0, PLAY_H, Color::srgb(0.02, 0.04, 0.09), Z_SPACE_BG);
    band(PLAY_TOP - 40.0, 80.0, Color::srgb(0.04, 0.06, 0.13), Z_SPACE_BAND);
    band(PLAY_BOTTOM + 40.0, 80.0, Color::srgb(0.04, 0.06, 0.13), Z_SPACE_BAND);
}

fn spawn_starfield(commands: &mut Commands) {
    let mut rng = thread_rng();
    for _ in 0..70 {
        let x = snap(rng.gen_range(PLAY_LEFT + px(2.0)..PLAY_RIGHT - px(2.0)));
        let y = snap(rng.gen_range(PLAY_BOTTOM..PLAY_TOP));
        let layer: u8 = rng.gen_range(0..3);
        // 尺寸必须是整画布像素：原来的 1.5~2.6 世界单位只有 0.5~0.87 像素，
        // Msaa::Off 下整片星空直接不可见，战场是纯黑的。
        let (size, brightness, speed) = match layer {
            0 => (px(1.0), 0.34, 50.0),
            1 => (px(1.0), 0.62, 90.0),
            _ => (px(2.0), 0.92, 140.0),
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
    // 右侧信息栏：从战场右边界排到画布右缘。原来面板只有 60 画布像素宽，
    // 里面塞的却是「动作一射击 · 动作二翻滚」这种 144 像素的整句，必然出屏。
    hud_panel(
        commands,
        hud_root,
        Vec2::new(px(79.0), px(30.0)),
        Vec2::new(px(78.0), px(112.0)),
        Color::srgb(0.06, 0.08, 0.14),
        Color::srgb(0.36, 0.48, 0.78),
    );
    hud_text(
        commands, font, hud_root, "太空射击",
        Vec2::new(px(79.0), px(78.0)), FONT_BODY, Color::srgb(0.78, 0.92, 1.0), (),
    );
    hud_text_anchored(
        commands, font, hud_root, "",
        Vec2::new(px(44.0), px(68.0)), FONT_BODY,
        Color::srgb(0.86, 0.94, 1.0), Anchor::TOP_LEFT, SpaceHud,
    );

    // 战场中央的瞬时提示
    hud_text(
        commands, font, hud_root, "",
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - px(20.0)), FONT_BODY,
        Color::srgb(1.0, 0.92, 0.5), SpaceMessageText,
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
                roll_left: 0.0,
                recoil_left: 0.0,
                move_dir: Vec2::ZERO,
            },
            Collider {
                size: Vec2::new(14.0, 22.0),
            },
        ))
        .id();
    attach_sprite_parts(commands, parent, &SHIP_PLAYER, GameEntity);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.35, 0.85, 1.0), Vec2::new(6.0, 11.0)),
        Transform::from_translation(Vec3::new(0.0, -19.0, -0.05)),
        SpaceEngineFlame,
        ChildOf(parent),
        GameEntity,
    ));
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
                hit_flash_left: 0.0,
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

pub fn spawn_muzzle_flash(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.95, 0.55), Vec2::new(12.0, 18.0)),
        Transform::from_translation(pos.extend(Z_PARTICLE)),
        GameEntity,
        SpaceMuzzleFlash,
        Lifetime(Timer::new(Duration::from_millis(70), TimerMode::Once)),
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
        SpaceExplosionParticle { grows: true },
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
            SpaceExplosionParticle { grows: false },
            Lifetime(Timer::new(
                Duration::from_millis(rng.gen_range(180..360)),
                TimerMode::Once,
            )),
        ));
    }
}

pub fn spawn_hit_spark(commands: &mut Commands, pos: Vec2) {
    for rotation in [0.0, std::f32::consts::FRAC_PI_2] {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.95, 0.65), Vec2::new(16.0, 3.0)),
            Transform::from_translation(pos.extend(Z_PARTICLE))
                .with_rotation(Quat::from_rotation_z(rotation)),
            GameEntity,
            SpaceExplosionParticle { grows: false },
            Lifetime(Timer::new(Duration::from_millis(90), TimerMode::Once)),
        ));
    }
}
