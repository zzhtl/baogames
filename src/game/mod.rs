use bevy::prelude::*;
use bevy::window::WindowResolution;

mod bomb_maze;
mod model;
mod space_shooter;
mod super_mario;
mod tank;

use crate::common::constants::{ARENA_H, ARENA_W, WINDOW_H, WINDOW_W};
use crate::common::render::{UiFont, background_rect, panel, rect, text};
use model::*;

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
            pause_and_result_input.run_if(in_state(AppState::Playing)),
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
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>),
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
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bomb_maze::BMStage>),
        )
        .add_systems(
            Update,
            (
                space_shooter::space_player_input,
                space_shooter::space_bullets_update,
                space_shooter::space_enemy_ai,
                space_shooter::space_spawner,
                space_shooter::space_stars_scroll,
                space_shooter::space_powerup_drop_and_pickup,
                space_shooter::space_collisions,
                space_shooter::space_despawn_offscreen,
                space_shooter::space_player_blink,
                space_shooter::space_lifetimes,
                space_shooter::space_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<space_shooter::SpaceState>),
        )
        .add_systems(
            Update,
            (
                super_mario::mario_player_input,
                super_mario::mario_physics,
                super_mario::mario_block_anim,
                super_mario::mario_coin_popup,
                super_mario::mario_goomba_ai,
                super_mario::mario_player_vs_goomba,
                super_mario::mario_koopa_ai,
                super_mario::mario_player_vs_koopa,
                super_mario::mario_shell_kills,
                super_mario::mario_platform_update,
                super_mario::mario_lava_check,
                super_mario::mario_powerup_update,
                super_mario::mario_player_vs_powerup,
                super_mario::mario_fireball_update,
                super_mario::mario_brick_break,
                super_mario::mario_shard_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<super_mario::MarioStage>),
        )
        .add_systems(
            Update,
            (
                super_mario::mario_bowser_ai,
                super_mario::mario_bowser_fireball_update,
                super_mario::mario_player_fire_vs_bowser,
                super_mario::mario_bowser_cleanup,
                super_mario::mario_axe_check,
                super_mario::mario_flag_check,
                super_mario::mario_flag_anim,
                super_mario::mario_finish_seq,
                super_mario::mario_respawn,
                super_mario::mario_time_check,
                super_mario::mario_player_blink,
                super_mario::mario_camera_follow,
                super_mario::mario_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<super_mario::MarioStage>),
        )
        .add_systems(OnExit(AppState::Playing), cleanup::<GameEntity>)
        .run();
}

fn setup_camera(mut commands: Commands) {
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
        "BaoGames 6 合 1 经典合集",
        Vec2::new(0.0, 215.0),
        34.0,
        Color::srgb(1.0, 0.94, 0.62),
        MenuEntity,
    );

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
        rect(
            &mut commands,
            Vec2::new(cx - card_w * 0.5 + 14.0, cy),
            Vec2::new(8.0, card_h - 16.0),
            accent,
            MenuEntity,
        );
        let srgba = accent.to_srgba();
        rect(
            &mut commands,
            Vec2::new(cx - card_w * 0.5 + 38.0, cy),
            Vec2::new(28.0, 28.0),
            Color::srgba(srgba.red, srgba.green, srgba.blue, 0.35),
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
        let info = if kind.implemented() {
            format!(
                "最高分 {}   已解锁第 {} 关",
                save.high_scores[kind.index()],
                save.unlocked_levels[kind.index()]
            )
        } else {
            "敬请期待".to_string()
        };
        text(
            &mut commands,
            &font,
            &info,
            Vec2::new(cx + 8.0, cy - 14.0),
            14.0,
            Color::srgb(0.78, 0.85, 0.94),
            MenuEntity,
        );
    }

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
        GameKind::SpaceShooter => Color::srgb(0.36, 0.72, 1.0),
        GameKind::SuperMario => Color::srgb(0.92, 0.35, 0.28),
        GameKind::Contra => Color::srgb(0.78, 0.42, 0.22),
        GameKind::BubbleBobble => Color::srgb(0.9, 0.34, 0.78),
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
        (KeyCode::Digit3, GameKind::SpaceShooter),
        (KeyCode::Digit4, GameKind::SuperMario),
        (KeyCode::Digit5, GameKind::Contra),
        (KeyCode::Digit6, GameKind::BubbleBobble),
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
        GameKind::SpaceShooter => (
            Color::srgb(0.03, 0.05, 0.1),
            Color::srgb(0.06, 0.09, 0.18),
            Color::srgb(0.38, 0.6, 0.96),
        ),
        GameKind::SuperMario => (
            Color::srgb(0.08, 0.13, 0.22),
            Color::srgb(0.1, 0.18, 0.3),
            Color::srgb(0.92, 0.78, 0.28),
        ),
        GameKind::Contra => (
            Color::srgb(0.12, 0.08, 0.07),
            Color::srgb(0.22, 0.13, 0.1),
            Color::srgb(0.82, 0.38, 0.26),
        ),
        GameKind::BubbleBobble => (
            Color::srgb(0.08, 0.09, 0.17),
            Color::srgb(0.12, 0.16, 0.28),
            Color::srgb(0.88, 0.45, 0.78),
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
        status: selected.0.goal_text().to_string(),
    });

    if !matches!(selected.0, GameKind::SuperMario) {
        paint_stage_backdrop(&mut commands, selected.0);
    }

    match selected.0 {
        GameKind::Tank => tank::setup_stage(&mut commands, &font, level),
        GameKind::BombMaze => bomb_maze::setup_stage(&mut commands, &font, level),
        GameKind::SpaceShooter => space_shooter::setup_stage(&mut commands, &font, level),
        GameKind::SuperMario => super_mario::setup_stage(&mut commands, &font, level),
        GameKind::Contra | GameKind::BubbleBobble => {
            setup_coming_soon(&mut commands, &font, selected.0);
        }
    }
}

fn setup_coming_soon(commands: &mut Commands, font: &UiFont, kind: GameKind) {
    panel(
        commands,
        Vec2::ZERO,
        Vec2::new(560.0, 220.0),
        Color::srgb(0.1, 0.13, 0.2),
        menu_accent(kind),
        GameEntity,
    );
    let title = kind.title().splitn(2, ' ').nth(1).unwrap_or(kind.title());
    text(
        commands,
        font,
        title,
        Vec2::new(0.0, 50.0),
        38.0,
        Color::srgb(1.0, 0.95, 0.75),
        GameEntity,
    );
    text(
        commands,
        font,
        "敬请期待",
        Vec2::new(0.0, -10.0),
        28.0,
        Color::srgb(0.96, 0.86, 0.5),
        GameEntity,
    );
    text(
        commands,
        font,
        "按 Backspace 或 Esc 返回菜单",
        Vec2::new(0.0, -60.0),
        16.0,
        Color::srgb(0.7, 0.82, 0.96),
        GameEntity,
    );
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
    // 占位卡片：Esc 或 Backspace 直接回菜单
    if !selected.0.implemented() {
        if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Backspace) {
            next_state.set(AppState::Menu);
        }
        return;
    }

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

fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
