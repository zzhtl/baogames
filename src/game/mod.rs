use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod bomb_maze;
mod bubble_shooter;
pub mod contra;
mod hud;
pub mod memory_match;
mod model;
mod overlay;
pub mod sokoban;
pub mod space_shooter;
mod super_mario;
pub mod tank;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::constants::{ARENA_H, ARENA_W, WINDOW_H, WINDOW_W, Z_HUD_LAYER};
use crate::common::render::{UiFont, background_rect, panel, rect, text};
use model::*;
use overlay::OverlayEntity;

pub fn run() {
    let save = SaveData::load();
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
        .add_plugins(crate::common::audio::SfxPlugin)
        .insert_resource(bevy::audio::GlobalVolume {
            volume: bevy::audio::Volume::Linear(save.volume),
        })
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(SelectedGame(GameKind::Tank))
        .insert_resource(save)
        .init_resource::<UiFont>()
        .init_resource::<bubble_shooter::BubbleAssets>()
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Menu), (reset_camera, setup_menu).chain())
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup::<MenuEntity>)
        .add_systems(OnEnter(AppState::Playing), (reset_camera, setup_game).chain())
        .add_systems(
            Update,
            (pause_and_result_input, sfx_on_session_edge, overlay::overlay_sync)
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            tank::tank_mode_select
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>),
        )
        .add_systems(
            Update,
            (
                tank::tank_player_input,
                tank::tank_enemy_ai,
                tank::tank_movement,
                tank::tank_bullet_update,
                tank::tank_powerup_pickup,
                tank::tank_freeze_tick,
                tank::tank_enemy_spawner,
                tank::tank_spawn_effect,
                tank::tank_player_respawn,
                tank::tank_lifetime_tick,
                tank::tank_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>)
                .run_if(tank::tank_playing),
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
        .add_systems(
            Update,
            (
                contra::contra_player_input,
                contra::contra_physics,
                contra::contra_enemy_ai,
                contra::contra_falcon_update,
                contra::contra_pickup_update,
                contra::contra_spawner,
                contra::contra_boss_update,
                contra::contra_bullets_update,
                contra::contra_player_vs_enemy,
                contra::contra_explosion_update,
                contra::contra_offscreen_cleanup,
                contra::contra_player_respawn,
                contra::contra_player_blink,
                contra::contra_camera_follow,
                contra::contra_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<contra::ContraStage>),
        )
        .add_systems(
            Update,
            (
                bubble_shooter::bubble_player_input,
                bubble_shooter::bubble_aim_dots_update,
                bubble_shooter::bubble_shot_update,
                bubble_shooter::bubble_pop_anim,
                bubble_shooter::bubble_fall_anim,
                bubble_shooter::bubble_next_preview_update,
                bubble_shooter::bubble_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bubble_shooter::BubbleStage>),
        )
        .add_systems(
            Update,
            (
                memory_match::memory_input,
                memory_match::memory_render_sync,
                memory_match::memory_cursor_follow,
                memory_match::memory_check_finish,
                memory_match::memory_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<memory_match::MemoryStage>),
        )
        .add_systems(
            Update,
            (
                sokoban::sokoban_input,
                sokoban::sokoban_render_sync,
                sokoban::sokoban_box_visual_sync,
                sokoban::sokoban_check_finish,
                sokoban::sokoban_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<sokoban::SokobanStage>),
        )
        .add_systems(
            OnExit(AppState::Playing),
            (
                cleanup::<GameEntity>,
                cleanup::<OverlayEntity>,
                cleanup_stage_resources,
            ),
        )
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

/// 相机是常驻实体，魂斗罗/超级玛丽会横向平移它；
/// 进菜单/进游戏（含重开）时必须复位，否则画面停留在上一局的偏移处。
fn reset_camera(mut cam_q: Query<&mut Transform, With<Camera2d>>) {
    for mut t in &mut cam_q {
        t.translation.x = 0.0;
        t.translation.y = 0.0;
    }
}

fn setup_menu(
    mut commands: Commands,
    save: Res<SaveData>,
    font: Res<UiFont>,
    selected: Res<SelectedGame>,
) {
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
        "BaoGames 8 合 1 经典合集",
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
        "按 1-8 选择游戏，Enter 开始 / Esc 暂停",
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
        let center = menu_card_center(i);
        let (cx, cy) = (center.x, center.y);
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
        let info = format!(
            "最高分 {}   已解锁第 {} 关",
            save.high_scores[kind.index()],
            save.unlocked_levels[kind.index()]
        );
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
        "方向键选择 · Enter 开始 · 数字 1-8 直达",
        Vec2::new(0.0, -244.0),
        17.0,
        Color::srgb(0.55, 0.7, 0.86),
        MenuEntity,
    );

    spawn_menu_cursor(&mut commands, selected.0.index());
}

/// 菜单第 i 张卡片的中心（2 列 × 4 行网格）。
fn menu_card_center(i: usize) -> Vec2 {
    Vec2::new(
        -190.0 + (i % 2) as f32 * 380.0,
        70.0 - (i / 2) as f32 * 78.0,
    )
}

/// 选中高亮：环绕当前卡片的 4 条金色边。
#[derive(Component)]
struct MenuCursor {
    offset: Vec2,
}

fn spawn_menu_cursor(commands: &mut Commands, index: usize) {
    let frame = Vec2::new(368.0, 78.0);
    let bar = 4.0;
    let color = Color::srgb(1.0, 0.88, 0.42);
    let center = menu_card_center(index);
    let edges: [(Vec2, Vec2); 4] = [
        (Vec2::new(0.0, frame.y * 0.5), Vec2::new(frame.x + bar, bar)),
        (Vec2::new(0.0, -frame.y * 0.5), Vec2::new(frame.x + bar, bar)),
        (Vec2::new(-frame.x * 0.5, 0.0), Vec2::new(bar, frame.y + bar)),
        (Vec2::new(frame.x * 0.5, 0.0), Vec2::new(bar, frame.y + bar)),
    ];
    for (offset, size) in edges {
        commands.spawn((
            Sprite::from_color(color, size),
            Transform::from_translation((center + offset).extend(5.0)),
            MenuCursor { offset },
            MenuEntity,
        ));
    }
}

fn menu_accent(kind: GameKind) -> Color {
    match kind {
        GameKind::Tank => Color::srgb(0.35, 0.78, 0.42),
        GameKind::BombMaze => Color::srgb(0.95, 0.58, 0.24),
        GameKind::SpaceShooter => Color::srgb(0.36, 0.72, 1.0),
        GameKind::SuperMario => Color::srgb(0.92, 0.35, 0.28),
        GameKind::Contra => Color::srgb(0.92, 0.18, 0.10),
        GameKind::BubbleBobble => Color::srgb(0.9, 0.34, 0.78),
        GameKind::MemoryMatch => Color::srgb(0.36, 0.86, 0.86),
        GameKind::Sokoban => Color::srgb(0.96, 0.78, 0.32),
    }
}

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedGame>,
    mut next_state: ResMut<NextState<AppState>>,
    mut cursor_q: Query<(&MenuCursor, &mut Transform)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    // 数字键直达
    let choices = [
        (KeyCode::Digit1, GameKind::Tank),
        (KeyCode::Digit2, GameKind::BombMaze),
        (KeyCode::Digit3, GameKind::SpaceShooter),
        (KeyCode::Digit4, GameKind::SuperMario),
        (KeyCode::Digit5, GameKind::Contra),
        (KeyCode::Digit6, GameKind::BubbleBobble),
        (KeyCode::Digit7, GameKind::MemoryMatch),
        (KeyCode::Digit8, GameKind::Sokoban),
    ];
    for (key, kind) in choices {
        if keys.just_pressed(key) {
            selected.0 = kind;
            sfx.write(PlaySfx(SfxKind::MenuConfirm));
            next_state.set(AppState::Playing);
            // 直达后立即返回，防止同帧方向键覆盖选中项
            return;
        }
    }

    // 方向键 / WASD 移动高亮（2 列 × 4 行）
    let idx = selected.0.index();
    let mut next = idx;
    if (keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA)) && idx % 2 == 1 {
        next = idx - 1;
    }
    if (keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD)) && idx % 2 == 0
    {
        next = idx + 1;
    }
    if (keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW)) && idx >= 2 {
        next = idx - 2;
    }
    if (keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS)) && idx + 2 < 8 {
        next = idx + 2;
    }
    if next != idx {
        selected.0 = GameKind::ALL[next];
        sfx.write(PlaySfx(SfxKind::MenuMove));
        let center = menu_card_center(next);
        for (cursor, mut tr) in &mut cursor_q {
            tr.translation.x = center.x + cursor.offset.x;
            tr.translation.y = center.y + cursor.offset.y;
        }
    }

    if keys.just_pressed(KeyCode::Enter) {
        sfx.write(PlaySfx(SfxKind::MenuConfirm));
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
        GameKind::MemoryMatch => (
            Color::srgb(0.06, 0.13, 0.18),
            Color::srgb(0.10, 0.20, 0.28),
            Color::srgb(0.40, 0.86, 0.86),
        ),
        GameKind::Sokoban => (
            Color::srgb(0.10, 0.08, 0.06),
            Color::srgb(0.18, 0.14, 0.10),
            Color::srgb(0.96, 0.74, 0.32),
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
    bubble_assets: Res<bubble_shooter::BubbleAssets>,
    camera_q: Query<Entity, With<Camera2d>>,
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

    // HUD 根：挂相机下，滚屏游戏的 HUD 自动跟随（见 hud.rs 模块说明）
    let hud_root = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, Z_HUD_LAYER),
            Visibility::default(),
            GameEntity,
        ))
        .id();
    if let Ok(camera) = camera_q.single() {
        commands.entity(hud_root).insert(ChildOf(camera));
    }

    if !matches!(selected.0, GameKind::SuperMario | GameKind::Contra) {
        paint_stage_backdrop(&mut commands, selected.0);
    }

    match selected.0 {
        GameKind::Tank => tank::setup_stage(&mut commands, &font, hud_root, level),
        GameKind::BombMaze => bomb_maze::setup_stage(&mut commands, &font, hud_root, level),
        GameKind::SpaceShooter => space_shooter::setup_stage(&mut commands, &font, hud_root, level),
        GameKind::SuperMario => super_mario::setup_stage(&mut commands, &font, hud_root, level),
        GameKind::Contra => contra::setup_stage(
            &mut commands,
            &font,
            hud_root,
            level,
            save.high_scores[GameKind::Contra.index()],
        ),
        GameKind::BubbleBobble => {
            bubble_shooter::setup_stage(&mut commands, &bubble_assets, &font, hud_root, level)
        }
        GameKind::MemoryMatch => memory_match::setup_stage(&mut commands, &font, hud_root, level),
        GameKind::Sokoban => sokoban::setup_stage(&mut commands, &font, hud_root, level),
    }
}

fn pause_and_result_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<AppState>>,
    selected: Res<SelectedGame>,
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
        // 重开：向 Playing 再转移一次（identity transition），
        // 走完整的 OnExit 清理 + OnEnter 重建，与首次进入同一条路径。
        next_state.set(AppState::Playing);
    }
}

/// 监听对局状态的沿变化，全局播放胜利 / 失败 / 暂停音效——8 个游戏免接入。
fn sfx_on_session_edge(
    session: Option<Res<GameSession>>,
    mut prev: Local<(bool, bool)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Some(session) = session else { return };
    // 新对局的 GameSession 刚插入时复位基线，避免 Local 残留上一局的状态吞掉边沿
    if session.is_added() {
        *prev = (false, false);
    }
    let (was_finished, was_paused) = *prev;
    if session.finished && !was_finished {
        sfx.write(PlaySfx(if session.won {
            SfxKind::Win
        } else {
            SfxKind::Lose
        }));
    }
    if session.paused && !was_paused {
        sfx.write(PlaySfx(SfxKind::Pause));
    }
    *prev = (session.finished, session.paused);
}

fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        // try_despawn：父实体先被 despawn 时其子实体已随层级递归销毁，
        // 快照里的子实体命令会落空，despawn() 会对每个落空刷一条警告。
        commands.entity(entity).try_despawn();
    }
}

fn cleanup_stage_resources(mut commands: Commands) {
    commands.remove_resource::<tank::TankStage>();
    commands.remove_resource::<bomb_maze::BMStage>();
    commands.remove_resource::<space_shooter::SpaceState>();
    commands.remove_resource::<super_mario::MarioStage>();
    commands.remove_resource::<contra::ContraStage>();
    commands.remove_resource::<bubble_shooter::BubbleStage>();
    commands.remove_resource::<memory_match::MemoryStage>();
    commands.remove_resource::<sokoban::SokobanStage>();
}
