use bevy::input::gamepad::Gamepad;
use bevy::prelude::*;

use crate::common::audio::{AudioMix, MusicKind, PlayMusic, PlaySfx, SfxKind};
use crate::common::constants::{ARENA_H, ARENA_W, FONT_BODY, FONT_TITLE};
use crate::common::px::px;
use crate::common::theme::{
    ACCENT, BG_DEEP, BORDER_DIM, SURFACE, SURFACE_SEL, TEXT_DIM, TEXT_MUTED, TEXT_PRIMARY,
};
use crate::common::input::ActionState;
use crate::common::pixel_canvas::PixelCanvasConfig;
use bevy::sprite::Anchor;
use crate::common::render::{UiFont, background_rect, panel, rect, text, text_anchored};
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

// 卡带墙的版面全部按画布像素设计（240×180），再用 px() 换算成世界单位。
// 直接写世界单位会得到 0.67 像素的描边和半像素的文字起笔。
const CARD_W: f32 = px(112.0);
const CARD_H: f32 = px(26.0);
const CARD_COL_X: f32 = px(58.0);
const CARD_ROW_TOP: f32 = px(49.0);
const CARD_ROW_STEP: f32 = px(28.0);
const PANEL_W: f32 = px(230.0);

fn spawn_backdrop(commands: &mut Commands) {
    background_rect(
        commands,
        Vec2::ZERO,
        Vec2::new(ARENA_W * 1.4, ARENA_H),
        BG_DEEP,
        MenuEntity,
    );
    // 12 像素一格的暗色网格，1 像素线宽——之前是 3 世界单位的线配 24 单位间距，
    // 换算到画布上是 1 像素线配 8 像素间距，密到发糊。
    let grid = Color::srgb(0.063, 0.078, 0.110);
    let step = px(12.0) as i32;
    for y in (-((ARENA_H * 0.5) as i32)..=((ARENA_H * 0.5) as i32)).step_by(step as usize) {
        rect(commands, Vec2::new(0.0, y as f32), Vec2::new(ARENA_W * 1.4, px(1.0)), grid, MenuEntity);
    }
    for x in (-672..=672).step_by(step as usize) {
        rect(commands, Vec2::new(x as f32, 0.0), Vec2::new(px(1.0), ARENA_H), grid, MenuEntity);
    }
}

fn card_center(index: usize) -> Vec2 {
    Vec2::new(
        if index.is_multiple_of(2) { -CARD_COL_X } else { CARD_COL_X },
        CARD_ROW_TOP - (index / 2) as f32 * CARD_ROW_STEP,
    )
}

/// 卡带封面图标：18×18 画布像素，所有零件都落在整像素网格上。
fn spawn_cover_icon(commands: &mut Commands, kind: GameKind, center: Vec2, accent: Color) {
    // 每个零件写成画布像素的 (dx, dy, w, h)，spawn 时统一 px() 换算。
    let dark = Color::srgb(0.10, 0.12, 0.16);
    let light = Color::srgb(0.96, 0.93, 0.85);
    let parts: &[(f32, f32, f32, f32, Color)] = match kind {
        GameKind::Tank => &[
            (0.0, -2.0, 11.0, 7.0, accent),
            (0.0, 3.0, 3.0, 6.0, accent),
            (-6.0, -1.0, 2.0, 11.0, dark),
            (6.0, -1.0, 2.0, 11.0, dark),
        ],
        GameKind::BombMaze => &[
            (0.0, -1.0, 11.0, 11.0, accent),
            (2.0, 6.0, 2.0, 4.0, dark),
            (4.0, 8.0, 3.0, 2.0, Color::srgb(1.0, 0.85, 0.3)),
        ],
        GameKind::SpaceShooter => &[
            (0.0, 1.0, 4.0, 13.0, accent),
            (0.0, -1.0, 14.0, 4.0, accent),
            (0.0, -7.0, 3.0, 3.0, Color::srgb(1.0, 0.72, 0.2)),
        ],
        GameKind::SuperMario => &[
            (0.0, 5.0, 10.0, 3.0, accent),
            (0.0, 1.0, 8.0, 5.0, Color::srgb(0.96, 0.72, 0.45)),
            (-2.0, -5.0, 3.0, 6.0, Color::srgb(0.24, 0.42, 0.9)),
            (2.0, -5.0, 3.0, 6.0, Color::srgb(0.24, 0.42, 0.9)),
        ],
        GameKind::Contra => &[
            (-2.0, 2.0, 6.0, 10.0, accent),
            (4.0, 3.0, 8.0, 2.0, Color::srgb(0.74, 0.78, 0.88)),
            (-4.0, -6.0, 3.0, 5.0, dark),
            (0.0, -6.0, 3.0, 5.0, dark),
        ],
        GameKind::BubbleBobble => &[
            (-4.0, 3.0, 7.0, 7.0, accent),
            (4.0, 3.0, 7.0, 7.0, Color::srgb(0.35, 0.8, 1.0)),
            (0.0, -4.0, 7.0, 7.0, Color::srgb(1.0, 0.75, 0.25)),
        ],
        GameKind::MemoryMatch => &[
            (-4.0, 0.0, 7.0, 11.0, accent),
            (4.0, 0.0, 7.0, 11.0, Color::srgb(0.9, 0.45, 0.55)),
            (-4.0, 0.0, 3.0, 5.0, light),
        ],
        GameKind::Sokoban => &[
            (0.0, 0.0, 13.0, 13.0, Color::srgb(0.55, 0.30, 0.12)),
            (0.0, 0.0, 9.0, 9.0, accent),
            (0.0, 0.0, 3.0, 3.0, Color::srgb(1.0, 0.93, 0.62)),
        ],
    };
    rect(commands, center, Vec2::splat(px(18.0)), Color::srgb(0.031, 0.039, 0.055), MenuEntity);
    for (dx, dy, w, h, color) in parts.iter().copied() {
        rect(
            commands,
            center + Vec2::new(px(dx), px(dy)),
            Vec2::new(px(w), px(h)),
            color,
            MenuEntity,
        );
    }
}

fn build_library(
    commands: &mut Commands,
    font: &UiFont,
    save: &SaveData,
    selected: GameKind,
    ui: &MenuUiState,
) {
    text(commands, font, "经典卡带墙", Vec2::new(0.0, px(78.0)), FONT_TITLE, ACCENT, MenuEntity);
    rect(commands, Vec2::new(0.0, px(65.0)), Vec2::new(PANEL_W, px(1.0)), BORDER_DIM, MenuEntity);

    for (index, kind) in GameKind::ALL.iter().copied().enumerate() {
        let center = card_center(index);
        let accent = kind.accent();
        let focused = kind == selected && ui.library_focus == LibraryFocus::Games;
        panel(
            commands,
            center,
            Vec2::new(CARD_W, CARD_H),
            if kind == selected { SURFACE_SEL } else { SURFACE },
            if focused { ACCENT } else { accent },
            MenuEntity,
        );
        spawn_cover_icon(commands, kind, center + Vec2::new(px(-45.0), 0.0), accent);
        text_anchored(
            commands, font, kind.short_title(),
            center + Vec2::new(px(-30.0), px(6.0)),
            FONT_BODY,
            if kind == selected { TEXT_PRIMARY } else { TEXT_MUTED },
            Anchor::CENTER_LEFT, MenuEntity,
        );
        let idx = kind.index();
        let info = format!(
            "{}/{}  {:06}",
            save.unlocked_levels[idx].min(kind.max_level()),
            kind.max_level(),
            save.high_scores[idx],
        );
        text_anchored(
            commands, font, &info,
            center + Vec2::new(px(-30.0), px(-6.0)),
            FONT_BODY, TEXT_DIM, Anchor::CENTER_LEFT, MenuEntity,
        );
    }

    let index = selected.index();
    let level = save.selected_levels[index].clamp(1, save.unlocked_levels[index]);
    let stage_focused = ui.library_focus == LibraryFocus::Stage;
    panel(
        commands,
        Vec2::new(0.0, px(-71.0)),
        Vec2::new(PANEL_W, px(38.0)),
        SURFACE,
        if stage_focused { ACCENT } else { BORDER_DIM },
        MenuEntity,
    );
    text(commands, font, selected.goal_text(), Vec2::new(0.0, px(-58.0)), FONT_BODY, TEXT_MUTED, MenuEntity);
    text(
        commands, font, &format!("◀  第 {level} 关  ▶"),
        Vec2::new(0.0, px(-71.0)), FONT_BODY,
        if stage_focused { ACCENT } else { TEXT_PRIMARY }, MenuEntity,
    );
    let hint = if stage_focused {
        "左右选关 · 开始键游玩"
    } else {
        "动作一选关 · 动作二设置"
    };
    text(commands, font, hint, Vec2::new(0.0, px(-84.0)), FONT_BODY, TEXT_DIM, MenuEntity);
}

fn build_settings(commands: &mut Commands, font: &UiFont, save: &SaveData, ui: &MenuUiState) {
    panel(commands, Vec2::ZERO, Vec2::new(px(224.0), px(164.0)), SURFACE, BORDER_DIM, MenuEntity);
    text(commands, font, "系统设置", Vec2::new(0.0, px(68.0)), FONT_TITLE, TEXT_PRIMARY, MenuEntity);
    let settings = &save.settings;
    let rows = [
        ("画面比例", settings.display_mode.label().to_string()),
        ("玩法模式", settings.gameplay_profile.label().to_string()),
        ("CRT 扫描线", if settings.crt_enabled { "开" } else { "关" }.to_string()),
        ("轻微震屏", if settings.screen_shake { "开" } else { "关" }.to_string()),
        ("音乐音量", format!("{}%", (settings.music_volume * 100.0).round() as i32)),
        ("音效音量", format!("{}%", (settings.sfx_volume * 100.0).round() as i32)),
        ("按键配置", "P1 / P2".to_string()),
        ("返回卡带墙", String::new()),
    ];
    for (index, (label, value)) in rows.iter().enumerate() {
        let y = px(48.0) - index as f32 * px(14.0);
        let on = index == ui.settings_cursor;
        if on {
            rect(commands, Vec2::new(0.0, y), Vec2::new(px(208.0), px(13.0)), SURFACE_SEL, MenuEntity);
            rect(commands, Vec2::new(px(-101.0), y), Vec2::new(px(2.0), px(11.0)), ACCENT, MenuEntity);
        }
        text_anchored(
            commands, font, label, Vec2::new(px(-94.0), y), FONT_BODY,
            if on { TEXT_PRIMARY } else { TEXT_MUTED }, Anchor::CENTER_LEFT, MenuEntity,
        );
        if !value.is_empty() {
            let value_color = if on { ACCENT } else { TEXT_MUTED };
            text(commands, font, value, Vec2::new(px(52.0), y), FONT_BODY, value_color, MenuEntity);
            if on {
                text(commands, font, "◀", Vec2::new(px(12.0), y), FONT_BODY, ACCENT, MenuEntity);
                text(commands, font, "▶", Vec2::new(px(92.0), y), FONT_BODY, ACCENT, MenuEntity);
            }
        }
    }
    text(commands, font, "上下选择 · 左右调整 · 动作二返回", Vec2::new(0.0, px(-70.0)), FONT_BODY, TEXT_DIM, MenuEntity);
}

fn build_controls(commands: &mut Commands, font: &UiFont, save: &SaveData, ui: &MenuUiState) {
    panel(commands, Vec2::ZERO, Vec2::new(px(232.0), px(174.0)), SURFACE, BORDER_DIM, MenuEntity);
    let player_label = if ui.control_player == PlayerSlot::One { "P1" } else { "P2" };
    text(commands, font, &format!("按键配置 · {player_label}"), Vec2::new(0.0, px(70.0)), FONT_TITLE, TEXT_PRIMARY, MenuEntity);

    // 列头，这样每行的手柄值就能去掉「手柄 」前缀，否则右列会顶出面板
    text(commands, font, "键盘", Vec2::new(px(24.0), px(50.0)), FONT_BODY, TEXT_DIM, MenuEntity);
    text(commands, font, "手柄", Vec2::new(px(84.0), px(50.0)), FONT_BODY, TEXT_DIM, MenuEntity);

    let bindings = save.settings.bindings[ui.control_player.index()];
    for (row, action) in InputAction::ALL.iter().copied().enumerate() {
        let y = px(36.0) - row as f32 * px(13.0);
        let on = row == ui.control_cursor;
        if on {
            let x = match ui.control_device {
                CaptureDevice::Keyboard => px(24.0),
                CaptureDevice::Gamepad => px(84.0),
            };
            rect(commands, Vec2::new(x, y), Vec2::new(px(52.0), px(12.0)), SURFACE_SEL, MenuEntity);
        }
        let binding = bindings.binding(action);
        text_anchored(
            commands, font, action.label(), Vec2::new(px(-108.0), y), FONT_BODY,
            if on { TEXT_PRIMARY } else { TEXT_MUTED }, Anchor::CENTER_LEFT, MenuEntity,
        );
        text(commands, font, binding.keyboard.label(), Vec2::new(px(24.0), y), FONT_BODY,
             if on && ui.control_device == CaptureDevice::Keyboard { ACCENT } else { TEXT_MUTED }, MenuEntity);
        let pad = binding.gamepad.label();
        let pad = pad.strip_prefix("手柄 ").unwrap_or(pad);
        text(commands, font, pad, Vec2::new(px(84.0), y), FONT_BODY,
             if on && ui.control_device == CaptureDevice::Gamepad { ACCENT } else { TEXT_MUTED }, MenuEntity);
    }
    let footer = if let Some((_, action, device)) = ui.capture {
        let device = if device == CaptureDevice::Keyboard { "键盘" } else { "手柄" };
        format!("等待{device}输入：{} · Esc 取消", action.label())
    } else {
        "重置切玩家 · 动作一修改 · 动作二返回".to_string()
    };
    text(commands, font, &footer, Vec2::new(0.0, px(-80.0)), FONT_BODY,
         if ui.capture.is_some() { ACCENT } else { TEXT_DIM }, MenuEntity);
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
