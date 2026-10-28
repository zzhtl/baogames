use bevy::input::gamepad::Gamepad;
use bevy::prelude::*;

use crate::common::audio::{AudioMix, MusicKind, PlayMusic, PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::pixel_canvas::PixelCanvasConfig;
use crate::common::settings::{
    InputAction, InputBindings, KeyboardKey, PadButton, PlayerSlot,
};

use super::model::{AppState, GameCategory, GameKind, MenuEntity, SaveData, SelectedGame};

mod icons;
mod view;
const GAMES_PER_PAGE: usize = 8;
use view::{MenuTarget, build_menu};
pub(super) use view::{MenuArt, MenuCamera, MenuPointer, menu_pointer, reset_cursor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum MenuPage {
    #[default]
    Library,
    Settings,
    Controls,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LibraryFocus {
    /// 顶部的「经典 | 益智」分类页签。
    Tabs,
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
    category: GameCategory,
    library_focus: LibraryFocus,
    /// 详情面板里的行游标：0 = 关卡，1 = 年龄档（仅益智栏有第 1 行）。
    stage_row: usize,
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
            category: GameCategory::Classic,
            library_focus: LibraryFocus::Games,
            stage_row: 0,
            settings_cursor: 0,
            control_player: PlayerSlot::One,
            control_cursor: 0,
            control_device: CaptureDevice::Keyboard,
            capture: None,
        }
    }
}

/// 在 2 列 × N 行的卡带网格里走一步。
///
/// 全程 clamp，返回值恒在 `0..count` —— 分栏之后下标基准从「全局 0..15」变成
/// 「栏内 0..8」，越界不再是显示错乱而是直接 panic，所以这里不留任何出口。
fn step_slot(slot: usize, dx: isize, dy: isize, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let rows = count.div_ceil(2) as isize;
    let col = ((slot % 2) as isize + dx).clamp(0, 1);
    let row = ((slot / 2) as isize + dy).clamp(0, rows - 1);
    ((row * 2 + col) as usize).min(count - 1)
}

fn page_slot(slot: usize, delta: i8, count: usize) -> usize {
    if count == 0 { return 0; }
    let page = ((slot / GAMES_PER_PAGE) as isize + delta as isize)
        .clamp(0, (count - 1).div_euclid(GAMES_PER_PAGE) as isize) as usize;
    (page * GAMES_PER_PAGE + slot % GAMES_PER_PAGE).min(count - 1)
}

pub(super) fn setup_menu(
    mut commands: Commands,
    save: Res<SaveData>,
    font: Res<MenuArt>,
    selected: Res<SelectedGame>,
    mut ui: ResMut<MenuUiState>,
    mut music: MessageWriter<PlayMusic>,
) {
    ui.page = MenuPage::Library;
    ui.category = selected.0.category();
    ui.library_focus = LibraryFocus::Games;
    ui.stage_row = 0;
    ui.capture = None;
    view::setup_camera(&mut commands);
    music.write(PlayMusic(MusicKind::Menu));
    build_menu(&mut commands, &font, &save, selected.0, &ui);
}

fn rebuild(
    commands: &mut Commands,
    entities: &Query<Entity, With<MenuEntity>>,
    font: &MenuArt,
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

fn arrow_shortcut(keys: &ButtonInput<KeyCode>, bindings: &InputBindings, key: KeyCode) -> bool {
    // 已重映射的方向键交给语义输入，避免一次按键同时触发导航和确认等动作。
    keys.just_pressed(key) && !InputAction::ALL.iter().any(|action| {
        bindings.0[PlayerSlot::One.index()].binding(*action).keyboard.key_code() == key
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn menu_input(
    mut commands: Commands,
    actions: Res<ActionState>,
    keys: Res<ButtonInput<KeyCode>>,
    pointer: Res<MenuPointer>,
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<SelectedGame>,
    mut save: ResMut<SaveData>,
    mut input_bindings: ResMut<InputBindings>,
    mut canvas: ResMut<PixelCanvasConfig>,
    mut audio_mix: ResMut<AudioMix>,
    font: Res<MenuArt>,
    mut ui: ResMut<MenuUiState>,
    mut next_state: ResMut<NextState<AppState>>,
    entities: Query<Entity, With<MenuEntity>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if let Some((player, action, device)) = ui.capture {
        if keys.just_pressed(KeyCode::Escape) || pointer.clicked == Some(MenuTarget::Back) {
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
    let up = actions.just_pressed(p1, InputAction::Up) || arrow_shortcut(&keys, &input_bindings, KeyCode::ArrowUp);
    let down = actions.just_pressed(p1, InputAction::Down) || arrow_shortcut(&keys, &input_bindings, KeyCode::ArrowDown);
    let mut left = actions.just_pressed(p1, InputAction::Left) || arrow_shortcut(&keys, &input_bindings, KeyCode::ArrowLeft);
    let mut right = actions.just_pressed(p1, InputAction::Right) || arrow_shortcut(&keys, &input_bindings, KeyCode::ArrowRight);
    let mut primary = actions.just_pressed(p1, InputAction::Primary);
    let mut secondary = actions.just_pressed(p1, InputAction::Secondary);
    let mut start = actions.just_pressed(p1, InputAction::Start) || keys.just_pressed(KeyCode::Enter);
    let back = actions.just_pressed(p1, InputAction::Back) || keys.just_pressed(KeyCode::Escape);
    let mut reset = actions.just_pressed(p1, InputAction::Reset);
    let mut changed = false;
    let mut settings_changed = false;

    if let Some(target) = pointer.clicked {
        match target {
            MenuTarget::Game(kind) => {
                selected.0 = kind;
                ui.library_focus = LibraryFocus::Games;
                sfx.write(PlaySfx(SfxKind::MenuMove));
            }
            MenuTarget::Category(category) => {
                ui.category = category;
                if selected.0.category() != category { selected.0 = category.games()[0]; }
                ui.library_focus = LibraryFocus::Games;
                sfx.write(PlaySfx(SfxKind::MenuMove));
            }
            MenuTarget::Settings => ui.page = MenuPage::Settings,
            MenuTarget::Launch => start = true,
            MenuTarget::Stage(delta) => {
                ui.library_focus = LibraryFocus::Stage;
                ui.stage_row = 0;
                left = delta < 0;
                right = delta > 0;
            }
            MenuTarget::Age => {
                ui.library_focus = LibraryFocus::Stage;
                ui.stage_row = 1;
                right = true;
            }
            MenuTarget::Setting(row, delta) => {
                ui.settings_cursor = row;
                left = delta < 0;
                right = delta > 0;
                primary = delta == 0;
            }
            MenuTarget::Back => secondary = true,
            MenuTarget::Binding(row, device) => {
                ui.control_cursor = row;
                ui.control_device = device;
                primary = true;
            }
            MenuTarget::Player => reset = true,
            MenuTarget::Page(delta) => {
                let games = ui.category.games();
                let slot = games.iter().position(|kind| *kind == selected.0).unwrap_or(0);
                selected.0 = games[page_slot(slot, delta, games.len())];
                ui.library_focus = LibraryFocus::Games;
                sfx.write(PlaySfx(SfxKind::MenuMove));
            }
        }
        changed = true;
    }
    if ui.page == MenuPage::Library && keys.just_pressed(KeyCode::Tab) {
        let slot = ui.category.games().iter().position(|kind| *kind == selected.0).unwrap_or(0);
        ui.category = ui.category.toggled();
        selected.0 = ui.category.games()[slot.min(ui.category.games().len() - 1)];
        ui.library_focus = LibraryFocus::Games;
        ui.stage_row = 0;
        changed = true;
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }
    if ui.page == MenuPage::Library && (keys.just_pressed(KeyCode::PageDown) || keys.just_pressed(KeyCode::PageUp)) {
        let games = ui.category.games();
        let slot = games.iter().position(|kind| *kind == selected.0).unwrap_or(0);
        let delta = if keys.just_pressed(KeyCode::PageDown) { 1 } else { -1 };
        selected.0 = games[page_slot(slot, delta, games.len())];
        ui.library_focus = LibraryFocus::Games;
        changed = true;
    }

    match ui.page {
        MenuPage::Library => match ui.library_focus {
            LibraryFocus::Tabs => {
                if left || right {
                    let games = ui.category.games();
                    let slot = games.iter().position(|kind| *kind == selected.0).unwrap_or(0);
                    let target = ui.category.toggled();
                    // 空栏不切：否则会停在一面没有卡带、选中项还属于另一栏的墙上。
                    if let Some(&kind) = target.games().get(slot).or_else(|| target.games().first()) {
                        ui.category = target;
                        selected.0 = kind;
                        changed = true;
                        sfx.write(PlaySfx(SfxKind::MenuMove));
                    }
                }
                if down || primary {
                    ui.library_focus = LibraryFocus::Games;
                    changed = true;
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
            LibraryFocus::Games => {
                let games = ui.category.games();
                let slot = games.iter().position(|kind| *kind == selected.0).unwrap_or(0);
                let next = step_slot(
                    slot,
                    right as isize - left as isize,
                    down as isize - up as isize,
                    games.len(),
                );
                if next != slot {
                    selected.0 = games[next];
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuMove));
                } else if up && slot < 2 {
                    ui.library_focus = LibraryFocus::Tabs;
                    changed = true;
                    sfx.write(PlaySfx(SfxKind::MenuMove));
                }
                if primary {
                    ui.library_focus = LibraryFocus::Stage;
                    ui.stage_row = 0;
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
                let puzzle = ui.category == GameCategory::Puzzle;
                // 益智栏第 1 行是年龄档；经典栏只有关卡一行，上下直接退回卡带网格。
                if up && (!puzzle || ui.stage_row == 0) {
                    ui.library_focus = LibraryFocus::Games;
                    changed = true;
                } else if puzzle && (up || down) {
                    let row = usize::from(down);
                    if row != ui.stage_row {
                        ui.stage_row = row;
                        changed = true;
                        sfx.write(PlaySfx(SfxKind::MenuMove));
                    }
                } else if down {
                    ui.library_focus = LibraryFocus::Games;
                    changed = true;
                }

                if puzzle && ui.stage_row == 1 {
                    if left || right {
                        save.age_tier = save.age_tier.toggled();
                        save.store();
                        changed = true;
                        sfx.write(PlaySfx(SfxKind::MenuConfirm));
                    }
                } else {
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
                }

                if primary || start {
                    sfx.write(PlaySfx(SfxKind::MenuConfirm));
                    launch_selected(selected.0, &mut save, &mut next_state);
                    return;
                }
                if secondary || back {
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
    fn arrow_navigation_respects_custom_bindings() {
        let mut keys = ButtonInput::default();
        let mut bindings = InputBindings::default();
        keys.press(KeyCode::ArrowRight);
        assert!(arrow_shortcut(&keys, &bindings, KeyCode::ArrowRight));
        bindings.0[0].rebind_keyboard(InputAction::Primary, KeyboardKey::Right);
        assert!(!arrow_shortcut(&keys, &bindings, KeyCode::ArrowRight));
        keys.release(KeyCode::ArrowRight);
        assert!(!arrow_shortcut(&keys, &bindings, KeyCode::ArrowLeft));
    }

    #[test]
    fn pages_cover_every_game_and_clamp_the_last_page() {
        for category in GameCategory::ALL {
            for slot in 0..category.games().len() {
                for delta in [-1, 1] {
                    assert!(page_slot(slot, delta, category.games().len()) < category.games().len());
                }
            }
        }
        assert_eq!(page_slot(0, -1, 18), 0);
        assert_eq!(page_slot(1, 1, 18), 9);
        assert_eq!(page_slot(15, 1, 18), 17);
        assert_eq!(page_slot(17, 1, 18), 17);
    }

    #[test]
    fn slot_navigation_never_escapes_the_tab() {
        for count in 1..=GameKind::ALL.len() {
            for slot in 0..count {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    assert!(
                        step_slot(slot, dx, dy, count) < count,
                        "count={count} slot={slot} dir=({dx},{dy}) 越界"
                    );
                }
            }
        }
        assert_eq!(step_slot(0, 0, 0, 0), 0);
        // 2 列网格：左上角按右到 1，按下到 2
        assert_eq!(step_slot(0, 1, 0, 8), 1);
        assert_eq!(step_slot(0, 0, 1, 8), 2);
        // 奇数张卡时最后一行只有左列，按右必须停在原地而不是越界
        assert_eq!(step_slot(6, 1, 0, 7), 6);
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
