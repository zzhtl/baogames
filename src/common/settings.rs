//! 可持久化的显示、玩法与输入设置。
//!
//! 这里使用自有的键位枚举，而不是直接序列化 Bevy 的输入类型，避免 Bevy
//! 升级导致存档线格式变化。运行时再显式转换为 `KeyCode` / `GamepadButton`。

use bevy::input::gamepad::GamepadButton;
use bevy::prelude::{KeyCode, Resource};
use serde::{Deserialize, Serialize};

pub const PLAYER_COUNT: usize = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayMode {
    #[default]
    Classic4x3,
    Widescreen16x9,
}

impl DisplayMode {
    pub const fn canvas_size(self) -> (u32, u32) {
        match self {
            Self::Classic4x3 => (240, 180),
            Self::Widescreen16x9 => (320, 180),
        }
    }

    pub const fn world_width(self) -> f32 {
        match self {
            Self::Classic4x3 => 720.0,
            Self::Widescreen16x9 => 960.0,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic4x3 => "经典 4:3",
            Self::Widescreen16x9 => "宽屏 16:9",
        }
    }

    pub const fn toggled(self) -> Self {
        match self {
            Self::Classic4x3 => Self::Widescreen16x9,
            Self::Widescreen16x9 => Self::Classic4x3,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameplayProfile {
    #[default]
    Classic,
    Assist,
}

impl GameplayProfile {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "经典",
            Self::Assist => "辅助",
        }
    }

    pub const fn toggled(self) -> Self {
        match self {
            Self::Classic => Self::Assist,
            Self::Assist => Self::Classic,
        }
    }

    pub const fn jump_buffer_ticks(self) -> u8 {
        match self {
            Self::Classic => 1,
            Self::Assist => 6,
        }
    }

    pub const fn coyote_ticks(self) -> u8 {
        match self {
            Self::Classic => 0,
            Self::Assist => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerSlot {
    One,
    Two,
}

impl PlayerSlot {
    pub const ALL: [Self; PLAYER_COUNT] = [Self::One, Self::Two];

    pub const fn index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        if index == 0 { Self::One } else { Self::Two }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputAction {
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
    Start,
    Back,
    Reset,
}

impl InputAction {
    pub const COUNT: usize = 9;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Primary,
        Self::Secondary,
        Self::Start,
        Self::Back,
        Self::Reset,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
            Self::Primary => 4,
            Self::Secondary => 5,
            Self::Start => 6,
            Self::Back => 7,
            Self::Reset => 8,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Up => "上",
            Self::Down => "下",
            Self::Left => "左",
            Self::Right => "右",
            Self::Primary => "动作一",
            Self::Secondary => "动作二",
            Self::Start => "开始/暂停",
            Self::Back => "返回",
            Self::Reset => "重置",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Backspace,
    Tab,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    Delete,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadEnter,
}

impl KeyboardKey {
    pub const fn key_code(self) -> KeyCode {
        match self {
            Self::A => KeyCode::KeyA,
            Self::B => KeyCode::KeyB,
            Self::C => KeyCode::KeyC,
            Self::D => KeyCode::KeyD,
            Self::E => KeyCode::KeyE,
            Self::F => KeyCode::KeyF,
            Self::G => KeyCode::KeyG,
            Self::H => KeyCode::KeyH,
            Self::I => KeyCode::KeyI,
            Self::J => KeyCode::KeyJ,
            Self::K => KeyCode::KeyK,
            Self::L => KeyCode::KeyL,
            Self::M => KeyCode::KeyM,
            Self::N => KeyCode::KeyN,
            Self::O => KeyCode::KeyO,
            Self::P => KeyCode::KeyP,
            Self::Q => KeyCode::KeyQ,
            Self::R => KeyCode::KeyR,
            Self::S => KeyCode::KeyS,
            Self::T => KeyCode::KeyT,
            Self::U => KeyCode::KeyU,
            Self::V => KeyCode::KeyV,
            Self::W => KeyCode::KeyW,
            Self::X => KeyCode::KeyX,
            Self::Y => KeyCode::KeyY,
            Self::Z => KeyCode::KeyZ,
            Self::Digit0 => KeyCode::Digit0,
            Self::Digit1 => KeyCode::Digit1,
            Self::Digit2 => KeyCode::Digit2,
            Self::Digit3 => KeyCode::Digit3,
            Self::Digit4 => KeyCode::Digit4,
            Self::Digit5 => KeyCode::Digit5,
            Self::Digit6 => KeyCode::Digit6,
            Self::Digit7 => KeyCode::Digit7,
            Self::Digit8 => KeyCode::Digit8,
            Self::Digit9 => KeyCode::Digit9,
            Self::Up => KeyCode::ArrowUp,
            Self::Down => KeyCode::ArrowDown,
            Self::Left => KeyCode::ArrowLeft,
            Self::Right => KeyCode::ArrowRight,
            Self::Space => KeyCode::Space,
            Self::Enter => KeyCode::Enter,
            Self::Backspace => KeyCode::Backspace,
            Self::Tab => KeyCode::Tab,
            Self::LeftShift => KeyCode::ShiftLeft,
            Self::RightShift => KeyCode::ShiftRight,
            Self::LeftControl => KeyCode::ControlLeft,
            Self::RightControl => KeyCode::ControlRight,
            Self::Delete => KeyCode::Delete,
            Self::Numpad0 => KeyCode::Numpad0,
            Self::Numpad1 => KeyCode::Numpad1,
            Self::Numpad2 => KeyCode::Numpad2,
            Self::Numpad3 => KeyCode::Numpad3,
            Self::Numpad4 => KeyCode::Numpad4,
            Self::Numpad5 => KeyCode::Numpad5,
            Self::Numpad6 => KeyCode::Numpad6,
            Self::Numpad7 => KeyCode::Numpad7,
            Self::Numpad8 => KeyCode::Numpad8,
            Self::Numpad9 => KeyCode::Numpad9,
            Self::NumpadEnter => KeyCode::NumpadEnter,
        }
    }

    pub fn from_key_code(code: KeyCode) -> Option<Self> {
        Some(match code {
            KeyCode::KeyA => Self::A,
            KeyCode::KeyB => Self::B,
            KeyCode::KeyC => Self::C,
            KeyCode::KeyD => Self::D,
            KeyCode::KeyE => Self::E,
            KeyCode::KeyF => Self::F,
            KeyCode::KeyG => Self::G,
            KeyCode::KeyH => Self::H,
            KeyCode::KeyI => Self::I,
            KeyCode::KeyJ => Self::J,
            KeyCode::KeyK => Self::K,
            KeyCode::KeyL => Self::L,
            KeyCode::KeyM => Self::M,
            KeyCode::KeyN => Self::N,
            KeyCode::KeyO => Self::O,
            KeyCode::KeyP => Self::P,
            KeyCode::KeyQ => Self::Q,
            KeyCode::KeyR => Self::R,
            KeyCode::KeyS => Self::S,
            KeyCode::KeyT => Self::T,
            KeyCode::KeyU => Self::U,
            KeyCode::KeyV => Self::V,
            KeyCode::KeyW => Self::W,
            KeyCode::KeyX => Self::X,
            KeyCode::KeyY => Self::Y,
            KeyCode::KeyZ => Self::Z,
            KeyCode::Digit0 => Self::Digit0,
            KeyCode::Digit1 => Self::Digit1,
            KeyCode::Digit2 => Self::Digit2,
            KeyCode::Digit3 => Self::Digit3,
            KeyCode::Digit4 => Self::Digit4,
            KeyCode::Digit5 => Self::Digit5,
            KeyCode::Digit6 => Self::Digit6,
            KeyCode::Digit7 => Self::Digit7,
            KeyCode::Digit8 => Self::Digit8,
            KeyCode::Digit9 => Self::Digit9,
            KeyCode::ArrowUp => Self::Up,
            KeyCode::ArrowDown => Self::Down,
            KeyCode::ArrowLeft => Self::Left,
            KeyCode::ArrowRight => Self::Right,
            KeyCode::Space => Self::Space,
            KeyCode::Enter => Self::Enter,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Tab => Self::Tab,
            KeyCode::ShiftLeft => Self::LeftShift,
            KeyCode::ShiftRight => Self::RightShift,
            KeyCode::ControlLeft => Self::LeftControl,
            KeyCode::ControlRight => Self::RightControl,
            KeyCode::Delete => Self::Delete,
            KeyCode::Numpad0 => Self::Numpad0,
            KeyCode::Numpad1 => Self::Numpad1,
            KeyCode::Numpad2 => Self::Numpad2,
            KeyCode::Numpad3 => Self::Numpad3,
            KeyCode::Numpad4 => Self::Numpad4,
            KeyCode::Numpad5 => Self::Numpad5,
            KeyCode::Numpad6 => Self::Numpad6,
            KeyCode::Numpad7 => Self::Numpad7,
            KeyCode::Numpad8 => Self::Numpad8,
            KeyCode::Numpad9 => Self::Numpad9,
            KeyCode::NumpadEnter => Self::NumpadEnter,
            _ => return None,
        })
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::A => "A", Self::B => "B", Self::C => "C", Self::D => "D",
            Self::E => "E", Self::F => "F", Self::G => "G", Self::H => "H",
            Self::I => "I", Self::J => "J", Self::K => "K", Self::L => "L",
            Self::M => "M", Self::N => "N", Self::O => "O", Self::P => "P",
            Self::Q => "Q", Self::R => "R", Self::S => "S", Self::T => "T",
            Self::U => "U", Self::V => "V", Self::W => "W", Self::X => "X",
            Self::Y => "Y", Self::Z => "Z",
            Self::Digit0 => "0", Self::Digit1 => "1", Self::Digit2 => "2",
            Self::Digit3 => "3", Self::Digit4 => "4", Self::Digit5 => "5",
            Self::Digit6 => "6", Self::Digit7 => "7", Self::Digit8 => "8",
            Self::Digit9 => "9",
            Self::Up => "↑", Self::Down => "↓", Self::Left => "←", Self::Right => "→",
            Self::Space => "空格", Self::Enter => "Enter", Self::Backspace => "退格",
            Self::Tab => "Tab", Self::LeftShift => "左 Shift",
            Self::RightShift => "右 Shift", Self::LeftControl => "左 Ctrl",
            Self::RightControl => "右 Ctrl", Self::Delete => "Delete",
            Self::Numpad0 => "小键盘 0", Self::Numpad1 => "小键盘 1",
            Self::Numpad2 => "小键盘 2", Self::Numpad3 => "小键盘 3",
            Self::Numpad4 => "小键盘 4", Self::Numpad5 => "小键盘 5",
            Self::Numpad6 => "小键盘 6", Self::Numpad7 => "小键盘 7",
            Self::Numpad8 => "小键盘 8", Self::Numpad9 => "小键盘 9",
            Self::NumpadEnter => "小键盘 Enter",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PadButton {
    South,
    East,
    North,
    West,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

impl PadButton {
    pub fn from_gamepad_button(button: GamepadButton) -> Option<Self> {
        Some(match button {
            GamepadButton::South => Self::South,
            GamepadButton::East => Self::East,
            GamepadButton::North => Self::North,
            GamepadButton::West => Self::West,
            GamepadButton::LeftTrigger => Self::LeftTrigger,
            GamepadButton::LeftTrigger2 => Self::LeftTrigger2,
            GamepadButton::RightTrigger => Self::RightTrigger,
            GamepadButton::RightTrigger2 => Self::RightTrigger2,
            GamepadButton::Select => Self::Select,
            GamepadButton::Start => Self::Start,
            GamepadButton::LeftThumb => Self::LeftThumb,
            GamepadButton::RightThumb => Self::RightThumb,
            GamepadButton::DPadUp => Self::DPadUp,
            GamepadButton::DPadDown => Self::DPadDown,
            GamepadButton::DPadLeft => Self::DPadLeft,
            GamepadButton::DPadRight => Self::DPadRight,
            _ => return None,
        })
    }

    pub const fn gamepad_button(self) -> GamepadButton {
        match self {
            Self::South => GamepadButton::South,
            Self::East => GamepadButton::East,
            Self::North => GamepadButton::North,
            Self::West => GamepadButton::West,
            Self::LeftTrigger => GamepadButton::LeftTrigger,
            Self::LeftTrigger2 => GamepadButton::LeftTrigger2,
            Self::RightTrigger => GamepadButton::RightTrigger,
            Self::RightTrigger2 => GamepadButton::RightTrigger2,
            Self::Select => GamepadButton::Select,
            Self::Start => GamepadButton::Start,
            Self::LeftThumb => GamepadButton::LeftThumb,
            Self::RightThumb => GamepadButton::RightThumb,
            Self::DPadUp => GamepadButton::DPadUp,
            Self::DPadDown => GamepadButton::DPadDown,
            Self::DPadLeft => GamepadButton::DPadLeft,
            Self::DPadRight => GamepadButton::DPadRight,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::South => "手柄 A/×", Self::East => "手柄 B/○",
            Self::North => "手柄 Y/△", Self::West => "手柄 X/□",
            Self::LeftTrigger => "手柄 LB", Self::LeftTrigger2 => "手柄 LT",
            Self::RightTrigger => "手柄 RB", Self::RightTrigger2 => "手柄 RT",
            Self::Select => "手柄 Select", Self::Start => "手柄 Start",
            Self::LeftThumb => "左摇杆按下", Self::RightThumb => "右摇杆按下",
            Self::DPadUp => "十字键上", Self::DPadDown => "十字键下",
            Self::DPadLeft => "十字键左", Self::DPadRight => "十字键右",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBinding {
    pub keyboard: KeyboardKey,
    pub gamepad: PadButton,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerBindings {
    pub actions: [ActionBinding; InputAction::COUNT],
}

impl PlayerBindings {
    pub const fn binding(self, action: InputAction) -> ActionBinding {
        self.actions[action.index()]
    }

    pub fn rebind_keyboard(&mut self, action: InputAction, key: KeyboardKey) {
        let target = action.index();
        let old = self.actions[target].keyboard;
        if let Some(conflict) = self.actions.iter().position(|binding| binding.keyboard == key) {
            self.actions[conflict].keyboard = old;
        }
        self.actions[target].keyboard = key;
    }

    pub fn rebind_gamepad(&mut self, action: InputAction, button: PadButton) {
        let target = action.index();
        let old = self.actions[target].gamepad;
        if let Some(conflict) = self.actions.iter().position(|binding| binding.gamepad == button) {
            self.actions[conflict].gamepad = old;
        }
        self.actions[target].gamepad = button;
    }
}

impl Default for PlayerBindings {
    fn default() -> Self {
        Self::for_player(PlayerSlot::One)
    }
}

impl PlayerBindings {
    pub const fn for_player(player: PlayerSlot) -> Self {
        let keyboard = match player {
            PlayerSlot::One => [
                KeyboardKey::W,
                KeyboardKey::S,
                KeyboardKey::A,
                KeyboardKey::D,
                KeyboardKey::J,
                KeyboardKey::K,
                KeyboardKey::Enter,
                KeyboardKey::Backspace,
                KeyboardKey::R,
            ],
            PlayerSlot::Two => [
                KeyboardKey::Up,
                KeyboardKey::Down,
                KeyboardKey::Left,
                KeyboardKey::Right,
                KeyboardKey::Numpad1,
                KeyboardKey::Numpad2,
                KeyboardKey::NumpadEnter,
                KeyboardKey::Delete,
                KeyboardKey::Numpad0,
            ],
        };
        let gamepad = [
            PadButton::DPadUp,
            PadButton::DPadDown,
            PadButton::DPadLeft,
            PadButton::DPadRight,
            PadButton::South,
            PadButton::East,
            PadButton::Start,
            PadButton::Select,
            PadButton::North,
        ];
        let mut actions = [ActionBinding {
            keyboard: KeyboardKey::A,
            gamepad: PadButton::South,
        }; InputAction::COUNT];
        let mut i = 0;
        while i < InputAction::COUNT {
            actions[i] = ActionBinding {
                keyboard: keyboard[i],
                gamepad: gamepad[i],
            };
            i += 1;
        }
        Self { actions }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    pub display_mode: DisplayMode,
    pub gameplay_profile: GameplayProfile,
    pub crt_enabled: bool,
    pub screen_shake: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub bindings: [PlayerBindings; PLAYER_COUNT],
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Classic4x3,
            gameplay_profile: GameplayProfile::Classic,
            crt_enabled: false,
            screen_shake: false,
            music_volume: 0.7,
            sfx_volume: 0.7,
            bindings: [
                PlayerBindings::for_player(PlayerSlot::One),
                PlayerBindings::for_player(PlayerSlot::Two),
            ],
        }
    }
}

impl UserSettings {
    pub fn sanitize(&mut self) {
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
    }
}

#[derive(Resource, Clone, Copy)]
pub struct InputBindings(pub [PlayerBindings; PLAYER_COUNT]);

impl Default for InputBindings {
    fn default() -> Self {
        Self(UserSettings::default().bindings)
    }
}

impl From<&UserSettings> for InputBindings {
    fn from(settings: &UserSettings) -> Self {
        Self(settings.bindings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_modes_have_expected_canvas_and_world_width() {
        assert_eq!(DisplayMode::Classic4x3.canvas_size(), (240, 180));
        assert_eq!(DisplayMode::Classic4x3.world_width(), 720.0);
        assert_eq!(DisplayMode::Widescreen16x9.canvas_size(), (320, 180));
        assert_eq!(DisplayMode::Widescreen16x9.world_width(), 960.0);
    }

    #[test]
    fn assist_profile_uses_agreed_jump_forgiveness() {
        assert_eq!(GameplayProfile::Classic.jump_buffer_ticks(), 1);
        assert_eq!(GameplayProfile::Classic.coyote_ticks(), 0);
        assert_eq!(GameplayProfile::Assist.jump_buffer_ticks(), 6);
        assert_eq!(GameplayProfile::Assist.coyote_ticks(), 5);
    }

    #[test]
    fn keyboard_rebind_swaps_conflict() {
        let mut bindings = PlayerBindings::for_player(PlayerSlot::One);
        bindings.rebind_keyboard(InputAction::Primary, KeyboardKey::K);
        assert_eq!(bindings.binding(InputAction::Primary).keyboard, KeyboardKey::K);
        assert_eq!(bindings.binding(InputAction::Secondary).keyboard, KeyboardKey::J);
    }

    #[test]
    fn gamepad_rebind_swaps_conflict() {
        let mut bindings = PlayerBindings::for_player(PlayerSlot::One);
        bindings.rebind_gamepad(InputAction::Primary, PadButton::East);
        assert_eq!(bindings.binding(InputAction::Primary).gamepad, PadButton::East);
        assert_eq!(bindings.binding(InputAction::Secondary).gamepad, PadButton::South);
    }

    #[test]
    fn settings_sanitize_volumes() {
        let mut settings = UserSettings {
            music_volume: 2.0,
            sfx_volume: -1.0,
            ..UserSettings::default()
        };
        settings.sanitize();
        assert_eq!(settings.music_volume, 1.0);
        assert_eq!(settings.sfx_volume, 0.0);
    }

    #[test]
    fn supported_key_codes_round_trip() {
        for key in [
            KeyboardKey::W,
            KeyboardKey::Space,
            KeyboardKey::Up,
            KeyboardKey::Numpad1,
        ] {
            assert_eq!(KeyboardKey::from_key_code(key.key_code()), Some(key));
        }
    }
}
