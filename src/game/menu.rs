use bevy::input::gamepad::Gamepad;
use bevy::prelude::*;

use crate::common::audio::{AudioMix, MusicKind, PlayMusic, PlaySfx, SfxKind};
use crate::common::constants::{ARENA_H, ARENA_W, FONT_BODY, FONT_HEADING, FONT_SMALL, FONT_TITLE};
use crate::common::input::ActionState;
use crate::common::pixel_canvas::PixelCanvasConfig;
use crate::common::render::{UiFont, background_rect, panel, rect, text};
use crate::common::settings::{
    InputAction, InputBindings, KeyboardKey, PadButton, PlayerSlot,
};

use super::model::{AppState, GameKind, MenuEntity, SaveData, SelectedGame};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum MenuPage {
    #[default]
    Library,
    Settings,
    Controls,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LibraryFocus {
    #[default]
    Games,
    Stage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CaptureDevice {
    Keyboard,
    Gamepad,
}

#[derive(Resource, Debug)]
pub(super) struct MenuUiState {
    page: MenuPage,
    library_focus: LibraryFocus,
    settings_cursor: usize,
    control_player: PlayerSlot,
    control_cursor: usize,
    control_device: CaptureDevice,
    capture: Option<(PlayerSlot, InputAction, CaptureDevice)>,
}

impl Default for MenuUiState {
    fn default() -> Self {
        Self {
            page: MenuPage::Library,
            library_focus: LibraryFocus::Games,
            settings_cursor: 0,
            control_player: PlayerSlot::One,
            control_cursor: 0,
            control_device: CaptureDevice::Keyboard,
            capture: None,
        }
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

fn spawn_backdrop(commands: &mut Commands) {
    background_rect(
        commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W * 1.4, ARENA_H),
        Color::srgb(0.025, 0.035, 0.055),
        MenuEntity,
    );
    for y in (-240..=240).step_by(24) {
        rect(
            commands,
            Vec2::new(0.0, y as f32),
            Vec2::new(ARENA_W * 1.4, 3.0),
            Color::srgb(0.045, 0.06, 0.09),
            MenuEntity,
        );
    }
    for x in (-480..=480).step_by(24) {
        rect(
            commands,
            Vec2::new(x as f32, 0.0),
            Vec2::new(3.0, ARENA_H),
            Color::srgb(0.04, 0.052, 0.078),
            MenuEntity,
        );
    }
}

fn card_center(index: usize) -> Vec2 {
    Vec2::new(
        -168.0 + (index % 2) as f32 * 336.0,
        130.0 - (index / 2) as f32 * 66.0,
    )
}

fn spawn_cover_icon(commands: &mut Commands, kind: GameKind, center: Vec2, accent: Color) {
    rect(commands, center, Vec2::new(48.0, 42.0), Color::srgb(0.025, 0.03, 0.04), MenuEntity);
    let dark = Color::srgb(0.12, 0.14, 0.18);
    match kind {
        GameKind::Tank => {
            rect(commands, center + Vec2::new(0.0, -5.0), Vec2::new(27.0, 22.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(0.0, 11.0), Vec2::new(5.0, 17.0), accent, MenuEntity);
            for x in [-17.0, 17.0] {
                rect(commands, center + Vec2::new(x, -5.0), Vec2::new(7.0, 28.0), dark, MenuEntity);
            }
        }
        GameKind::BombMaze => {
            rect(commands, center, Vec2::splat(27.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(9.0, 13.0), Vec2::new(5.0, 10.0), dark, MenuEntity);
            rect(commands, center + Vec2::new(13.0, 19.0), Vec2::new(9.0, 3.0), Color::srgb(1.0, 0.85, 0.3), MenuEntity);
        }
        GameKind::SpaceShooter => {
            rect(commands, center, Vec2::new(11.0, 37.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(0.0, -5.0), Vec2::new(37.0, 10.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(0.0, -19.0), Vec2::new(7.0, 8.0), Color::srgb(1.0, 0.72, 0.2), MenuEntity);
        }
        GameKind::SuperMario => {
            rect(commands, center + Vec2::new(0.0, 10.0), Vec2::new(25.0, 8.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(0.0, -1.0), Vec2::new(19.0, 13.0), Color::srgb(0.96, 0.72, 0.45), MenuEntity);
            rect(commands, center + Vec2::new(-7.0, -14.0), Vec2::new(9.0, 15.0), Color::srgb(0.24, 0.42, 0.9), MenuEntity);
            rect(commands, center + Vec2::new(7.0, -14.0), Vec2::new(9.0, 15.0), Color::srgb(0.24, 0.42, 0.9), MenuEntity);
        }
        GameKind::Contra => {
            rect(commands, center + Vec2::new(-5.0, 4.0), Vec2::new(16.0, 31.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(9.0, 8.0), Vec2::new(24.0, 5.0), Color::srgb(0.74, 0.78, 0.88), MenuEntity);
            rect(commands, center + Vec2::new(-10.0, -17.0), Vec2::new(7.0, 12.0), dark, MenuEntity);
            rect(commands, center + Vec2::new(2.0, -17.0), Vec2::new(7.0, 12.0), dark, MenuEntity);
        }
        GameKind::BubbleBobble => {
            for (offset, color) in [
                (Vec2::new(-10.0, 7.0), accent),
                (Vec2::new(10.0, 7.0), Color::srgb(0.35, 0.8, 1.0)),
                (Vec2::new(0.0, -10.0), Color::srgb(1.0, 0.75, 0.25)),
            ] {
                rect(commands, center + offset, Vec2::splat(17.0), color, MenuEntity);
            }
        }
        GameKind::MemoryMatch => {
            rect(commands, center + Vec2::new(-10.0, 0.0), Vec2::new(17.0, 27.0), accent, MenuEntity);
            rect(commands, center + Vec2::new(10.0, 0.0), Vec2::new(17.0, 27.0), Color::srgb(0.9, 0.45, 0.55), MenuEntity);
        }
        GameKind::Sokoban => {
            rect(commands, center, Vec2::splat(29.0), Color::srgb(0.55, 0.28, 0.1), MenuEntity);
            rect(commands, center, Vec2::splat(21.0), accent, MenuEntity);
            rect(commands, center, Vec2::splat(7.0), Color::srgb(1.0, 0.93, 0.62), MenuEntity);
        }
    }
}

fn build_library(
    commands: &mut Commands,
    font: &UiFont,
    save: &SaveData,
    selected: GameKind,
    ui: &MenuUiState,
) {
    panel(
        commands,
        Vec2::new(0.0, 232.0),
        Vec2::new(660.0, 54.0),
        Color::srgb(0.08, 0.10, 0.15),
        Color::srgb(0.98, 0.78, 0.28),
        MenuEntity,
    );
    text(commands, font, "BAOGAMES · 经典卡带墙", Vec2::new(0.0, 235.0), FONT_TITLE, Color::srgb(1.0, 0.92, 0.58), MenuEntity);
    text(commands, font, "方向选择 · 开始键游玩 · 动作一选关 · 动作二设置", Vec2::new(0.0, 198.0), FONT_BODY, Color::srgb(0.65, 0.8, 0.95), MenuEntity);

    for (index, kind) in GameKind::ALL.iter().copied().enumerate() {
        let center = card_center(index);
        let accent = menu_accent(kind);
        let selected_card = kind == selected;
        let border = if selected_card && ui.library_focus == LibraryFocus::Games {
            Color::srgb(1.0, 0.9, 0.4)
        } else {
            accent
        };
        panel(
            commands,
            center,
            Vec2::new(320.0, 56.0),
            if selected_card { Color::srgb(0.13, 0.16, 0.22) } else { Color::srgb(0.075, 0.09, 0.13) },
            border,
            MenuEntity,
        );
        spawn_cover_icon(commands, kind, center + Vec2::new(-128.0, 0.0), accent);
        let title = kind.title().split_once(' ').map(|(_, title)| title).unwrap_or(kind.title());
        text(commands, font, title, center + Vec2::new(-47.0, 11.0), FONT_HEADING, Color::srgb(1.0, 0.96, 0.84), MenuEntity);
        let info = format!(
            "记录 {:06}  关卡 {}/{}",
            save.high_scores[kind.index()],
            save.unlocked_levels[kind.index()],
            kind.max_level(),
        );
        text(commands, font, &info, center + Vec2::new(25.0, -13.0), FONT_SMALL, Color::srgb(0.65, 0.76, 0.88), MenuEntity);
    }

    let index = selected.index();
    let level = save.selected_levels[index].clamp(1, save.unlocked_levels[index]);
    let stage_border = if ui.library_focus == LibraryFocus::Stage {
        Color::srgb(1.0, 0.9, 0.4)
    } else {
        menu_accent(selected)
    };
    panel(commands, Vec2::new(0.0, -190.0), Vec2::new(660.0, 104.0), Color::srgb(0.055, 0.07, 0.105), stage_border, MenuEntity);
    text(commands, font, selected.goal_text(), Vec2::new(0.0, -162.0), FONT_BODY, Color::srgb(0.88, 0.92, 1.0), MenuEntity);
    text(commands, font, &format!("◀  第 {level} 关  ▶"), Vec2::new(0.0, -194.0), FONT_HEADING, Color::srgb(1.0, 0.88, 0.45), MenuEntity);
    let hint = if ui.library_focus == LibraryFocus::Stage {
        "左右选关 · 动作一 / 开始键游玩 · 动作二返回"
    } else {
        "动作一进入选关 · 开始键从当前关游玩"
    };
    text(commands, font, hint, Vec2::new(0.0, -226.0), FONT_SMALL, Color::srgb(0.58, 0.72, 0.88), MenuEntity);
}

fn build_settings(commands: &mut Commands, font: &UiFont, save: &SaveData, ui: &MenuUiState) {
    panel(commands, Vec2::ZERO, Vec2::new(660.0, 480.0), Color::srgb(0.055, 0.07, 0.105), Color::srgb(0.45, 0.75, 0.95), MenuEntity);
    text(commands, font, "系统设置", Vec2::new(0.0, 205.0), FONT_TITLE, Color::srgb(0.75, 0.9, 1.0), MenuEntity);
    let settings = &save.settings;
    let rows = [
        ("画面比例", settings.display_mode.label().to_string()),
        ("玩法模式", settings.gameplay_profile.label().to_string()),
        ("CRT 扫描线", if settings.crt_enabled { "开" } else { "关" }.to_string()),
        ("轻微震屏", if settings.screen_shake { "开" } else { "关" }.to_string()),
        ("音乐音量", format!("{:>3}%", (settings.music_volume * 100.0).round() as i32)),
        ("音效音量", format!("{:>3}%", (settings.sfx_volume * 100.0).round() as i32)),
        ("按键配置", "P1 / P2".to_string()),
        ("返回卡带墙", String::new()),
    ];
    for (index, (label, value)) in rows.iter().enumerate() {
        let y = 145.0 - index as f32 * 43.0;
        if index == ui.settings_cursor {
            rect(commands, Vec2::new(0.0, y), Vec2::new(600.0, 36.0), Color::srgb(0.13, 0.22, 0.32), MenuEntity);
            rect(commands, Vec2::new(-285.0, y), Vec2::new(6.0, 30.0), Color::srgb(0.98, 0.82, 0.32), MenuEntity);
        }
        text(commands, font, label, Vec2::new(-180.0, y), FONT_HEADING, Color::srgb(0.9, 0.94, 1.0), MenuEntity);
        if !value.is_empty() {
            text(commands, font, &format!("◀  {value}  ▶"), Vec2::new(170.0, y), FONT_BODY, Color::srgb(1.0, 0.86, 0.42), MenuEntity);
        }
    }
    text(commands, font, "上下选择 · 左右调整 · 动作一确认 · 动作二返回", Vec2::new(0.0, -222.0), FONT_SMALL, Color::srgb(0.58, 0.72, 0.88), MenuEntity);
}

fn build_controls(commands: &mut Commands, font: &UiFont, save: &SaveData, ui: &MenuUiState) {
    panel(commands, Vec2::ZERO, Vec2::new(690.0, 500.0), Color::srgb(0.055, 0.07, 0.105), Color::srgb(0.58, 0.86, 0.55), MenuEntity);
    let player_label = if ui.control_player == PlayerSlot::One { "P1" } else { "P2" };
    text(commands, font, &format!("按键配置 · {player_label}"), Vec2::new(0.0, 222.0), FONT_TITLE, Color::srgb(0.78, 1.0, 0.72), MenuEntity);
    text(commands, font, "重置键切换玩家 · 左右选择键盘/手柄 · 动作一修改", Vec2::new(0.0, 188.0), FONT_SMALL, Color::srgb(0.62, 0.78, 0.9), MenuEntity);
    let bindings = save.settings.bindings[ui.control_player.index()];
    for (row, action) in InputAction::ALL.iter().copied().enumerate() {
        let y = 145.0 - row as f32 * 36.0;
        if row == ui.control_cursor {
            let x = match ui.control_device {
                CaptureDevice::Keyboard => 25.0,
                CaptureDevice::Gamepad => 235.0,
            };
            rect(commands, Vec2::new(x, y), Vec2::new(190.0, 30.0), Color::srgb(0.13, 0.25, 0.2), MenuEntity);
        }
        let binding = bindings.binding(action);
        text(commands, font, action.label(), Vec2::new(-235.0, y), FONT_BODY, Color::srgb(0.9, 0.94, 1.0), MenuEntity);
        text(commands, font, binding.keyboard.label(), Vec2::new(25.0, y), FONT_SMALL, Color::srgb(1.0, 0.86, 0.42), MenuEntity);
        text(commands, font, binding.gamepad.label(), Vec2::new(235.0, y), FONT_SMALL, Color::srgb(0.65, 0.88, 1.0), MenuEntity);
    }
    let footer = if let Some((player, action, device)) = ui.capture {
        let who = if player == PlayerSlot::One { "P1" } else { "P2" };
        let device = if device == CaptureDevice::Keyboard { "键盘按键" } else { "手柄按键" };
        format!("等待输入：{who} {} 的{device} · Esc 取消", action.label())
    } else {
        "动作二 / Esc 返回设置".to_string()
    };
    text(commands, font, &footer, Vec2::new(0.0, -225.0), FONT_SMALL, Color::srgb(0.72, 0.9, 0.7), MenuEntity);
}

fn build_menu(
    commands: &mut Commands,
    font: &UiFont,
    save: &SaveData,
    selected: GameKind,
    ui: &MenuUiState,
) {
    spawn_backdrop(commands);
    match ui.page {
        MenuPage::Library => build_library(commands, font, save, selected, ui),
        MenuPage::Settings => build_settings(commands, font, save, ui),
        MenuPage::Controls => build_controls(commands, font, save, ui),
    }
}

pub(super) fn setup_menu(
    mut commands: Commands,
    save: Res<SaveData>,
    font: Res<UiFont>,
    selected: Res<SelectedGame>,
    mut ui: ResMut<MenuUiState>,
    mut music: MessageWriter<PlayMusic>,
) {
    ui.page = MenuPage::Library;
    ui.library_focus = LibraryFocus::Games;
    ui.capture = None;
    music.write(PlayMusic(MusicKind::Menu));
    build_menu(&mut commands, &font, &save, selected.0, &ui);
}

fn rebuild(
    commands: &mut Commands,
    entities: &Query<Entity, With<MenuEntity>>,
    font: &UiFont,
    save: &SaveData,
    selected: GameKind,
    ui: &MenuUiState,
) {
    for entity in entities.iter() {
        commands.entity(entity).try_despawn();
    }
    build_menu(commands, font, save, selected, ui);
}

fn apply_runtime_settings(
    save: &SaveData,
    input_bindings: &mut InputBindings,
    canvas: &mut PixelCanvasConfig,
    audio_mix: &mut AudioMix,
) {
    input_bindings.0 = save.settings.bindings;
    canvas.display_mode = save.settings.display_mode;
    canvas.crt_enabled = save.settings.crt_enabled;
    canvas.shake_enabled = save.settings.screen_shake;
    audio_mix.music_volume = save.settings.music_volume;
    audio_mix.sfx_volume = save.settings.sfx_volume;
}

fn clamp_selected_level(selected: GameKind, save: &mut SaveData) -> u8 {
    let index = selected.index();
    save.selected_levels[index] = save.selected_levels[index]
        .clamp(1, save.unlocked_levels[index].min(selected.max_level()));
    save.selected_levels[index]
}

fn launch_selected(
    selected: GameKind,
    save: &mut SaveData,
    next_state: &mut NextState<AppState>,
) {
    clamp_selected_level(selected, save);
    save.store();
    next_state.set(AppState::Playing);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn menu_input(
    mut commands: Commands,
    actions: Res<ActionState>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<SelectedGame>,
    mut save: ResMut<SaveData>,
    mut input_bindings: ResMut<InputBindings>,
    mut canvas: ResMut<PixelCanvasConfig>,
    mut audio_mix: ResMut<AudioMix>,
    font: Res<UiFont>,
    mut ui: ResMut<MenuUiState>,
    mut next_state: ResMut<NextState<AppState>>,
    entities: Query<Entity, With<MenuEntity>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if let Some((player, action, device)) = ui.capture {
        if keys.just_pressed(KeyCode::Escape) {
            ui.capture = None;
            rebuild(&mut commands, &entities, &font, &save, selected.0, &ui);
            return;
        }
        let captured = match device {
            CaptureDevice::Keyboard => {
                if let Some(key) = keys
                    .get_just_pressed()
                    .copied()
                    .find_map(KeyboardKey::from_key_code)
                {
                    save.settings.bindings[player.index()].rebind_keyboard(action, key);
                    true
                } else {
                    false
                }
            }
            CaptureDevice::Gamepad => {
                if let Some(button) = gamepads.iter().find_map(|gamepad| {
                    gamepad
                        .get_just_pressed()
                        .copied()
                        .find_map(PadButton::from_gamepad_button)
                }) {
                    save.settings.bindings[player.index()].rebind_gamepad(action, button);
                    true
                } else {
                    false
                }
            }
        };
        if captured {
            ui.capture = None;
            apply_runtime_settings(&save, &mut input_bindings, &mut canvas, &mut audio_mix);
            save.store();
            sfx.write(PlaySfx(SfxKind::MenuConfirm));
            rebuild(&mut commands, &entities, &font, &save, selected.0, &ui);
        }
        return;
    }

    let p1 = PlayerSlot::One;
    let up = actions.just_pressed(p1, InputAction::Up);
    let down = actions.just_pressed(p1, InputAction::Down);
    let left = actions.just_pressed(p1, InputAction::Left);
    let right = actions.just_pressed(p1, InputAction::Right);
    let primary = actions.just_pressed(p1, InputAction::Primary);
    let secondary = actions.just_pressed(p1, InputAction::Secondary);
    let start = actions.just_pressed(p1, InputAction::Start) || keys.just_pressed(KeyCode::Enter);
    let back = actions.just_pressed(p1, InputAction::Back) || keys.just_pressed(KeyCode::Escape);
    let reset = actions.just_pressed(p1, InputAction::Reset);
    let mut changed = false;
    let mut settings_changed = false;

    match ui.page {
        MenuPage::Library => match ui.library_focus {
            LibraryFocus::Games => {
                let index = selected.0.index();
                let mut next = index;
                if left && !index.is_multiple_of(2) { next = index - 1; }
                if right && index.is_multiple_of(2) { next = index + 1; }
                if up && index >= 2 { next = index - 2; }
                if down && index + 2 < GameKind::ALL.len() { next = index + 2; }
                if next != index {
                    selected.0 = GameKind::ALL[next];
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuMove));
                }
                if primary {
                    ui.library_focus = LibraryFocus::Stage;
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                } else if secondary {
                    ui.page = MenuPage::Settings;
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                } else if start {
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                    launch_selected(selected.0, &mut save, &mut next_state);
                    return;
                }
            }
            LibraryFocus::Stage => {
                let index = selected.0.index();
                let unlocked = save.unlocked_levels[index].min(selected.0.max_level());
                let mut level = save.selected_levels[index].clamp(1, unlocked);
                if left { level = level.saturating_sub(1).max(1); }
                if right { level = (level + 1).min(unlocked); }
                if level != save.selected_levels[index] {
                    save.selected_levels[index] = level;
                    save.store();
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuMove));
                }
                if primary || start {
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                    launch_selected(selected.0, &mut save, &mut next_state);
                    return;
                }
                if secondary || back || up || down {
                    ui.library_focus = LibraryFocus::Games;
                    changed = true;
                }
            }
        },
        MenuPage::Settings => {
            if up {
                ui.settings_cursor = ui.settings_cursor.saturating_sub(1);
                changed = true;
            }
            if down {
                ui.settings_cursor = (ui.settings_cursor + 1).min(7);
                changed = true;
            }
            let adjust = if left { -1 } else if right { 1 } else { 0 };
            let activate = primary || start;
            match ui.settings_cursor {
                0 if adjust != 0 || activate => {
                    save.settings.display_mode = save.settings.display_mode.toggled();
                    changed = true;
                    settings_changed = true;
                }
                1 if adjust != 0 || activate => {
                    save.settings.gameplay_profile = save.settings.gameplay_profile.toggled();
                    changed = true;
                    settings_changed = true;
                }
                2 if adjust != 0 || activate => {
                    save.settings.crt_enabled = !save.settings.crt_enabled;
                    changed = true;
                    settings_changed = true;
                }
                3 if adjust != 0 || activate => {
                    save.settings.screen_shake = !save.settings.screen_shake;
                    changed = true;
                    settings_changed = true;
                }
                4 if adjust != 0 => {
                    save.settings.music_volume = (save.settings.music_volume + adjust as f32 * 0.1).clamp(0.0, 1.0);
                    changed = true;
                    settings_changed = true;
                }
                5 if adjust != 0 => {
                    save.settings.sfx_volume = (save.settings.sfx_volume + adjust as f32 * 0.1).clamp(0.0, 1.0);
                    changed = true;
                    settings_changed = true;
                }
                6 if activate => {
                    ui.page = MenuPage::Controls;
                    changed = true;
                }
                7 if activate => {
                    ui.page = MenuPage::Library;
                    changed = true;
                }
                _ => {}
            }
            if secondary || back {
                ui.page = MenuPage::Library;
                changed = true;
            }
            if settings_changed {
                apply_runtime_settings(&save, &mut input_bindings, &mut canvas, &mut audio_mix);
                save.store();
            }
        }
        MenuPage::Controls => {
            if up {
                ui.control_cursor = ui.control_cursor.saturating_sub(1);
                changed = true;
            }
            if down {
                ui.control_cursor = (ui.control_cursor + 1).min(InputAction::COUNT - 1);
                changed = true;
            }
            if left && ui.control_device != CaptureDevice::Keyboard {
                ui.control_device = CaptureDevice::Keyboard;
                changed = true;
            }
            if right && ui.control_device != CaptureDevice::Gamepad {
                ui.control_device = CaptureDevice::Gamepad;
                changed = true;
            }
            if reset {
                ui.control_player = if ui.control_player == PlayerSlot::One { PlayerSlot::Two } else { PlayerSlot::One };
                changed = true;
            }
            if primary || start {
                ui.capture = Some((ui.control_player, InputAction::ALL[ui.control_cursor], ui.control_device));
                changed = true;
            }
            if secondary || back {
                ui.page = MenuPage::Settings;
                changed = true;
            }
        }
    }

    if changed {
        rebuild(&mut commands, &entities, &font, &save, selected.0, &ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_grid_fits_classic_width() {
        for index in 0..GameKind::ALL.len() {
            let center = card_center(index);
            assert!(center.x.abs() + 160.0 <= 360.0);
        }
    }

    #[test]
    fn selected_level_is_clamped_before_launch() {
        let kind = GameKind::SuperMario;
        let mut save = SaveData::default();
        save.unlocked_levels[kind.index()] = 3;
        save.selected_levels[kind.index()] = 9;
        clamp_selected_level(kind, &mut save);
        assert_eq!(save.selected_levels[kind.index()], 3);
    }
}
