use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::audio::{AudioSink, AudioSinkPlayback};

pub mod bomb_maze;
mod bubble_shooter;
pub mod contra;
#[cfg(feature = "devtools")]
mod devtools;
mod hud;
pub mod memory_match;
mod menu;
mod model;
mod overlay;
pub mod sokoban;
pub mod space_shooter;
mod super_mario;
pub mod tank;

use crate::common::audio::{AudioMix, MusicEntity, MusicKind, PlayMusic, PlaySfx, SfxKind};
use crate::common::constants::{ARENA_H, ARENA_W, WINDOW_H, WINDOW_W, Z_HUD_LAYER};
use crate::common::input::{ActionInputPlugin, ActionInputSet, ActionState};
use crate::common::px::px;
use crate::common::pixel_canvas::{InGameCamera, PixelCanvasConfig, PixelCanvasPlugin};
use crate::common::render::{UiFont, background_rect, rect};
use crate::common::settings::{DisplayMode, InputAction, InputBindings, PlayerSlot};
use model::*;
use overlay::OverlayEntity;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MarioFixedSet {
    World,
    FinishAndHud,
}

pub fn run() {
    let save = SaveData::load();
    let bindings = InputBindings::from(&save.settings);
    let audio_mix = AudioMix {
        music_volume: save.settings.music_volume,
        sfx_volume: save.settings.sfx_volume,
    };
    let canvas_config = PixelCanvasConfig {
        display_mode: save.settings.display_mode,
        crt_enabled: save.settings.crt_enabled,
        shake_enabled: save.settings.screen_shake,
    };
    let mut app = App::new();
    app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BaoGames".into(),
                        resolution: WindowResolution::new(WINDOW_W, WINDOW_H),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(crate::common::audio::SfxPlugin)
        .add_plugins(ActionInputPlugin)
        .add_plugins(PixelCanvasPlugin)
        .insert_resource(audio_mix)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(SelectedGame(GameKind::Tank))
        .insert_resource(bindings)
        .insert_resource(canvas_config)
        .insert_resource(save)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<UiFont>()
        .init_resource::<menu::MenuUiState>()
        .init_resource::<bubble_shooter::BubbleAssets>()
        .init_resource::<super_mario::MarioControls>()
        .init_state::<AppState>()
        .configure_sets(
            FixedUpdate,
            (MarioFixedSet::World, MarioFixedSet::FinishAndHud).chain(),
        )
        .add_systems(OnEnter(AppState::Menu), (reset_camera, menu::setup_menu).chain())
        .add_systems(Update, menu::menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup::<MenuEntity>)
        .add_systems(OnEnter(AppState::Playing), (reset_camera, setup_game).chain())
        .add_systems(
            PreUpdate,
            super_mario::mario_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<super_mario::MarioStage>),
        )
        .add_systems(
            PreUpdate,
            contra::contra_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<contra::ContraStage>),
        )
        .add_systems(
            PreUpdate,
            tank::tank_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>),
        )
        .add_systems(
            PreUpdate,
            bomb_maze::bm_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bomb_maze::BMStage>),
        )
        .add_systems(
            PreUpdate,
            space_shooter::space_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<space_shooter::SpaceState>),
        )
        .add_systems(
            PreUpdate,
            bubble_shooter::bubble_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bubble_shooter::BubbleStage>),
        )
        .add_systems(
            PreUpdate,
            sokoban::sokoban_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<sokoban::SokobanStage>),
        )
        .add_systems(
            PreUpdate,
            memory_match::memory_sample_input
                .after(ActionInputSet)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<memory_match::MemoryStage>),
        )
        .add_systems(
            Update,
            (
                progression_on_session_edge,
                pause_and_result_input,
                sfx_on_session_edge,
                overlay::overlay_sync,
            )
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
            FixedUpdate,
            (
                tank::tank_player_input,
                tank::tank_enemy_ai,
                tank::tank_movement,
                tank::tank_bullet_update,
                tank::tank_bullet_clash,
                tank::tank_powerup_pickup,
                tank::tank_freeze_tick,
                tank::tank_enemy_spawner,
                tank::tank_spawn_effect,
                tank::tank_player_respawn,
                tank::tank_lifetime_tick,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>)
                .run_if(tank::tank_playing),
        )
        .add_systems(
            Update,
            (
                tank::tank_motion_visual_update,
                tank::tank_shield_visual_update,
                tank::tank_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<tank::TankStage>)
                .run_if(tank::tank_playing),
        )
        .add_systems(
            FixedUpdate,
            (
                bomb_maze::bm_player_input,
                bomb_maze::bm_enemy_ai,
                bomb_maze::bm_bomb_tick,
                bomb_maze::bm_flame_tick,
                bomb_maze::bm_flame_damage,
                bomb_maze::bm_flame_burn_powerups,
                bomb_maze::bm_enemy_touch,
                bomb_maze::bm_powerup_pickup,
                bomb_maze::bm_exit_and_respawn,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bomb_maze::BMStage>),
        )
        .add_systems(
            Update,
            (
                bomb_maze::bm_player_blink,
                bomb_maze::bm_actor_visual_update,
                bomb_maze::bm_bomb_visual_update,
                bomb_maze::bm_item_visual_update,
                bomb_maze::bm_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bomb_maze::BMStage>),
        )
        .add_systems(
            FixedUpdate,
            (
                space_shooter::space_player_input,
                space_shooter::space_spawner,
                space_shooter::space_enemy_ai,
                space_shooter::space_bullets_update,
                space_shooter::space_powerup_drop_and_pickup,
                space_shooter::space_collisions,
                space_shooter::space_despawn_offscreen,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<space_shooter::SpaceState>),
        )
        .add_systems(
            Update,
            (
                space_shooter::space_stars_scroll,
                space_shooter::space_player_blink,
                space_shooter::space_enemy_visual_update,
                space_shooter::space_powerup_visual_update,
                space_shooter::space_effects_update,
                space_shooter::space_lifetimes,
                space_shooter::space_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<space_shooter::SpaceState>),
        )
        .add_systems(
            FixedUpdate,
            (
                super_mario::mario_player_input,
                // 平台先动、物理再解算：反过来的话 last_dx/last_dy 是上一帧的位移，
                // 玩家会先被平台压进去、再被地形解算横向弹开
                super_mario::mario_platform_update,
                super_mario::mario_physics,
                super_mario::mario_checkpoint_update,
                super_mario::mario_player_animation,
                super_mario::mario_block_anim,
                super_mario::mario_coin_popup,
                super_mario::mario_goomba_ai,
                super_mario::mario_player_vs_goomba,
                super_mario::mario_koopa_ai,
                super_mario::mario_player_vs_koopa,
                super_mario::mario_shell_kills,
                super_mario::mario_lava_check,
                super_mario::mario_powerup_update,
                super_mario::mario_player_vs_powerup,
                super_mario::mario_fireball_update,
                super_mario::mario_shard_update,
            )
                .chain()
                .in_set(MarioFixedSet::World)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<super_mario::MarioStage>),
        )
        .add_systems(
            FixedUpdate,
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
                .in_set(MarioFixedSet::FinishAndHud)
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<super_mario::MarioStage>),
        )
        .add_systems(
            FixedUpdate,
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
                contra::contra_player_respawn,
                contra::contra_player_pose_sync,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<contra::ContraStage>),
        )
        .add_systems(
            Update,
            (
                contra::contra_explosion_update,
                contra::contra_muzzle_flash_update,
                contra::contra_boss_flash_update,
                contra::contra_turret_flash_update,
                contra::contra_offscreen_cleanup,
                contra::contra_player_blink,
                contra::contra_player_animation,
                contra::contra_camera_follow,
                contra::contra_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<contra::ContraStage>),
        )
        .add_systems(
            FixedUpdate,
            (
                bubble_shooter::bubble_player_input,
                bubble_shooter::bubble_shot_update,
                bubble_shooter::bubble_pop_anim,
                bubble_shooter::bubble_fall_anim,
                bubble_shooter::bubble_settle_anim,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bubble_shooter::BubbleStage>),
        )
        .add_systems(
            Update,
            (
                bubble_shooter::bubble_aim_dots_update,
                bubble_shooter::bubble_field_feedback,
                bubble_shooter::bubble_next_preview_update,
                bubble_shooter::bubble_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<bubble_shooter::BubbleStage>),
        )
        .add_systems(
            FixedUpdate,
            (
                memory_match::memory_input,
                memory_match::memory_check_finish,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<memory_match::MemoryStage>),
        )
        .add_systems(
            Update,
            (
                memory_match::memory_card_flip_update,
                memory_match::memory_render_sync,
                memory_match::memory_cursor_follow,
                memory_match::memory_hud_update,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<memory_match::MemoryStage>),
        )
        .add_systems(
            FixedUpdate,
            (
                sokoban::sokoban_input,
                sokoban::sokoban_check_finish,
            )
                .chain()
                .run_if(in_state(AppState::Playing))
                .run_if(resource_exists::<sokoban::SokobanStage>),
        )
        .add_systems(
            Update,
            (
                sokoban::sokoban_render_sync,
                sokoban::sokoban_box_visual_sync,
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
        );

    #[cfg(feature = "devtools")]
    if let Some((scene, out)) = devtools::requested_scene() {
        app.add_plugins(devtools::CapturePlugin { scene, out });
    }

    app.run();
}

/// 相机是常驻实体，魂斗罗/超级玛丽会横向平移它；
/// 进菜单/进游戏（含重开）时必须复位，否则画面停留在上一局的偏移处。
fn reset_camera(mut cam_q: Query<&mut Transform, With<InGameCamera>>) {
    for mut t in &mut cam_q {
        t.translation.x = 0.0;
        t.translation.y = 0.0;
    }
}

fn paint_stage_backdrop(commands: &mut Commands, kind: GameKind, display_mode: DisplayMode) {
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

    // 背景铺满最宽的比例即可；边框必须按**当前可见宽度**画，
    // 原来用 ARENA_W(=16:9 的 960) 减 28，在默认 4:3(可见 720) 下整条边框直接出屏。
    let visible_w = display_mode.world_width();
    background_rect(
        commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W, ARENA_H),
        base,
        GameEntity,
    );
    let half_w = (visible_w * 0.5) as i32;
    let half_h = (ARENA_H * 0.5) as i32;
    let step = px(16.0) as usize; // 16 画布像素一格
    for x in (-half_w..=half_w).step_by(step) {
        rect(
            commands,
            Vec2::new(x as f32, 0.0),
            Vec2::new(px(1.0), ARENA_H),
            grid,
            GameEntity,
        );
    }
    for y in (-half_h..=half_h).step_by(step) {
        rect(
            commands,
            Vec2::new(0.0, y as f32),
            Vec2::new(visible_w, px(1.0)),
            grid,
            GameEntity,
        );
    }
    // 画四条边而不是「透明填充的 panel」：panel 是「描边色整块 + 内缩的填充块」，
    // 填充块透明时下面那整块描边色会盖满全屏，把游戏画面整个吃掉。
    let frame = Vec2::new(visible_w - px(2.0), ARENA_H - px(2.0));
    for (pos, size) in [
        (Vec2::new(0.0, frame.y * 0.5), Vec2::new(frame.x, px(1.0))),
        (Vec2::new(0.0, -frame.y * 0.5), Vec2::new(frame.x, px(1.0))),
        (Vec2::new(-frame.x * 0.5, 0.0), Vec2::new(px(1.0), frame.y)),
        (Vec2::new(frame.x * 0.5, 0.0), Vec2::new(px(1.0), frame.y)),
    ] {
        rect(commands, pos, size, border, GameEntity);
    }
}

#[allow(clippy::too_many_arguments)]
fn setup_game(
    mut commands: Commands,
    selected: Res<SelectedGame>,
    save: Res<SaveData>,
    font: Res<UiFont>,
    canvas: Res<PixelCanvasConfig>,
    bubble_assets: Res<bubble_shooter::BubbleAssets>,
    camera_q: Query<Entity, With<InGameCamera>>,
    mut music: MessageWriter<PlayMusic>,
) {
    let index = selected.0.index();
    let level = save.selected_levels[index]
        .clamp(1, save.unlocked_levels[index].min(selected.0.max_level()));
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
    music.write(PlayMusic(match selected.0 {
        GameKind::Tank => MusicKind::Tank,
        GameKind::BombMaze => MusicKind::BombMaze,
        GameKind::SpaceShooter => MusicKind::SpaceShooter,
        GameKind::SuperMario => MusicKind::SuperMario,
        GameKind::Contra => MusicKind::Contra,
        GameKind::BubbleBobble => MusicKind::BubbleShooter,
        GameKind::MemoryMatch => MusicKind::MemoryMatch,
        GameKind::Sokoban => MusicKind::Sokoban,
    }));

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
        paint_stage_backdrop(&mut commands, selected.0, canvas.display_mode);
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
    actions: Res<ActionState>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<AppState>>,
    selected: Res<SelectedGame>,
    music_q: Query<&AudioSink, With<MusicEntity>>,
) {
    let player = PlayerSlot::One;
    if actions.just_pressed(player, InputAction::Start) {
        if session.finished {
            next_state.set(AppState::Playing);
        } else {
            session.paused = !session.paused;
            session.status = if session.paused {
                "已暂停：开始键继续，返回键回到菜单".to_string()
            } else {
                selected.0.goal_text().to_string()
            };
            for sink in &music_q {
                if session.paused {
                    sink.pause();
                } else {
                    sink.play();
                }
            }
        }
    }
    if actions.just_pressed(player, InputAction::Back) {
        next_state.set(AppState::Menu);
    }
    if session.finished && actions.just_pressed(player, InputAction::Primary) {
        next_state.set(AppState::Playing);
    }
}

/// 对局结束后把自由选关游标推进到下一关；失败时保留当前关，方便立即重试。
fn selected_level_after_result(kind: GameKind, level: u8, won: bool, unlocked: u8) -> u8 {
    let level = level.clamp(1, kind.max_level());
    let unlocked = unlocked.clamp(1, kind.max_level());
    if won {
        (level + 1).min(kind.max_level())
    } else {
        level
    }
    .min(unlocked)
}

fn progression_on_session_edge(
    session: Option<Res<GameSession>>,
    mut save: ResMut<SaveData>,
    mut handled: Local<Option<(GameKind, u8, bool)>>,
) {
    let Some(session) = session else {
        *handled = None;
        return;
    };
    if !session.finished {
        *handled = None;
        return;
    }
    let result = (session.kind, session.level, session.won);
    if *handled == Some(result) {
        return;
    }
    *handled = Some(result);

    let index = session.kind.index();
    let selected_level = selected_level_after_result(
        session.kind,
        session.level,
        session.won,
        save.unlocked_levels[index],
    );
    if save.selected_levels[index] != selected_level {
        save.selected_levels[index] = selected_level;
        save.store();
    }
}

/// 监听对局状态的沿变化，全局播放胜利 / 失败 / 暂停音效——8 个游戏免接入。
fn sfx_on_session_edge(
    session: Option<Res<GameSession>>,
    mut prev: Local<(bool, bool)>,
    mut sfx: MessageWriter<PlaySfx>,
    music_q: Query<&AudioSink, With<MusicEntity>>,
) {
    let Some(session) = session else { return };
    // 新对局的 GameSession 刚插入时复位基线，避免 Local 残留上一局的状态吞掉边沿
    if session.is_added() {
        *prev = (false, false);
    }
    let (was_finished, was_paused) = *prev;
    if session.finished && !was_finished {
        for sink in &music_q {
            sink.pause();
        }
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

#[cfg(test)]
mod progression_tests {
    use super::*;

    #[test]
    fn victory_selects_unlocked_next_level() {
        assert_eq!(selected_level_after_result(GameKind::SuperMario, 2, true, 3), 3);
    }

    #[test]
    fn final_level_and_failure_stay_on_current_level() {
        assert_eq!(selected_level_after_result(GameKind::SuperMario, 4, true, 4), 4);
        assert_eq!(selected_level_after_result(GameKind::Tank, 6, false, 8), 6);
    }

    #[test]
    fn result_never_selects_a_locked_level() {
        assert_eq!(selected_level_after_result(GameKind::Tank, 5, true, 5), 5);
    }
}
