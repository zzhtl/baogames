use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::input::input_for;
use crate::common::render::{UiFont, panel, rect, text};

use super::model::*;

// ========== 常量 ==========
pub(super) const PLAY_W: f32 = 460.0;
pub(super) const PLAY_H: f32 = 520.0;
pub(super) const PLAY_OFFSET_X: f32 = -120.0;
pub(super) const PLAY_LEFT: f32 = PLAY_OFFSET_X - PLAY_W * 0.5;
pub(super) const PLAY_RIGHT: f32 = PLAY_OFFSET_X + PLAY_W * 0.5;
pub(super) const PLAY_TOP: f32 = PLAY_H * 0.5;
pub(super) const PLAY_BOTTOM: f32 = -PLAY_H * 0.5;

const PLAYER_SPEED: f32 = 280.0;
const PLAYER_FIRE_CD_LV1: f32 = 0.18;
const PLAYER_FIRE_CD_LV2: f32 = 0.12;
const PLAYER_FIRE_CD_LV3: f32 = 0.09;
const PLAYER_BULLET_SPEED: f32 = 640.0;
const ENEMY_BULLET_SPEED: f32 = 230.0;
const PLAYER_INVINCIBLE: f32 = 1.6;
const PLAYER_RESPAWN_X: f32 = PLAY_OFFSET_X;
const PLAYER_RESPAWN_Y: f32 = PLAY_BOTTOM + 60.0;

const Z_STAR: f32 = -0.5;
const Z_ENEMY: f32 = 1.0;
const Z_BULLET: f32 = 1.4;
const Z_PLAYER: f32 = 1.6;
const Z_PARTICLE: f32 = 2.0;
const Z_POWERUP: f32 = 1.8;
const Z_HUD: f32 = 8.0;

// ========== 组件 ==========
#[derive(Component)]
pub(super) struct SpaceShipPlayer {
    pub fire_cd_left: f32,
    pub invincible_left: f32,
    pub blink_phase: f32,
}

#[derive(Component)]
pub(super) struct SpaceBullet {
    pub vel: Vec2,
    pub from_player: bool,
    pub damage: i32,
}

#[derive(Component)]
pub(super) struct SpaceEnemy {
    pub kind: EnemyKind,
    pub hp: i32,
    pub points: u32,
    pub fire_cd_left: f32,
    pub time_alive: f32,
    pub spawn_x: f32,
    pub drops_power: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EnemyKind {
    Scout,
    Sniper,
    Bomber,
    Carrier,
    Boss,
}

impl EnemyKind {
    fn size(self) -> Vec2 {
        match self {
            EnemyKind::Scout => Vec2::new(28.0, 28.0),
            EnemyKind::Sniper => Vec2::new(34.0, 24.0),
            EnemyKind::Bomber => Vec2::new(54.0, 36.0),
            EnemyKind::Carrier => Vec2::new(32.0, 32.0),
            EnemyKind::Boss => Vec2::new(170.0, 110.0),
        }
    }
    fn collider(self) -> Vec2 {
        let s = self.size();
        Vec2::new(s.x * 0.85, s.y * 0.85)
    }
    fn hp(self) -> i32 {
        match self {
            EnemyKind::Scout => 1,
            EnemyKind::Sniper => 2,
            EnemyKind::Bomber => 5,
            EnemyKind::Carrier => 3,
            EnemyKind::Boss => 80,
        }
    }
    fn points(self) -> u32 {
        match self {
            EnemyKind::Scout => 100,
            EnemyKind::Sniper => 200,
            EnemyKind::Bomber => 350,
            EnemyKind::Carrier => 250,
            EnemyKind::Boss => 5000,
        }
    }
    fn body_color(self) -> Color {
        match self {
            EnemyKind::Scout => Color::srgb(0.92, 0.36, 0.4),
            EnemyKind::Sniper => Color::srgb(0.96, 0.66, 0.3),
            EnemyKind::Bomber => Color::srgb(0.42, 0.6, 0.36),
            EnemyKind::Carrier => Color::srgb(0.92, 0.42, 0.86),
            EnemyKind::Boss => Color::srgb(0.78, 0.32, 0.36),
        }
    }
    fn accent_color(self) -> Color {
        match self {
            EnemyKind::Scout => Color::srgb(0.62, 0.16, 0.18),
            EnemyKind::Sniper => Color::srgb(0.62, 0.32, 0.1),
            EnemyKind::Bomber => Color::srgb(0.18, 0.34, 0.18),
            EnemyKind::Carrier => Color::srgb(0.55, 0.18, 0.5),
            EnemyKind::Boss => Color::srgb(0.32, 0.12, 0.16),
        }
    }
}

#[derive(Component)]
pub(super) struct SpacePowerUp;

#[derive(Component)]
pub(super) struct SpaceStar {
    pub speed: f32,
}

#[derive(Component)]
pub(super) struct SpaceFrame;

#[derive(Component)]
pub(super) struct SpaceHud;

#[derive(Component)]
pub(super) struct SpaceMessageText;

// ========== 资源 ==========
#[derive(Resource)]
pub(super) struct SpaceState {
    pub power: u8,
    pub wave_idx: usize,
    pub wave_clock: f32,
    pub pending: Vec<PendingSpawn>,
    pub wave_in_progress: bool,
    pub between_wave_clock: f32,
    pub boss_spawned: bool,
    pub boss_defeated: bool,
    pub boss_hp_max: i32,
    pub message: String,
    pub message_clock: f32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PendingSpawn {
    pub delay: f32,
    pub kind: EnemyKind,
    pub pos: Vec2,
    pub drops_power: bool,
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    // 太空背景：深蓝渐变 + 星星
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

    // 星空：三层视差
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

    // 边框
    let frame_thickness = 6.0;
    let outer_w = PLAY_W + frame_thickness * 2.0;
    let outer_h = PLAY_H + frame_thickness * 2.0;
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP + frame_thickness * 0.5),
        Vec2::new(outer_w, frame_thickness),
        Color::srgb(0.32, 0.38, 0.5),
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM - frame_thickness * 0.5),
        Vec2::new(outer_w, frame_thickness),
        Color::srgb(0.32, 0.38, 0.5),
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_LEFT - frame_thickness * 0.5, 0.0),
        Vec2::new(frame_thickness, outer_h),
        Color::srgb(0.32, 0.38, 0.5),
        GameEntity,
    )
    .insert(SpaceFrame);
    rect(
        commands,
        Vec2::new(PLAY_RIGHT + frame_thickness * 0.5, 0.0),
        Vec2::new(frame_thickness, outer_h),
        Color::srgb(0.32, 0.38, 0.5),
        GameEntity,
    )
    .insert(SpaceFrame);

    // 玩家
    spawn_player_ship(commands, Vec2::new(PLAYER_RESPAWN_X, PLAYER_RESPAWN_Y));

    // HUD 面板（右侧）
    let hud_x = PLAY_RIGHT + 110.0;
    panel(
        commands,
        Vec2::new(hud_x, 180.0),
        Vec2::new(180.0, 230.0),
        Color::srgb(0.06, 0.08, 0.14),
        Color::srgb(0.36, 0.48, 0.78),
        GameEntity,
    );
    text(
        commands,
        font,
        "太空射击",
        Vec2::new(hud_x, 270.0),
        20.0,
        Color::srgb(0.78, 0.92, 1.0),
        GameEntity,
    );
    text(
        commands,
        font,
        "P1\nWASD 移动\nJ / 空格 射击\nEsc 暂停",
        Vec2::new(hud_x, 195.0),
        14.0,
        Color::srgb(0.7, 0.84, 1.0),
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
    .insert(SpaceHud);

    // 居中提示
    text(
        commands,
        font,
        "",
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 60.0),
        24.0,
        Color::srgb(1.0, 0.92, 0.5),
        GameEntity,
    )
    .insert(SpaceMessageText)
    .insert(Transform::from_translation(Vec3::new(
        PLAY_OFFSET_X,
        PLAY_TOP - 60.0,
        Z_HUD,
    )));

    // 初始化资源
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

fn spawn_player_ship(commands: &mut Commands, pos: Vec2) {
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.4, 0.82, 0.98), Vec2::new(18.0, 30.0)),
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
        .with_children(|p| {
            // 机翼
            p.spawn((
                Sprite::from_color(Color::srgb(0.32, 0.7, 0.92), Vec2::new(36.0, 8.0)),
                Transform::from_xyz(0.0, -4.0, 0.0),
            ));
            // 副翼
            p.spawn((
                Sprite::from_color(Color::srgb(0.28, 0.6, 0.85), Vec2::new(28.0, 5.0)),
                Transform::from_xyz(0.0, -10.0, 0.0),
            ));
            // 机头
            p.spawn((
                Sprite::from_color(Color::srgb(0.85, 0.95, 1.0), Vec2::new(6.0, 10.0)),
                Transform::from_xyz(0.0, 12.0, 0.01),
            ));
            // 座舱
            p.spawn((
                Sprite::from_color(Color::srgb(0.18, 0.28, 0.42), Vec2::new(8.0, 10.0)),
                Transform::from_xyz(0.0, 2.0, 0.01),
            ));
            // 引擎光
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.78, 0.32), Vec2::new(10.0, 6.0)),
                Transform::from_xyz(0.0, -16.0, 0.01),
            ));
        });
}

fn spawn_enemy(
    commands: &mut Commands,
    kind: EnemyKind,
    pos: Vec2,
    drops_power: bool,
) -> Entity {
    let size = kind.size();
    let body = kind.body_color();
    let accent = kind.accent_color();
    let id = commands
        .spawn((
            Sprite::from_color(body, size),
            Transform::from_translation(pos.extend(Z_ENEMY)),
            GameEntity,
            SpaceEnemy {
                kind,
                hp: kind.hp(),
                points: kind.points(),
                fire_cd_left: enemy_initial_cd(kind),
                time_alive: 0.0,
                spawn_x: pos.x,
                drops_power,
            },
            Collider { size: kind.collider() },
        ))
        .with_children(|p| {
            match kind {
                EnemyKind::Scout => {
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(20.0, 6.0)),
                        Transform::from_xyz(0.0, -4.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.96, 0.78, 0.42), Vec2::new(6.0, 6.0)),
                        Transform::from_xyz(0.0, 4.0, 0.01),
                    ));
                }
                EnemyKind::Sniper => {
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(34.0, 6.0)),
                        Transform::from_xyz(0.0, -6.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.96, 0.86, 0.4), Vec2::new(8.0, 8.0)),
                        Transform::from_xyz(0.0, 0.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(4.0, 10.0)),
                        Transform::from_xyz(-12.0, -8.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(4.0, 10.0)),
                        Transform::from_xyz(12.0, -8.0, 0.01),
                    ));
                }
                EnemyKind::Bomber => {
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(54.0, 8.0)),
                        Transform::from_xyz(0.0, -10.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.7, 0.86, 0.5), Vec2::new(12.0, 12.0)),
                        Transform::from_xyz(0.0, 6.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.18, 0.32, 0.18), Vec2::new(10.0, 6.0)),
                        Transform::from_xyz(-18.0, -16.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.18, 0.32, 0.18), Vec2::new(10.0, 6.0)),
                        Transform::from_xyz(18.0, -16.0, 0.01),
                    ));
                }
                EnemyKind::Carrier => {
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(28.0, 8.0)),
                        Transform::from_xyz(0.0, -6.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(1.0, 0.96, 0.52), Vec2::new(10.0, 10.0)),
                        Transform::from_xyz(0.0, 2.0, 0.01),
                    ));
                }
                EnemyKind::Boss => {
                    // 主体细节
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(170.0, 22.0)),
                        Transform::from_xyz(0.0, -28.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.92, 0.5, 0.5), Vec2::new(40.0, 16.0)),
                        Transform::from_xyz(0.0, 8.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.36, 0.18, 0.2), Vec2::new(60.0, 8.0)),
                        Transform::from_xyz(0.0, -42.0, 0.01),
                    ));
                    // 两侧炮台
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(28.0, 28.0)),
                        Transform::from_xyz(-66.0, -10.0, 0.01),
                    ));
                    p.spawn((
                        Sprite::from_color(accent, Vec2::new(28.0, 28.0)),
                        Transform::from_xyz(66.0, -10.0, 0.01),
                    ));
                    // 中央指挥塔
                    p.spawn((
                        Sprite::from_color(Color::srgb(0.95, 0.86, 0.48), Vec2::new(18.0, 18.0)),
                        Transform::from_xyz(0.0, 18.0, 0.02),
                    ));
                }
            }
        })
        .id();
    id
}

fn enemy_initial_cd(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Scout => 1.4,
        EnemyKind::Sniper => 1.0,
        EnemyKind::Bomber => 1.2,
        EnemyKind::Carrier => 1.6,
        EnemyKind::Boss => 1.5,
    }
}

fn enemy_fire_period(kind: EnemyKind, level: u8) -> f32 {
    let lvl = level as f32;
    match kind {
        EnemyKind::Scout => (2.4 - lvl * 0.12).max(1.4),
        EnemyKind::Sniper => (1.6 - lvl * 0.08).max(0.9),
        EnemyKind::Bomber => (1.8 - lvl * 0.08).max(1.0),
        EnemyKind::Carrier => (2.2 - lvl * 0.1).max(1.2),
        EnemyKind::Boss => (1.4 - lvl * 0.05).max(0.8),
    }
}

// ========== 波次 ==========
fn build_wave(wave_idx: usize, level: u8) -> Vec<PendingSpawn> {
    let mut spawns = Vec::new();
    let lvl_bonus = level.saturating_sub(1) as usize;
    match wave_idx {
        0 => {
            // V 字侦察兵
            for i in 0..7 {
                let x = -180.0 + i as f32 * 60.0 + PLAY_OFFSET_X;
                let dy = (i as f32 - 3.0).abs() * 24.0;
                spawns.push(PendingSpawn {
                    delay: 0.4 + i as f32 * 0.18,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, PLAY_TOP + 40.0 + dy),
                    drops_power: false,
                });
            }
        }
        1 => {
            // 斜线编队
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.35,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_LEFT + 40.0 + i as f32 * 18.0,
                        PLAY_TOP + 30.0 + i as f32 * 30.0,
                    ),
                    drops_power: false,
                });
                spawns.push(PendingSpawn {
                    delay: 0.5 + i as f32 * 0.35,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_RIGHT - 40.0 - i as f32 * 18.0,
                        PLAY_TOP + 30.0 + i as f32 * 30.0,
                    ),
                    drops_power: false,
                });
            }
        }
        2 => {
            // 狙击手
            for i in 0..(3 + lvl_bonus.min(2)) {
                let x = -150.0 + i as f32 * 100.0 + PLAY_OFFSET_X;
                spawns.push(PendingSpawn {
                    delay: 0.5 + i as f32 * 0.6,
                    kind: EnemyKind::Sniper,
                    pos: Vec2::new(x, PLAY_TOP + 60.0),
                    drops_power: false,
                });
            }
            // 一颗补给
            spawns.push(PendingSpawn {
                delay: 1.6,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 80.0),
                drops_power: true,
            });
        }
        3 => {
            // 重型轰炸机
            for i in 0..2 {
                spawns.push(PendingSpawn {
                    delay: 0.6 + i as f32 * 1.4,
                    kind: EnemyKind::Bomber,
                    pos: Vec2::new(
                        if i == 0 { -90.0 } else { 90.0 } + PLAY_OFFSET_X,
                        PLAY_TOP + 80.0,
                    ),
                    drops_power: i == 1,
                });
            }
            // 两侧侦察
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 1.0 + i as f32 * 0.25,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        if i % 2 == 0 { PLAY_LEFT + 40.0 } else { PLAY_RIGHT - 40.0 },
                        PLAY_TOP + 30.0 + i as f32 * 22.0,
                    ),
                    drops_power: false,
                });
            }
        }
        4 => {
            // 弧形侦察
            for i in 0..9 {
                let t = i as f32 / 8.0;
                let x = PLAY_OFFSET_X + (t - 0.5) * 360.0;
                let y = PLAY_TOP + 30.0 + (t - 0.5).abs() * 60.0;
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.16,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, y),
                    drops_power: false,
                });
            }
        }
        5 => {
            // 狙击 + 补给
            for i in 0..4 {
                spawns.push(PendingSpawn {
                    delay: 0.4 + i as f32 * 0.5,
                    kind: EnemyKind::Sniper,
                    pos: Vec2::new(
                        PLAY_OFFSET_X + (i as f32 - 1.5) * 90.0,
                        PLAY_TOP + 60.0,
                    ),
                    drops_power: false,
                });
            }
            spawns.push(PendingSpawn {
                delay: 2.4,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X - 60.0, PLAY_TOP + 100.0),
                drops_power: true,
            });
            spawns.push(PendingSpawn {
                delay: 2.8,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X + 60.0, PLAY_TOP + 100.0),
                drops_power: true,
            });
        }
        6 => {
            // 轰炸机 + 群侦察
            spawns.push(PendingSpawn {
                delay: 0.4,
                kind: EnemyKind::Bomber,
                pos: Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 80.0),
                drops_power: false,
            });
            for i in 0..10 {
                let x = PLAY_LEFT + 30.0 + (i as f32 / 9.0) * (PLAY_W - 60.0);
                spawns.push(PendingSpawn {
                    delay: 1.2 + i as f32 * 0.18,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, PLAY_TOP + 40.0),
                    drops_power: false,
                });
            }
        }
        _ => {
            // 兜底：再来几个
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.3,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_OFFSET_X + (i as f32 - 2.5) * 70.0,
                        PLAY_TOP + 40.0,
                    ),
                    drops_power: false,
                });
            }
        }
    }
    spawns
}

const TOTAL_WAVES_BEFORE_BOSS: usize = 7;

// ========== 系统 ==========
pub(super) fn space_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    state: Res<SpaceState>,
    mut players: Query<(&mut Transform, &mut SpaceShipPlayer)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let input = input_for(&keys, 0);
    let mut dir = input.move_dir;
    if dir.length_squared() > 1.0 {
        dir = dir.normalize();
    }
    let fire_cd = match state.power {
        0 | 1 => PLAYER_FIRE_CD_LV1,
        2 => PLAYER_FIRE_CD_LV2,
        _ => PLAYER_FIRE_CD_LV3,
    };
    for (mut t, mut player) in &mut players {
        t.translation.x += dir.x * PLAYER_SPEED * delta;
        t.translation.y += dir.y * PLAYER_SPEED * delta;
        t.translation.x = t
            .translation
            .x
            .clamp(PLAY_LEFT + 14.0, PLAY_RIGHT - 14.0);
        t.translation.y = t.translation.y.clamp(PLAY_BOTTOM + 18.0, PLAY_TOP - 18.0);
        player.fire_cd_left = (player.fire_cd_left - delta).max(0.0);
        player.invincible_left = (player.invincible_left - delta).max(0.0);
        player.blink_phase += delta * 18.0;
        if input.fire && player.fire_cd_left <= 0.0 {
            let pos = t.translation.truncate();
            let bullet_color = Color::srgb(1.0, 0.92, 0.42);
            match state.power {
                0 | 1 => {
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(0.0, 18.0),
                        Vec2::new(0.0, PLAYER_BULLET_SPEED),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(5.0, 14.0),
                    );
                }
                2 => {
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(-7.0, 16.0),
                        Vec2::new(0.0, PLAYER_BULLET_SPEED),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(5.0, 14.0),
                    );
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(7.0, 16.0),
                        Vec2::new(0.0, PLAYER_BULLET_SPEED),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(5.0, 14.0),
                    );
                }
                _ => {
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(0.0, 20.0),
                        Vec2::new(0.0, PLAYER_BULLET_SPEED),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(6.0, 16.0),
                    );
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(-10.0, 14.0),
                        Vec2::new(-110.0, PLAYER_BULLET_SPEED * 0.96),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(5.0, 14.0),
                    );
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(10.0, 14.0),
                        Vec2::new(110.0, PLAYER_BULLET_SPEED * 0.96),
                        true,
                        1,
                        bullet_color,
                        Vec2::new(5.0, 14.0),
                    );
                }
            }
            player.fire_cd_left = fire_cd;
        }
    }
}

fn spawn_bullet(
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

pub(super) fn space_bullets_update(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bullets: Query<(Entity, &mut Transform, &SpaceBullet)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (e, mut t, b) in &mut bullets {
        t.translation.x += b.vel.x * delta;
        t.translation.y += b.vel.y * delta;
        if t.translation.y > PLAY_TOP + 40.0
            || t.translation.y < PLAY_BOTTOM - 40.0
            || t.translation.x < PLAY_LEFT - 40.0
            || t.translation.x > PLAY_RIGHT + 40.0
        {
            commands.entity(e).despawn();
        }
    }
}

pub(super) fn space_stars_scroll(
    time: Res<Time>,
    session: Res<GameSession>,
    mut stars: Query<(&mut Transform, &SpaceStar)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (mut t, s) in &mut stars {
        t.translation.y -= s.speed * delta;
        if t.translation.y < PLAY_BOTTOM - 4.0 {
            t.translation.y = PLAY_TOP + 4.0;
        }
    }
}

pub(super) fn space_spawner(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut state: ResMut<SpaceState>,
    enemies: Query<(), With<SpaceEnemy>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();

    if state.message_clock > 0.0 {
        state.message_clock -= delta;
        if state.message_clock <= 0.0 {
            state.message.clear();
        }
    }

    if state.wave_in_progress {
        state.wave_clock += delta;
        let mut spawned_now = Vec::new();
        let now = state.wave_clock;
        let mut remaining = Vec::with_capacity(state.pending.len());
        std::mem::swap(&mut remaining, &mut state.pending);
        for spawn in remaining.into_iter() {
            if now >= spawn.delay {
                spawned_now.push(spawn);
            } else {
                state.pending.push(spawn);
            }
        }
        for s in spawned_now {
            spawn_enemy(&mut commands, s.kind, s.pos, s.drops_power);
        }
        // 当所有 pending 都已 spawn 且场上敌人清空 → 进入下一波
        if state.pending.is_empty() && enemies.iter().count() == 0 {
            state.wave_in_progress = false;
            state.between_wave_clock = 1.2;
        }
    } else {
        state.between_wave_clock -= delta;
        if state.between_wave_clock <= 0.0 {
            // 下一波
            let next_idx = state.wave_idx + 1;
            if next_idx >= TOTAL_WAVES_BEFORE_BOSS && !state.boss_spawned {
                // BOSS 出场
                let boss_pos = Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 90.0);
                spawn_enemy(&mut commands, EnemyKind::Boss, boss_pos, false);
                state.boss_spawned = true;
                state.boss_hp_max = EnemyKind::Boss.hp();
                state.message = "警告：BOSS 来袭".to_string();
                state.message_clock = 2.5;
                state.wave_in_progress = true;
                state.wave_clock = 0.0;
                state.pending.clear();
                state.wave_idx = next_idx;
            } else if state.boss_spawned {
                // 不再产生新波次
            } else {
                state.wave_idx = next_idx;
                state.wave_clock = 0.0;
                state.pending = build_wave(next_idx, session.level);
                state.wave_in_progress = true;
                state.message = format!("第 {} 波", next_idx + 1);
                state.message_clock = 1.4;
            }
        }
    }
}

pub(super) fn space_enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    players: Query<&Transform, (With<SpaceShipPlayer>, Without<SpaceEnemy>)>,
    mut enemies: Query<(&mut Transform, &mut SpaceEnemy), Without<SpaceShipPlayer>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let player_pos = players.iter().next().map(|t| t.translation.truncate());
    for (mut t, mut enemy) in &mut enemies {
        enemy.time_alive += delta;
        match enemy.kind {
            EnemyKind::Scout => {
                let speed = 130.0 + session.level as f32 * 6.0;
                t.translation.y -= speed * delta;
                t.translation.x = enemy.spawn_x
                    + (enemy.time_alive * 1.6).sin() * 24.0;
            }
            EnemyKind::Sniper => {
                let target_y = 80.0;
                if t.translation.y > target_y {
                    let speed = 110.0 + session.level as f32 * 5.0;
                    t.translation.y -= speed * delta;
                } else if enemy.time_alive < 4.0 {
                    // 悬停轻微摆动
                    t.translation.x = enemy.spawn_x
                        + (enemy.time_alive * 1.2).sin() * 60.0;
                } else {
                    let speed = 140.0;
                    t.translation.y -= speed * delta;
                }
            }
            EnemyKind::Bomber => {
                let speed = 70.0 + session.level as f32 * 4.0;
                t.translation.y -= speed * delta;
                t.translation.x = enemy.spawn_x
                    + (enemy.time_alive * 1.0).sin() * 80.0;
            }
            EnemyKind::Carrier => {
                let speed = 95.0;
                t.translation.y -= speed * delta;
                t.translation.x = enemy.spawn_x
                    + (enemy.time_alive * 1.4).cos() * 40.0;
            }
            EnemyKind::Boss => {
                let target_y = 130.0;
                if t.translation.y > target_y {
                    t.translation.y -= 60.0 * delta;
                } else {
                    t.translation.x = PLAY_OFFSET_X
                        + (enemy.time_alive * 0.8).sin() * 130.0;
                }
            }
        }

        // 射击
        enemy.fire_cd_left -= delta;
        if enemy.fire_cd_left <= 0.0 && t.translation.y < PLAY_TOP - 20.0 {
            let pos = t.translation.truncate();
            match enemy.kind {
                EnemyKind::Scout => {
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(0.0, -16.0),
                        Vec2::new(0.0, -ENEMY_BULLET_SPEED),
                        false,
                        1,
                        Color::srgb(1.0, 0.55, 0.4),
                        Vec2::new(6.0, 10.0),
                    );
                }
                EnemyKind::Sniper => {
                    if let Some(pp) = player_pos {
                        let dir = (pp - pos).normalize_or_zero();
                        spawn_bullet(
                            &mut commands,
                            pos + dir * 18.0,
                            dir * ENEMY_BULLET_SPEED * 1.15,
                            false,
                            1,
                            Color::srgb(1.0, 0.78, 0.32),
                            Vec2::new(7.0, 7.0),
                        );
                    }
                }
                EnemyKind::Bomber => {
                    for &dx in &[-110.0_f32, 0.0, 110.0] {
                        spawn_bullet(
                            &mut commands,
                            pos + Vec2::new(0.0, -18.0),
                            Vec2::new(dx, -ENEMY_BULLET_SPEED),
                            false,
                            1,
                            Color::srgb(0.7, 0.95, 0.6),
                            Vec2::new(7.0, 11.0),
                        );
                    }
                }
                EnemyKind::Carrier => {
                    spawn_bullet(
                        &mut commands,
                        pos + Vec2::new(0.0, -16.0),
                        Vec2::new(0.0, -ENEMY_BULLET_SPEED * 0.85),
                        false,
                        1,
                        Color::srgb(1.0, 0.55, 0.95),
                        Vec2::new(8.0, 8.0),
                    );
                }
                EnemyKind::Boss => {
                    // 三种交替模式
                    let phase = (enemy.time_alive / 3.0) as i32 % 3;
                    match phase {
                        0 => {
                            // 5 路扇形
                            for i in -2..=2 {
                                let angle =
                                    -std::f32::consts::FRAC_PI_2 + i as f32 * 0.18;
                                let dir = Vec2::new(angle.cos(), angle.sin());
                                spawn_bullet(
                                    &mut commands,
                                    pos + Vec2::new(0.0, -36.0),
                                    dir * ENEMY_BULLET_SPEED,
                                    false,
                                    1,
                                    Color::srgb(1.0, 0.55, 0.5),
                                    Vec2::new(9.0, 9.0),
                                );
                            }
                        }
                        1 => {
                            // 双侧瞄准
                            if let Some(pp) = player_pos {
                                for &offset in &[-60.0_f32, 60.0] {
                                    let from = pos + Vec2::new(offset, -8.0);
                                    let dir = (pp - from).normalize_or_zero();
                                    spawn_bullet(
                                        &mut commands,
                                        from,
                                        dir * ENEMY_BULLET_SPEED * 1.1,
                                        false,
                                        1,
                                        Color::srgb(1.0, 0.78, 0.4),
                                        Vec2::new(8.0, 8.0),
                                    );
                                }
                            }
                        }
                        _ => {
                            // 弹幕环
                            for k in 0..10 {
                                let a = (k as f32) * std::f32::consts::TAU / 10.0
                                    - std::f32::consts::FRAC_PI_2;
                                let dir = Vec2::new(a.cos(), a.sin());
                                spawn_bullet(
                                    &mut commands,
                                    pos + dir * 30.0,
                                    dir * ENEMY_BULLET_SPEED * 0.85,
                                    false,
                                    1,
                                    Color::srgb(1.0, 0.45, 0.55),
                                    Vec2::new(8.0, 8.0),
                                );
                            }
                        }
                    }
                }
            }
            enemy.fire_cd_left = enemy_fire_period(enemy.kind, session.level);
        }

        // 离开屏幕底部 → 销毁
        if t.translation.y < PLAY_BOTTOM - 60.0 {
            // 漏过去的小杂兵不计分
        }
    }
}

pub(super) fn space_despawn_offscreen(
    mut commands: Commands,
    session: Res<GameSession>,
    enemies: Query<(Entity, &Transform, &SpaceEnemy)>,
    powerups: Query<(Entity, &Transform), With<SpacePowerUp>>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (e, t, enemy) in &enemies {
        if enemy.kind != EnemyKind::Boss && t.translation.y < PLAY_BOTTOM - 80.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &powerups {
        if t.translation.y < PLAY_BOTTOM - 40.0 {
            commands.entity(e).despawn();
        }
    }
}

pub(super) fn space_powerup_drop_and_pickup(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut state: ResMut<SpaceState>,
    mut powerups: Query<
        (Entity, &mut Transform, &Collider),
        (With<SpacePowerUp>, Without<SpaceShipPlayer>),
    >,
    players: Query<(&Transform, &Collider), With<SpaceShipPlayer>>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (e, mut pt, pc) in &mut powerups {
        pt.translation.y -= 110.0 * delta;
        for (player_t, player_c) in &players {
            if aabb(
                pt.translation.truncate(),
                pc.size,
                player_t.translation.truncate(),
                player_c.size,
            ) {
                commands.entity(e).despawn();
                state.power = (state.power + 1).min(3);
                session.score += 200;
                state.message = match state.power {
                    2 => "火力 LV.2".to_string(),
                    3 => "火力 LV.3 — MAX".to_string(),
                    _ => "补给".to_string(),
                };
                state.message_clock = 1.4;
                break;
            }
        }
    }
}

fn aabb(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    (a.x - b.x).abs() * 2.0 < a_size.x + b_size.x
        && (a.y - b.y).abs() * 2.0 < a_size.y + b_size.y
}

fn spawn_powerup(commands: &mut Commands, pos: Vec2) {
    commands
        .spawn((
            Sprite::from_color(Color::srgb(1.0, 0.86, 0.32), Vec2::new(22.0, 22.0)),
            Transform::from_translation(pos.extend(Z_POWERUP)),
            GameEntity,
            SpacePowerUp,
            Collider { size: Vec2::splat(22.0) },
        ))
        .with_children(|p| {
            p.spawn((
                Sprite::from_color(Color::srgb(0.88, 0.32, 0.32), Vec2::new(14.0, 14.0)),
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.96, 0.78), Vec2::new(4.0, 10.0)),
                Transform::from_xyz(0.0, 0.0, 0.02),
            ));
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.96, 0.78), Vec2::new(10.0, 4.0)),
                Transform::from_xyz(0.0, 0.0, 0.02),
            ));
        });
}

fn spawn_explosion(commands: &mut Commands, pos: Vec2, big: bool) {
    let count = if big { 14 } else { 6 };
    let mut rng = thread_rng();
    let base_color = if big {
        Color::srgb(1.0, 0.65, 0.28)
    } else {
        Color::srgb(1.0, 0.78, 0.42)
    };
    // 中心爆光
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

pub(super) fn space_collisions(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut state: ResMut<SpaceState>,
    mut enemies: Query<
        (Entity, &Transform, &Collider, &mut SpaceEnemy),
        (Without<SpaceShipPlayer>, Without<SpaceBullet>),
    >,
    bullets: Query<
        (Entity, &Transform, &Collider, &SpaceBullet),
        (Without<SpaceShipPlayer>, Without<SpaceEnemy>),
    >,
    mut players: Query<
        (Entity, &mut Transform, &Collider, &mut SpaceShipPlayer),
        (Without<SpaceEnemy>, Without<SpaceBullet>),
    >,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }

    // 子弹 vs 敌机 / 玩家
    for (be, bt, bc, b) in &bullets {
        if b.from_player {
            for (ee, et, ec, mut enemy) in &mut enemies {
                if aabb(
                    bt.translation.truncate(),
                    bc.size,
                    et.translation.truncate(),
                    ec.size,
                ) {
                    enemy.hp -= b.damage;
                    commands.entity(be).despawn();
                    if enemy.hp <= 0 {
                        session.score += enemy.points;
                        let pos = et.translation.truncate();
                        let big = matches!(enemy.kind, EnemyKind::Bomber | EnemyKind::Boss);
                        spawn_explosion(&mut commands, pos, big);
                        if enemy.drops_power {
                            spawn_powerup(&mut commands, pos);
                        }
                        if enemy.kind == EnemyKind::Boss {
                            state.boss_defeated = true;
                            state.message = "BOSS 已摧毁！".to_string();
                            state.message_clock = 3.0;
                            finish_space(&mut session, &mut save, true);
                        }
                        commands.entity(ee).despawn();
                    }
                    break;
                }
            }
        } else {
            // 敌人子弹击中玩家
            for (pe, mut pt, pc, mut player) in &mut players {
                if player.invincible_left > 0.0 {
                    continue;
                }
                if aabb(
                    bt.translation.truncate(),
                    bc.size,
                    pt.translation.truncate(),
                    pc.size,
                ) {
                    commands.entity(be).despawn();
                    player_take_hit(
                        &mut commands,
                        &mut session,
                        &mut save,
                        &mut state,
                        &mut player,
                        &mut pt,
                        pe,
                    );
                    break;
                }
            }
        }
    }

    // 敌机撞玩家
    let mut enemies_to_kill: Vec<Entity> = Vec::new();
    for (ee, et, ec, enemy) in &enemies {
        for (pe, mut pt, pc, mut player) in &mut players {
            if player.invincible_left > 0.0 {
                continue;
            }
            if aabb(
                et.translation.truncate(),
                ec.size,
                pt.translation.truncate(),
                pc.size,
            ) {
                if enemy.kind != EnemyKind::Boss {
                    enemies_to_kill.push(ee);
                    spawn_explosion(&mut commands, et.translation.truncate(), false);
                }
                player_take_hit(
                    &mut commands,
                    &mut session,
                    &mut save,
                    &mut state,
                    &mut player,
                    &mut pt,
                    pe,
                );
                break;
            }
        }
    }
    for e in enemies_to_kill {
        commands.entity(e).despawn();
    }
}

fn player_take_hit(
    commands: &mut Commands,
    session: &mut GameSession,
    save: &mut SaveData,
    state: &mut SpaceState,
    player: &mut SpaceShipPlayer,
    player_t: &mut Transform,
    player_entity: Entity,
) {
    spawn_explosion(commands, player_t.translation.truncate(), true);
    session.lives -= 1;
    if session.lives < 0 {
        session.lives = 0;
        finish_space(session, save, false);
        commands.entity(player_entity).despawn();
        return;
    }
    state.power = state.power.saturating_sub(1).max(1);
    state.message = "再来一次！".to_string();
    state.message_clock = 1.2;
    player_t.translation.x = PLAYER_RESPAWN_X;
    player_t.translation.y = PLAYER_RESPAWN_Y;
    player.invincible_left = PLAYER_INVINCIBLE;
}

pub(super) fn space_player_blink(
    session: Res<GameSession>,
    mut players: Query<(&SpaceShipPlayer, &mut Sprite)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (p, mut sprite) in &mut players {
        if p.invincible_left > 0.0 {
            let visible = (p.blink_phase.sin() > 0.0) as i32 as f32;
            sprite
                .color
                .set_alpha(0.35 + visible * 0.55);
        } else {
            sprite.color.set_alpha(1.0);
        }
    }
}

pub(super) fn space_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut life: Query<(Entity, &mut Lifetime)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    for (e, mut l) in &mut life {
        l.0.tick(time.delta());
        if l.0.just_finished() {
            commands.entity(e).despawn();
        }
    }
}

pub(super) fn space_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    state: Res<SpaceState>,
    enemies: Query<&SpaceEnemy>,
    mut hud: Query<&mut Text2d, (With<SpaceHud>, Without<SpaceMessageText>)>,
    mut message: Query<&mut Text2d, (With<SpaceMessageText>, Without<SpaceHud>)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let high = save.high_scores[GameKind::SpaceShooter.index()].max(session.score);
        let boss_line = if state.boss_spawned && !state.boss_defeated {
            let boss_hp = enemies
                .iter()
                .find(|e| e.kind == EnemyKind::Boss)
                .map(|e| e.hp.max(0))
                .unwrap_or(0);
            format!(
                "BOSS: {} / {}",
                boss_hp,
                state.boss_hp_max.max(1)
            )
        } else {
            String::new()
        };
        **t = format!(
            "分数 {}\n纪录 {}\n生命 {}\n火力 LV.{}\n第 {} 关\n{}",
            session.score,
            high,
            session.lives.max(0),
            state.power,
            session.level,
            boss_line
        );
    }
    if let Ok(mut t) = message.single_mut() {
        **t = if session.finished {
            if session.won {
                "通关！Enter 重玩，Esc 返回".to_string()
            } else {
                "再试一次！Enter 重玩，Esc 返回".to_string()
            }
        } else if session.paused {
            "已暂停：Esc 继续，Backspace 返回".to_string()
        } else if state.message_clock > 0.0 {
            state.message.clone()
        } else {
            String::new()
        };
    }
}

fn finish_space(session: &mut GameSession, save: &mut SaveData, won: bool) {
    if session.finished {
        return;
    }
    session.finished = true;
    session.won = won;
    if won {
        session.score += 1000;
        let idx = GameKind::SpaceShooter.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else {
        session.status = "战机被击毁……Enter 重试，Esc 返回".to_string();
    }
}
