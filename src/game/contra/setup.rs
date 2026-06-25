use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::common::render::{UiFont, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;
use super::resources::*;
use super::setup_actors::spawn_player;
use super::setup_terrain::{
    spawn_background, spawn_bridge, spawn_decor, spawn_ground_run, spawn_platform, spawn_water,
};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8, top_score: u32) {
    spawn_background(commands, stage_sky(level));
    spawn_decor(commands);

    // FC 1-1 地形：左侧起点水域 + 陆地段之间用可踏木桥连接
    spawn_water(commands, 0.0, 110.0);
    spawn_ground_run(commands, 110.0, 1100.0, GROUND_TOP);
    spawn_bridge(commands, 1100.0, 1280.0);
    spawn_ground_run(commands, 1280.0, 2280.0, GROUND_TOP);
    spawn_bridge(commands, 2280.0, 2480.0);
    spawn_ground_run(commands, 2480.0, 3700.0, GROUND_TOP);
    spawn_ground_run(commands, 3700.0, WORLD_W, GROUND_TOP);

    // 高处岩石平台
    spawn_platform(commands, Vec2::new(560.0, GROUND_TOP + TILE * 4.0), 144.0);
    spawn_platform(commands, Vec2::new(820.0, GROUND_TOP + TILE * 6.0), 96.0);
    spawn_platform(commands, Vec2::new(1660.0, GROUND_TOP + TILE * 3.5), 192.0);
    spawn_platform(commands, Vec2::new(1900.0, GROUND_TOP + TILE * 6.0), 120.0);
    spawn_platform(commands, Vec2::new(2680.0, GROUND_TOP + TILE * 5.0), 168.0);
    spawn_platform(commands, Vec2::new(3220.0, GROUND_TOP + TILE * 4.0), 200.0);
    spawn_platform(commands, Vec2::new(3520.0, GROUND_TOP + TILE * 6.5), 120.0);

    // 玩家
    let player_spawn = Vec2::new(200.0, GROUND_TOP + PLAYER_H * 0.5);
    spawn_player(commands, player_spawn);

    let falcon_marks = build_falcon_marks();
    let spawn_marks = build_enemy_marks(level);

    spawn_hud(commands, font, level, top_score);

    let stage = ContraStage {
        level,
        world_w: WORLD_W,
        player_spawn,
        spawn_marks,
        spawn_idx: 0,
        falcon_marks,
        falcon_idx: 0,
        boss_x: 4040.0,
        boss_hp: stage_boss_hp(level),
        boss_spawned: false,
        boss_dead: false,
        top_score,
    };
    commands.insert_resource(stage);
}

fn build_falcon_marks() -> Vec<FalconMark> {
    vec![
        FalconMark {
            trigger_x: 280.0,
            start: Vec2::new(0.0 - 80.0, GROUND_TOP + TILE * 7.0),
            vx: FALCON_SPEED,
            weapon: Weapon::S,
        },
        FalconMark {
            trigger_x: 1300.0,
            start: Vec2::new(1100.0, GROUND_TOP + TILE * 8.0),
            vx: FALCON_SPEED,
            weapon: Weapon::R,
        },
        FalconMark {
            trigger_x: 2500.0,
            start: Vec2::new(2300.0, GROUND_TOP + TILE * 7.5),
            vx: FALCON_SPEED,
            weapon: Weapon::F,
        },
        FalconMark {
            trigger_x: 3120.0,
            start: Vec2::new(2900.0, GROUND_TOP + TILE * 7.0),
            vx: FALCON_SPEED,
            weapon: Weapon::L,
        },
    ]
}

/// 关卡天空主题色 [底, 中, 顶]：丛林夜 / 雪原冷调 / 要塞暗红，按关卡循环。
fn stage_sky(level: u8) -> [Color; 3] {
    match (level.max(1) - 1) % 3 {
        0 => [COLOR_SKY, COLOR_SKY_MID, COLOR_SKY_HI],
        1 => [
            Color::srgb(0.10, 0.13, 0.20),
            Color::srgb(0.18, 0.23, 0.33),
            Color::srgb(0.30, 0.38, 0.50),
        ],
        _ => [
            Color::srgb(0.10, 0.06, 0.07),
            Color::srgb(0.18, 0.10, 0.11),
            Color::srgb(0.30, 0.16, 0.15),
        ],
    }
}

/// Boss 血量随关卡线性增长。
fn stage_boss_hp(level: u8) -> i32 {
    BOSS_HP + (level as i32 - 1) * 8
}

/// 1 关沿用手工编排，2 关起按关卡程序化加压（更密、重型/机枪手更多）。
fn build_enemy_marks(level: u8) -> Vec<EnemySpawnMark> {
    if level <= 1 {
        build_enemy_marks_l1()
    } else {
        build_enemy_marks_proc(level)
    }
}

fn pick_enemy_kind(i: i32, level: u8) -> EnemyKind {
    let pool: &[EnemyKind] = if level == 2 {
        &[
            EnemyKind::Soldier,
            EnemyKind::Sniper,
            EnemyKind::Gunner,
            EnemyKind::Soldier,
            EnemyKind::Jumper,
            EnemyKind::Heavy,
        ]
    } else {
        &[
            EnemyKind::Soldier,
            EnemyKind::Gunner,
            EnemyKind::Heavy,
            EnemyKind::Sniper,
            EnemyKind::Jumper,
            EnemyKind::Gunner,
            EnemyKind::Heavy,
        ]
    };
    pool[(i as usize) % pool.len()]
}

fn build_enemy_marks_proc(level: u8) -> Vec<EnemySpawnMark> {
    let count = (16 + level as i32 * 3).min(42);
    let x0 = 360.0;
    let x1 = WORLD_W - 500.0;
    let span = x1 - x0;
    let denom = (count - 1).max(1) as f32;
    (0..count)
        .map(|i| {
            let x = x0 + (i as f32 / denom) * span;
            EnemySpawnMark {
                trigger_x: (x - 340.0).max(0.0),
                pos: Vec2::new(x, GROUND_TOP + ENEMY_H * 0.5),
                kind: pick_enemy_kind(i, level),
                facing: -1.0,
            }
        })
        .collect()
}

fn build_enemy_marks_l1() -> Vec<EnemySpawnMark> {
    let mk = |trigger_x: f32, pos: Vec2, kind: EnemyKind| EnemySpawnMark {
        trigger_x,
        pos,
        kind,
        facing: -1.0,
    };
    vec![
        // 段 1
        mk(200.0, Vec2::new(360.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(280.0, Vec2::new(560.0, GROUND_TOP + TILE * 4.0 + ENEMY_H * 0.5), EnemyKind::Sniper),
        mk(360.0, Vec2::new(740.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(480.0, Vec2::new(820.0, GROUND_TOP + TILE * 6.0 + ENEMY_H * 0.5), EnemyKind::Jumper),
        // 段 2 之前 (跨水)
        mk(680.0, Vec2::new(960.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(880.0, Vec2::new(1060.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        // 段 2
        mk(1100.0, Vec2::new(1380.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(1280.0, Vec2::new(1660.0, GROUND_TOP + TILE * 3.5 + ENEMY_H * 0.5), EnemyKind::Sniper),
        mk(1380.0, Vec2::new(1780.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(1500.0, Vec2::new(1900.0, GROUND_TOP + TILE * 6.0 + ENEMY_H * 0.5), EnemyKind::Jumper),
        mk(1700.0, Vec2::new(2080.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(1840.0, Vec2::new(2200.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        // 段 3
        mk(2200.0, Vec2::new(2520.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(2360.0, Vec2::new(2680.0, GROUND_TOP + TILE * 5.0 + ENEMY_H * 0.5), EnemyKind::Sniper),
        mk(2520.0, Vec2::new(2820.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(2680.0, Vec2::new(2960.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(2800.0, Vec2::new(3100.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(2960.0, Vec2::new(3220.0, GROUND_TOP + TILE * 4.0 + ENEMY_H * 0.5), EnemyKind::Sniper),
        mk(3060.0, Vec2::new(3360.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
        mk(3180.0, Vec2::new(3520.0, GROUND_TOP + TILE * 6.5 + ENEMY_H * 0.5), EnemyKind::Jumper),
        mk(3260.0, Vec2::new(3600.0, GROUND_TOP + ENEMY_H * 0.5), EnemyKind::Soldier),
    ]
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8, top_score: u32) {
    let hud_z = 9.0_f32;
    let top_y = ARENA_H * 0.5 - 14.0;
    let bar_off = Vec2::new(0.0, top_y);
    let bar_size = Vec2::new(ARENA_W, 28.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), bar_size),
            Transform::from_translation(bar_off.extend(hud_z)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: bar_off });
    let line_off = Vec2::new(0.0, top_y - 14.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.85, 0.85, 0.85), Vec2::new(ARENA_W, 1.0)),
            Transform::from_translation(line_off.extend(hud_z + 0.05)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: line_off });

    let top_off = Vec2::new(-ARENA_W * 0.5 + 96.0, top_y);
    text(
        commands,
        font,
        &format!("TOP-{:06}", top_score),
        top_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::TopScore,
        offset: top_off,
    });

    let score_off = Vec2::new(-ARENA_W * 0.5 + 232.0, top_y);
    text(
        commands,
        font,
        "1P-000000",
        score_off,
        14.0,
        Color::srgb(1.0, 0.95, 0.30),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Score,
        offset: score_off,
    });

    let world_off = Vec2::new(0.0, top_y);
    text(
        commands,
        font,
        &format!("STAGE 1-{}", level),
        world_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::World,
        offset: world_off,
    });

    let lives_label_off = Vec2::new(110.0, top_y);
    text(
        commands,
        font,
        "x3",
        lives_label_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Lives,
        offset: lives_label_off,
    });
    for i in 0..4 {
        let icon_off = Vec2::new(140.0 + i as f32 * 14.0, top_y);
        spawn_life_icon(commands, icon_off, hud_z + 0.2, i);
    }

    let weapon_off = Vec2::new(ARENA_W * 0.5 - 80.0, top_y);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(20.0, 20.0)),
            Transform::from_translation(weapon_off.extend(hud_z + 0.1)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: weapon_off });
    commands
        .spawn((
            Sprite::from_color(COLOR_PICKUP_M, Vec2::new(16.0, 16.0)),
            Transform::from_translation(weapon_off.extend(hud_z + 0.15)),
            GameEntity,
        ))
        .insert(ContraHud {
            kind: ContraHudKind::Weapon,
            offset: weapon_off,
        });
    let weapon_letter_off = Vec2::new(ARENA_W * 0.5 - 80.0, top_y);
    text(
        commands,
        font,
        "M",
        weapon_letter_off,
        14.0,
        Color::srgb(0.0, 0.0, 0.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::WeaponLetter,
        offset: weapon_letter_off,
    });

    let status_off = Vec2::new(0.0, 0.0);
    text(
        commands,
        font,
        "",
        status_off,
        22.0,
        Color::srgb(1.0, 0.95, 0.30),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Status,
        offset: status_off,
    });

    let boss_off = Vec2::new(ARENA_W * 0.5 - 30.0, top_y);
    text(
        commands,
        font,
        "",
        boss_off,
        12.0,
        Color::srgb(1.0, 0.4, 0.32),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::BossHp,
        offset: boss_off,
    });
}

fn spawn_life_icon(commands: &mut Commands, offset: Vec2, z: f32, idx: i32) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(12.0, 12.0)),
            Transform::from_translation(offset.extend(z)),
            ContraHudLifeIcon { idx },
            ContraHudPanel { offset },
            GameEntity,
        ))
        .id();
    // 小 Bill 头像：棕发 + 裸肩，与新版立绘一致
    let parts: [(f32, f32, f32, f32, Color); 5] = [
        (0.0, 4.0, 8.0, 2.0, COLOR_PLAYER_HAIR),
        (-3.0, 3.0, 2.0, 2.0, COLOR_PLAYER_HAIR),
        (0.0, 0.5, 8.0, 4.0, COLOR_PLAYER_SKIN),
        (2.0, 1.0, 1.0, 1.0, COLOR_PLAYER_OUTLINE),
        (0.0, -3.0, 9.0, 2.0, COLOR_PLAYER_SKIN),
    ];
    for (dx, dy, w, h, color) in parts {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_marks_scale_and_stay_in_bounds() {
        for level in 2u8..=8 {
            let marks = build_enemy_marks(level);
            assert!(!marks.is_empty(), "level {level} 无敌人");
            for m in &marks {
                assert!(
                    m.pos.x >= 300.0 && m.pos.x <= WORLD_W - 400.0,
                    "level {level} 敌人 x={} 越界",
                    m.pos.x
                );
            }
        }
        assert!(
            build_enemy_marks(6).len() > build_enemy_marks(2).len(),
            "关卡越高敌人应越多"
        );
    }

    #[test]
    fn high_level_uses_all_enemy_kinds() {
        let kinds: Vec<EnemyKind> = build_enemy_marks(3).iter().map(|m| m.kind).collect();
        for k in [
            EnemyKind::Soldier,
            EnemyKind::Sniper,
            EnemyKind::Jumper,
            EnemyKind::Heavy,
            EnemyKind::Gunner,
        ] {
            assert!(kinds.contains(&k), "3 关应出现 {k:?}");
        }
    }

    #[test]
    fn boss_hp_grows_with_level() {
        assert_eq!(stage_boss_hp(1), BOSS_HP);
        assert!(stage_boss_hp(5) > stage_boss_hp(1));
    }

    #[test]
    fn level_one_keeps_handcrafted_layout() {
        assert_eq!(build_enemy_marks(1).len(), build_enemy_marks_l1().len());
    }
}
