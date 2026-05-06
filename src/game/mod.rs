use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::prelude::*;
use std::time::Duration;

mod bomb_maze;
mod model;
mod rules;
mod spawn;
mod tank;

use crate::common::constants::{ARENA_H, ARENA_W, WINDOW_H, WINDOW_W};
use crate::common::input::input_for;
use crate::common::render::{UiFont, background_rect, panel, rect, text};
use model::*;
use rules::{aabb, bomb_tiles, bubble_cluster, circle_aabb};
use spawn::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BaoGames".into(),
                resolution: WindowResolution::new(WINDOW_W, WINDOW_H),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(SelectedGame(GameKind::Tank))
        .insert_resource(SaveData::load())
        .init_resource::<UiFont>()
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup::<MenuEntity>)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            (
                pause_and_result_input,
                player_input,
                tick_lifetimes,
                update_projectiles,
                update_bombs,
                update_enemies,
                apply_gravity,
                collision_system,
                update_hud,
                check_level_end,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            (
                tank::tank_player_input,
                tank::tank_enemy_ai,
                tank::tank_movement,
                tank::tank_bullet_update,
                tank::tank_enemy_spawner,
                tank::tank_spawn_effect,
                tank::tank_player_respawn,
                tank::tank_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            (
                bomb_maze::bm_player_input,
                bomb_maze::bm_enemy_ai,
                bomb_maze::bm_bomb_tick,
                bomb_maze::bm_flame_tick,
                bomb_maze::bm_flame_damage,
                bomb_maze::bm_enemy_touch,
                bomb_maze::bm_powerup_pickup,
                bomb_maze::bm_exit_and_respawn,
                bomb_maze::bm_player_blink,
                bomb_maze::bm_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnExit(AppState::Playing), cleanup::<GameEntity>)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 把相机视野锁定为逻辑 ARENA 区域，窗口被放大时画面等比放大
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMin {
                min_width: ARENA_W,
                min_height: ARENA_H,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_menu(mut commands: Commands, save: Res<SaveData>, font: Res<UiFont>) {
    // 渐变感的双层背景
    background_rect(
        &mut commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W, ARENA_H),
        Color::srgb(0.04, 0.05, 0.08),
        MenuEntity,
    );
    background_rect(
        &mut commands,
        Vec2::new(0.0, -40.0),
        Vec2::new(ARENA_W, ARENA_H * 0.55),
        Color::srgb(0.06, 0.08, 0.13),
        MenuEntity,
    );
    // 细网格
    let half_w = (ARENA_W * 0.5) as i32;
    let half_h = (ARENA_H * 0.5) as i32;
    for x in (-half_w..=half_w).step_by(40) {
        rect(
            &mut commands,
            Vec2::new(x as f32, 0.0),
            Vec2::new(1.0, ARENA_H),
            Color::srgb(0.08, 0.1, 0.14),
            MenuEntity,
        );
    }
    for y in (-half_h..=half_h).step_by(40) {
        rect(
            &mut commands,
            Vec2::new(0.0, y as f32),
            Vec2::new(ARENA_W, 1.0),
            Color::srgb(0.08, 0.1, 0.14),
            MenuEntity,
        );
    }

    // 标题徽章
    panel(
        &mut commands,
        Vec2::new(0.0, 215.0),
        Vec2::new(560.0, 70.0),
        Color::srgb(0.13, 0.18, 0.28),
        Color::srgb(0.95, 0.78, 0.32),
        MenuEntity,
    );
    rect(
        &mut commands,
        Vec2::new(-250.0, 215.0),
        Vec2::new(20.0, 56.0),
        Color::srgb(0.95, 0.78, 0.32),
        MenuEntity,
    );
    rect(
        &mut commands,
        Vec2::new(250.0, 215.0),
        Vec2::new(20.0, 56.0),
        Color::srgb(0.95, 0.78, 0.32),
        MenuEntity,
    );
    text(
        &mut commands,
        &font,
        "BaoGames 儿童小游戏合集",
        Vec2::new(0.0, 215.0),
        34.0,
        Color::srgb(1.0, 0.94, 0.62),
        MenuEntity,
    );

    // 操作说明
    panel(
        &mut commands,
        Vec2::new(0.0, 162.0),
        Vec2::new(720.0, 30.0),
        Color::srgb(0.09, 0.12, 0.18),
        Color::srgb(0.32, 0.45, 0.6),
        MenuEntity,
    );
    text(
        &mut commands,
        &font,
        "按 1-6 选择游戏，Enter 开始 / Esc 暂停",
        Vec2::new(0.0, 162.0),
        17.0,
        Color::srgb(0.85, 0.92, 1.0),
        MenuEntity,
    );

    // P1/P2 操作图例
    panel(
        &mut commands,
        Vec2::new(-180.0, 128.0),
        Vec2::new(330.0, 28.0),
        Color::srgb(0.14, 0.13, 0.06),
        Color::srgb(0.85, 0.78, 0.36),
        MenuEntity,
    );
    text(
        &mut commands,
        &font,
        "P1：WASD 移动  J 射击  K 跳跃",
        Vec2::new(-180.0, 128.0),
        15.0,
        Color::srgb(1.0, 0.95, 0.75),
        MenuEntity,
    );
    panel(
        &mut commands,
        Vec2::new(180.0, 128.0),
        Vec2::new(330.0, 28.0),
        Color::srgb(0.06, 0.1, 0.16),
        Color::srgb(0.46, 0.7, 0.95),
        MenuEntity,
    );
    text(
        &mut commands,
        &font,
        "P2：方向键 移动  小键盘1 射击 2 跳跃",
        Vec2::new(180.0, 128.0),
        15.0,
        Color::srgb(0.78, 0.92, 1.0),
        MenuEntity,
    );

    // 游戏选项卡片：两列三行
    let card_w = 360.0;
    let card_h = 70.0;
    for (i, kind) in GameKind::ALL.iter().enumerate() {
        let col = (i % 2) as f32;
        let row = (i / 2) as f32;
        let cx = -190.0 + col * 380.0;
        let cy = 70.0 - row * 78.0;
        let accent = menu_accent(*kind);
        panel(
            &mut commands,
            Vec2::new(cx, cy),
            Vec2::new(card_w, card_h),
            Color::srgb(0.1, 0.13, 0.19),
            accent,
            MenuEntity,
        );
        // 左侧色条
        rect(
            &mut commands,
            Vec2::new(cx - card_w * 0.5 + 14.0, cy),
            Vec2::new(8.0, card_h - 16.0),
            accent,
            MenuEntity,
        );
        // 数字徽标
        rect(
            &mut commands,
            Vec2::new(cx - card_w * 0.5 + 38.0, cy),
            Vec2::new(28.0, 28.0),
            Color::srgba(accent.to_srgba().red, accent.to_srgba().green, accent.to_srgba().blue, 0.35),
            MenuEntity,
        );
        text(
            &mut commands,
            &font,
            &format!("{}", i + 1),
            Vec2::new(cx - card_w * 0.5 + 38.0, cy),
            22.0,
            Color::WHITE,
            MenuEntity,
        );
        // 标题（去掉前缀数字）
        let title_only = kind.title().splitn(2, ' ').nth(1).unwrap_or(kind.title());
        text(
            &mut commands,
            &font,
            title_only,
            Vec2::new(cx + 8.0, cy + 12.0),
            20.0,
            Color::srgb(1.0, 0.96, 0.86),
            MenuEntity,
        );
        text(
            &mut commands,
            &font,
            &format!(
                "最高分 {}   已解锁第 {} 关",
                save.high_scores[kind.index()],
                save.unlocked_levels[kind.index()]
            ),
            Vec2::new(cx + 8.0, cy - 14.0),
            14.0,
            Color::srgb(0.78, 0.85, 0.94),
            MenuEntity,
        );
    }

    // 底部提示
    text(
        &mut commands,
        &font,
        "选好就按 Enter 出发吧！",
        Vec2::new(0.0, -200.0),
        17.0,
        Color::srgb(0.55, 0.7, 0.86),
        MenuEntity,
    );
}

fn menu_accent(kind: GameKind) -> Color {
    match kind {
        GameKind::Tank => Color::srgb(0.35, 0.78, 0.42),
        GameKind::BombMaze => Color::srgb(0.95, 0.58, 0.24),
        GameKind::BubbleShooter => Color::srgb(0.9, 0.34, 0.78),
        GameKind::PlaneShooter => Color::srgb(0.36, 0.72, 1.0),
        GameKind::RunGun => Color::srgb(0.92, 0.35, 0.28),
        GameKind::Platformer => Color::srgb(0.98, 0.82, 0.22),
    }
}

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedGame>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let choices = [
        (KeyCode::Digit1, GameKind::Tank),
        (KeyCode::Digit2, GameKind::BombMaze),
        (KeyCode::Digit3, GameKind::BubbleShooter),
        (KeyCode::Digit4, GameKind::PlaneShooter),
        (KeyCode::Digit5, GameKind::RunGun),
        (KeyCode::Digit6, GameKind::Platformer),
    ];
    for (key, kind) in choices {
        if keys.just_pressed(key) {
            selected.0 = kind;
            next_state.set(AppState::Playing);
        }
    }
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::Playing);
    }
}

fn paint_stage_backdrop(commands: &mut Commands, kind: GameKind) {
    let (base, grid, border) = match kind {
        GameKind::Tank => (
            Color::srgb(0.07, 0.12, 0.08),
            Color::srgb(0.14, 0.22, 0.13),
            Color::srgb(0.44, 0.58, 0.32),
        ),
        GameKind::BombMaze => (
            Color::srgb(0.09, 0.08, 0.1),
            Color::srgb(0.18, 0.16, 0.2),
            Color::srgb(0.76, 0.52, 0.28),
        ),
        GameKind::BubbleShooter => (
            Color::srgb(0.08, 0.09, 0.17),
            Color::srgb(0.12, 0.16, 0.28),
            Color::srgb(0.88, 0.45, 0.78),
        ),
        GameKind::PlaneShooter => (
            Color::srgb(0.05, 0.08, 0.16),
            Color::srgb(0.08, 0.16, 0.28),
            Color::srgb(0.38, 0.72, 1.0),
        ),
        GameKind::RunGun => (
            Color::srgb(0.13, 0.09, 0.08),
            Color::srgb(0.24, 0.15, 0.12),
            Color::srgb(0.82, 0.38, 0.26),
        ),
        GameKind::Platformer => (
            Color::srgb(0.08, 0.13, 0.18),
            Color::srgb(0.1, 0.2, 0.24),
            Color::srgb(0.92, 0.78, 0.28),
        ),
    };

    background_rect(
        commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W, ARENA_H),
        base,
        GameEntity,
    );
    let half_w = (ARENA_W * 0.5) as i32;
    let half_h = (ARENA_H * 0.5) as i32;
    for x in (-half_w..=half_w).step_by(48) {
        rect(
            commands,
            Vec2::new(x as f32, 0.0),
            Vec2::new(2.0, ARENA_H),
            grid,
            GameEntity,
        );
    }
    for y in (-half_h..=half_h).step_by(48) {
        rect(
            commands,
            Vec2::new(0.0, y as f32),
            Vec2::new(ARENA_W, 2.0),
            grid,
            GameEntity,
        );
    }
    panel(
        commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W - 28.0, ARENA_H - 28.0),
        Color::srgba(0.0, 0.0, 0.0, 0.0),
        border,
        GameEntity,
    );
}

fn setup_game(
    mut commands: Commands,
    selected: Res<SelectedGame>,
    save: Res<SaveData>,
    font: Res<UiFont>,
) {
    let level = save.unlocked_levels[selected.0.index()].clamp(1, 10);
    commands.insert_resource(GameSession {
        kind: selected.0,
        level,
        score: 0,
        lives: 3,
        paused: false,
        finished: false,
        won: false,
        time_left: match selected.0 {
            GameKind::PlaneShooter => 65.0,
            GameKind::BubbleShooter => 90.0,
            _ => 120.0,
        },
        spawn_clock: 0.0,
        player_cooldown: [0.0, 0.0],
        status: selected.0.goal_text().to_string(),
    });

    paint_stage_backdrop(&mut commands, selected.0);
    if !matches!(selected.0, GameKind::Tank | GameKind::BombMaze) {
        text(
            &mut commands,
            &font,
            "",
            Vec2::new(0.0, ARENA_H * 0.5 - 34.0),
            22.0,
            Color::WHITE,
            GameEntity,
        )
        .insert(HudText);
    }

    match selected.0 {
        GameKind::Tank => tank::setup_stage(&mut commands, &font, level),
        GameKind::BombMaze => bomb_maze::setup_stage(&mut commands, &font, level),
        GameKind::BubbleShooter => setup_bubble_shooter(&mut commands, level),
        GameKind::PlaneShooter => setup_plane_shooter(&mut commands, level),
        GameKind::RunGun => setup_run_gun(&mut commands, level),
        GameKind::Platformer => setup_platformer(&mut commands, level),
    }
}

fn setup_bubble_shooter(commands: &mut Commands, level: u8) {
    spawn_player(
        commands,
        0,
        Vec2::new(0.0, -215.0),
        Color::srgb(0.4, 0.85, 0.7),
    );
    let rows = 5 + (level as usize / 2);
    for row in 0..rows {
        for col in 0..11 {
            let x = -330.0 + col as f32 * 66.0 + if row % 2 == 0 { 0.0 } else { 33.0 };
            let y = 190.0 - row as f32 * 48.0;
            let color_id = (row + col + level as usize) % 4;
            spawn_bubble(commands, Vec2::new(x, y), color_id);
        }
    }
}

fn setup_plane_shooter(commands: &mut Commands, level: u8) {
    spawn_player(
        commands,
        0,
        Vec2::new(-60.0, -200.0),
        Color::srgb(0.35, 0.78, 0.98),
    );
    spawn_player(
        commands,
        1,
        Vec2::new(60.0, -200.0),
        Color::srgb(0.5, 0.9, 0.42),
    );
    for i in 0..(6 + level.min(6)) {
        spawn_enemy(
            commands,
            Vec2::new(-380.0 + i as f32 * 85.0, 170.0 + (i % 2) as f32 * 45.0),
            1,
            100,
            Color::srgb(0.9, 0.38, 0.55),
        );
    }
}

fn setup_run_gun(commands: &mut Commands, level: u8) {
    spawn_runner(
        commands,
        0,
        Vec2::new(-410.0, -140.0),
        Color::srgb(0.25, 0.78, 0.5),
    );
    spawn_runner(
        commands,
        1,
        Vec2::new(-350.0, -140.0),
        Color::srgb(0.35, 0.66, 0.95),
    );
    spawn_platforms(commands);
    spawn_goal(
        commands,
        Vec2::new(420.0, -120.0),
        Vec2::new(35.0, 120.0),
        Color::srgb(0.98, 0.74, 0.3),
    );
    for i in 0..(5 + level.min(5)) {
        spawn_enemy(
            commands,
            Vec2::new(-140.0 + i as f32 * 115.0, -100.0 + (i % 2) as f32 * 85.0),
            1,
            130,
            Color::srgb(0.84, 0.34, 0.33),
        );
    }
}

fn setup_platformer(commands: &mut Commands, level: u8) {
    spawn_runner(
        commands,
        0,
        Vec2::new(-420.0, -145.0),
        Color::srgb(0.35, 0.81, 0.5),
    );
    spawn_runner(
        commands,
        1,
        Vec2::new(-360.0, -145.0),
        Color::srgb(0.37, 0.68, 0.96),
    );
    spawn_platforms(commands);
    spawn_goal(
        commands,
        Vec2::new(430.0, 110.0),
        Vec2::new(42.0, 150.0),
        Color::srgb(0.96, 0.78, 0.3),
    );
    for i in 0..(8 + level.min(6)) {
        rect(
            commands,
            Vec2::new(-250.0 + i as f32 * 90.0, -50.0 + (i % 3) as f32 * 75.0),
            Vec2::splat(24.0),
            Color::srgb(1.0, 0.86, 0.24),
            GameEntity,
        )
        .insert((
            Collectible,
            Collider {
                size: Vec2::splat(28.0),
            },
        ));
    }
}

fn pause_and_result_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    selected: Res<SelectedGame>,
    save: Res<SaveData>,
    font: Res<UiFont>,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if session.finished {
            next_state.set(AppState::Menu);
        } else {
            session.paused = !session.paused;
            session.status = if session.paused {
                "已暂停：Esc 继续，Backspace 返回菜单".to_string()
            } else {
                selected.0.goal_text().to_string()
            };
        }
    }
    if keys.just_pressed(KeyCode::Backspace) {
        next_state.set(AppState::Menu);
    }
    if session.finished && keys.just_pressed(KeyCode::Enter) {
        for entity in &game_entities {
            commands.entity(entity).despawn();
        }
        setup_game(commands, selected, save, font);
    }
}

fn player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<GameSession>,
    mut players: Query<(
        &Player,
        &mut Transform,
        Option<&mut Velocity>,
        Option<&mut Grounded>,
    )>,
) {
    if matches!(session.kind, GameKind::Tank | GameKind::BombMaze) {
        return;
    }
    if session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    session.time_left -= delta;
    for cooldown in &mut session.player_cooldown {
        *cooldown = (*cooldown - delta).max(0.0);
    }

    for (player, mut transform, velocity, grounded) in &mut players {
        let input = input_for(&keys, player.id);
        let mut dir = input.move_dir;
        if dir.length_squared() > 1.0 {
            dir = dir.normalize();
        }

        match session.kind {
            GameKind::Tank | GameKind::BombMaze | GameKind::PlaneShooter => {
                let speed = if session.kind == GameKind::PlaneShooter {
                    260.0
                } else {
                    185.0
                };
                transform.translation.x += dir.x * speed * delta;
                transform.translation.y += dir.y * speed * delta;
                clamp_translation(&mut transform.translation);
                if input.fire && session.player_cooldown[player.id] <= 0.0 {
                    if session.kind == GameKind::BombMaze {
                        spawn_bomb(&mut commands, transform.translation.truncate());
                        session.player_cooldown[player.id] = 1.1;
                    } else {
                        let bullet_dir = if session.kind == GameKind::PlaneShooter {
                            Vec2::Y
                        } else if dir.length_squared() > 0.1 {
                            dir
                        } else {
                            Vec2::Y
                        };
                        spawn_projectile(
                            &mut commands,
                            transform.translation.truncate() + bullet_dir * 28.0,
                            bullet_dir * 520.0,
                            ProjectileOwner::Player,
                            Color::srgb(1.0, 0.9, 0.35),
                        );
                        session.player_cooldown[player.id] = 0.28;
                    }
                }
            }
            GameKind::BubbleShooter => {
                transform.translation.x += dir.x * 260.0 * delta;
                transform.translation.x = transform.translation.x.clamp(-390.0, 390.0);
                if input.fire && session.player_cooldown[player.id] <= 0.0 {
                    let color_id = ((session.score / 50) as usize + player.id) % 4;
                    spawn_bubble_projectile(
                        &mut commands,
                        transform.translation.truncate() + Vec2::Y * 34.0,
                        Vec2::new(dir.x * 160.0, 430.0),
                        color_id,
                    );
                    session.player_cooldown[player.id] = 0.55;
                }
            }
            GameKind::RunGun | GameKind::Platformer => {
                if let Some(mut velocity) = velocity {
                    velocity.x = dir.x * 235.0;
                    if input.jump {
                        if let Some(mut grounded) = grounded {
                            if grounded.0 {
                                velocity.y = 430.0;
                                grounded.0 = false;
                            }
                        }
                    }
                    if input.fire
                        && session.kind == GameKind::RunGun
                        && session.player_cooldown[player.id] <= 0.0
                    {
                        let facing = if dir.x < -0.1 { -1.0 } else { 1.0 };
                        spawn_projectile(
                            &mut commands,
                            transform.translation.truncate() + Vec2::new(facing * 28.0, 6.0),
                            Vec2::new(facing * 540.0, 0.0),
                            ProjectileOwner::Player,
                            Color::srgb(1.0, 0.88, 0.42),
                        );
                        session.player_cooldown[player.id] = 0.33;
                    }
                }
            }
        }
    }
}

fn tick_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
) {
    if session.paused || session.finished {
        return;
    }
    for (entity, mut lifetime) in &mut lifetimes {
        lifetime.0.tick(time.delta());
        if lifetime.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut projectiles: Query<(Entity, &mut Transform, &Velocity), With<Projectile>>,
) {
    if session.paused || session.finished {
        return;
    }
    for (entity, mut transform, velocity) in &mut projectiles {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
        if transform.translation.x.abs() > ARENA_W * 0.55
            || transform.translation.y.abs() > ARENA_H * 0.58
        {
            commands.entity(entity).despawn();
        }
    }
}

fn update_bombs(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bombs: Query<(Entity, &Transform, &mut Bomb)>,
) {
    if matches!(session.kind, GameKind::BombMaze) {
        return;
    }
    if session.paused || session.finished {
        return;
    }
    for (entity, transform, mut bomb) in &mut bombs {
        bomb.timer.tick(time.delta());
        if bomb.timer.just_finished() {
            let center = transform.translation.truncate();
            commands.entity(entity).despawn();
            let tiles = bomb_tiles((0, 0), 2);
            for (tx, ty) in tiles {
                let pos = center + Vec2::new(tx as f32 * 44.0, ty as f32 * 44.0);
                rect(
                    &mut commands,
                    pos,
                    Vec2::splat(40.0),
                    Color::srgb(1.0, 0.58, 0.2),
                    GameEntity,
                )
                .insert((
                    Explosion {},
                    Collider {
                        size: Vec2::splat(42.0),
                    },
                    Lifetime(Timer::new(Duration::from_millis(320), TimerMode::Once)),
                ));
            }
        }
    }
}

fn update_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    players: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&mut Transform, Option<&mut Velocity>), With<Enemy>>,
) {
    if matches!(session.kind, GameKind::Tank | GameKind::BombMaze) {
        return;
    }
    if session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    session.spawn_clock += delta;
    let target = players
        .iter()
        .next()
        .map(|t| t.translation.truncate())
        .unwrap_or(Vec2::ZERO);

    for (mut transform, velocity) in &mut enemies {
        let pos = transform.translation.truncate();
        match session.kind {
            GameKind::PlaneShooter => {
                transform.translation.y -= (85.0 + session.level as f32 * 8.0) * delta;
                transform.translation.x +=
                    (time.elapsed_secs() * 2.0 + pos.y * 0.02).sin() * 75.0 * delta;
                if transform.translation.y < -280.0 {
                    transform.translation.y = 250.0;
                }
            }
            GameKind::RunGun | GameKind::Platformer => {
                if let Some(mut velocity) = velocity {
                    velocity.x = if target.x < pos.x { -80.0 } else { 80.0 };
                }
            }
            _ => {
                let dir = (target - pos).normalize_or_zero();
                transform.translation +=
                    (dir * (55.0 + session.level as f32 * 5.0) * delta).extend(0.0);
            }
        }
    }

    if session.kind == GameKind::PlaneShooter && session.spawn_clock > 2.2 {
        session.spawn_clock = 0.0;
        let mut rng = thread_rng();
        spawn_enemy(
            &mut commands,
            Vec2::new(rng.gen_range(-400.0..400.0), 255.0),
            1,
            100,
            Color::srgb(0.88, 0.38, 0.55),
        );
    }
}

fn apply_gravity(
    time: Res<Time>,
    session: Res<GameSession>,
    mut actors: Query<
        (&mut Transform, &mut Velocity, Option<&mut Grounded>),
        (Without<Projectile>, Without<Wall>),
    >,
    walls: Query<(&Transform, &Collider), (With<Wall>, Without<Player>, Without<Enemy>)>,
) {
    if session.paused || session.finished {
        return;
    }
    if !matches!(session.kind, GameKind::RunGun | GameKind::Platformer) {
        return;
    }
    let delta = time.delta_secs();
    for (mut transform, mut velocity, grounded) in &mut actors {
        velocity.y -= 880.0 * delta;
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta;
        transform.translation.x = transform.translation.x.clamp(-450.0, 450.0);

        let mut new_grounded = false;
        for (wall_t, wall_c) in &walls {
            let pos = transform.translation.truncate();
            let wall_pos = wall_t.translation.truncate();
            let overlap_x = (pos.x - wall_pos.x).abs() < (24.0 + wall_c.size.x * 0.5);
            let above = pos.y >= wall_pos.y;
            let close_y = (pos.y - 18.0 - (wall_pos.y + wall_c.size.y * 0.5)).abs() < 12.0;
            if overlap_x && above && close_y && velocity.y <= 0.0 {
                transform.translation.y = wall_pos.y + wall_c.size.y * 0.5 + 18.0;
                velocity.y = 0.0;
                new_grounded = true;
            }
        }
        if transform.translation.y < -250.0 {
            transform.translation = Vec3::new(-420.0, -145.0, transform.translation.z);
            velocity.0 = Vec2::ZERO;
            new_grounded = true;
        }
        if let Some(mut grounded) = grounded {
            grounded.0 = new_grounded;
        }
    }
}

fn collision_system(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    projectiles: Query<(Entity, &Transform, &Projectile, Option<&Bubble>), Without<Player>>,
    mut enemies: Query<(Entity, &Transform, &Collider, &mut Enemy), Without<Player>>,
    mut players: Query<(&mut Transform, &Collider), With<Player>>,
    walls: Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>,
    soft_blocks: Query<(Entity, &Transform, &Collider), (With<SoftBlock>, Without<Player>)>,
    explosions: Query<(&Transform, &Collider), (With<Explosion>, Without<Player>)>,
    collectibles: Query<(Entity, &Transform, &Collider), (With<Collectible>, Without<Player>)>,
    goals: Query<(&Transform, &Collider), (With<Goal>, Without<Player>)>,
    bubbles: Query<(Entity, &Transform, &Bubble), (Without<Projectile>, Without<Player>)>,
) {
    if matches!(session.kind, GameKind::Tank | GameKind::BombMaze) {
        return;
    }
    if session.paused || session.finished {
        return;
    }

    for (mut player_t, player_c) in &mut players {
        for (wall_t, wall_c) in &walls {
            if aabb(
                player_t.translation.truncate(),
                player_c.size,
                wall_t.translation.truncate(),
                wall_c.size,
            ) {
                let delta = player_t.translation.truncate() - wall_t.translation.truncate();
                if delta.x.abs() > delta.y.abs() {
                    player_t.translation.x += delta.x.signum() * 4.0;
                } else {
                    player_t.translation.y += delta.y.signum() * 4.0;
                }
            }
        }
    }

    for (projectile_e, projectile_t, projectile, bubble) in &projectiles {
        if let Some(bubble) = bubble {
            let mut hit = None;
            for (target_e, target_t, target_bubble) in &bubbles {
                if projectile_e != target_e
                    && projectile_t
                        .translation
                        .truncate()
                        .distance(target_t.translation.truncate())
                        < 34.0
                {
                    hit = Some((
                        target_e,
                        target_bubble.color_id,
                        target_t.translation.truncate(),
                    ));
                    break;
                }
            }
            if let Some((target, color_id, pos)) = hit {
                commands.entity(projectile_e).despawn();
                if color_id == bubble.color_id {
                    let positions: Vec<(Entity, Vec2, usize)> = bubbles
                        .iter()
                        .map(|(e, t, b)| (e, t.translation.truncate(), b.color_id))
                        .collect();
                    for entity in bubble_cluster(target, color_id, &positions) {
                        commands.entity(entity).despawn();
                        session.score += 25;
                    }
                } else {
                    spawn_bubble(&mut commands, pos + Vec2::new(0.0, -38.0), bubble.color_id);
                }
            }
        }

        if projectile.owner == ProjectileOwner::Player {
            for (enemy_e, enemy_t, enemy_c, mut enemy) in &mut enemies {
                if circle_aabb(
                    projectile_t.translation.truncate(),
                    projectile.radius,
                    enemy_t.translation.truncate(),
                    enemy_c.size,
                ) {
                    enemy.hp -= 1;
                    commands.entity(projectile_e).despawn();
                    if enemy.hp <= 0 {
                        session.score += enemy.points;
                        commands.entity(enemy_e).despawn();
                    }
                    break;
                }
            }
        }
    }

    for (explosion_t, explosion_c) in &explosions {
        for (enemy_e, enemy_t, enemy_c, enemy) in &mut enemies {
            if aabb(
                explosion_t.translation.truncate(),
                explosion_c.size,
                enemy_t.translation.truncate(),
                enemy_c.size,
            ) {
                session.score += enemy.points;
                commands.entity(enemy_e).despawn();
            }
        }
        for (block_e, block_t, block_c) in &soft_blocks {
            if aabb(
                explosion_t.translation.truncate(),
                explosion_c.size,
                block_t.translation.truncate(),
                block_c.size,
            ) {
                session.score += 30;
                commands.entity(block_e).despawn();
            }
        }
    }

    for (enemy_e, enemy_t, enemy_c, _) in &enemies {
        for (mut player_t, player_c) in &mut players {
            if aabb(
                enemy_t.translation.truncate(),
                enemy_c.size,
                player_t.translation.truncate(),
                player_c.size,
            ) {
                commands.entity(enemy_e).despawn();
                session.lives -= 1;
                player_t.translation.x -= 60.0;
                if session.lives <= 0 {
                    finish_game(&mut session, &mut save, false);
                }
            }
        }
    }

    for (item_e, item_t, item_c) in &collectibles {
        for (player_t, player_c) in &players {
            if aabb(
                item_t.translation.truncate(),
                item_c.size,
                player_t.translation.truncate(),
                player_c.size,
            ) {
                commands.entity(item_e).despawn();
                session.score += 40;
                break;
            }
        }
    }

    for (goal_t, goal_c) in &goals {
        for (player_t, player_c) in &players {
            if aabb(
                goal_t.translation.truncate(),
                goal_c.size,
                player_t.translation.truncate(),
                player_c.size,
            ) {
                if matches!(session.kind, GameKind::RunGun | GameKind::Platformer) {
                    finish_game(&mut session, &mut save, true);
                }
            }
        }
    }
}

fn check_level_end(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    enemies: Query<Entity, With<Enemy>>,
    bubbles: Query<Entity, (With<Bubble>, Without<Projectile>)>,
    collectibles: Query<Entity, With<Collectible>>,
) {
    if session.finished || matches!(session.kind, GameKind::Tank | GameKind::BombMaze) {
        return;
    }
    if session.time_left <= 0.0 {
        let won = session.kind == GameKind::PlaneShooter;
        finish_game(&mut session, &mut save, won);
        return;
    }
    match session.kind {
        GameKind::BombMaze => {}
        GameKind::Tank => {}
        GameKind::BubbleShooter => {
            if bubbles.is_empty() {
                finish_game(&mut session, &mut save, true);
            }
        }
        GameKind::PlaneShooter => {}
        GameKind::RunGun => {
            if enemies.is_empty() {
                session.status = "路已经清好了，快去右侧终点！".to_string();
            }
        }
        GameKind::Platformer => {
            if collectibles.is_empty() {
                session.status = "星星收齐了，去彩旗那里！".to_string();
            }
        }
    }
}

fn update_hud(session: Res<GameSession>, mut hud: Query<&mut Text2d, With<HudText>>) {
    if !session.is_changed() || matches!(session.kind, GameKind::Tank | GameKind::BombMaze) {
        return;
    }
    if let Ok(mut text) = hud.single_mut() {
        let state = if session.finished {
            if session.won {
                "过关啦！Enter 重玩，Esc 返回"
            } else {
                "再试一次！Enter 重玩，Esc 返回"
            }
        } else if session.paused {
            "暂停中"
        } else {
            &session.status
        };
        **text = format!(
            "{}  第{}关  分数:{}  生命:{}  时间:{:.0}s  {}",
            session.kind.title(),
            session.level,
            session.score,
            session.lives.max(0),
            session.time_left.max(0.0),
            state
        );
    }
}

fn finish_game(session: &mut GameSession, save: &mut SaveData, won: bool) {
    session.finished = true;
    session.won = won;
    if won {
        session.score += 250 + session.time_left.max(0.0) as u32;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] = save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
    }
}

fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn clamp_translation(pos: &mut Vec3) {
    pos.x = pos.x.clamp(-ARENA_W * 0.5 + 20.0, ARENA_W * 0.5 - 20.0);
    pos.y = pos.y.clamp(-ARENA_H * 0.5 + 20.0, ARENA_H * 0.5 - 20.0);
}
