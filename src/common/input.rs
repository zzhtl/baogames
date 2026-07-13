use bevy::input::InputSystems;
use bevy::input::gamepad::{Gamepad, GamepadAxis};
use bevy::prelude::*;

use super::settings::{
    InputAction, InputBindings, PLAYER_COUNT, PlayerBindings, PlayerSlot,
};

const GAMEPAD_AXIS_THRESHOLD: f32 = 0.25;

#[derive(Clone, Copy, Default)]
struct ActionButtonState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool,
}

#[derive(Clone, Copy)]
struct PlayerActionState {
    buttons: [ActionButtonState; InputAction::COUNT],
}

impl Default for PlayerActionState {
    fn default() -> Self {
        Self {
            buttons: [ActionButtonState::default(); InputAction::COUNT],
        }
    }
}

/// 八款游戏共享的语义动作快照。
///
/// 菜单和普通 `Update` 系统可直接读取；固定时间步系统应把 `just_pressed`
/// 锁存到自己的输入缓冲，保证一帧内多次 fixed tick 只消费一次。
#[derive(Resource)]
pub struct ActionState {
    players: [PlayerActionState; PLAYER_COUNT],
}

impl Default for ActionState {
    fn default() -> Self {
        Self {
            players: [PlayerActionState::default(); PLAYER_COUNT],
        }
    }
}

impl ActionState {
    pub fn pressed(&self, player: PlayerSlot, action: InputAction) -> bool {
        self.players[player.index()].buttons[action.index()].pressed
    }

    pub fn just_pressed(&self, player: PlayerSlot, action: InputAction) -> bool {
        self.players[player.index()].buttons[action.index()].just_pressed
    }

    pub fn just_released(&self, player: PlayerSlot, action: InputAction) -> bool {
        self.players[player.index()].buttons[action.index()].just_released
    }

    pub fn movement(&self, player: PlayerSlot) -> Vec2 {
        let x = self.pressed(player, InputAction::Right) as i8
            - self.pressed(player, InputAction::Left) as i8;
        let y = self.pressed(player, InputAction::Up) as i8
            - self.pressed(player, InputAction::Down) as i8;
        Vec2::new(x as f32, y as f32)
    }

    pub fn any_just_pressed(&self, action: InputAction) -> bool {
        PlayerSlot::ALL
            .iter()
            .copied()
            .any(|player| self.just_pressed(player, action))
    }
}

#[derive(Resource, Default)]
struct GamepadAssignments([Option<Entity>; PLAYER_COUNT]);

fn gamepad_action_pressed(gamepad: &Gamepad, action: InputAction, bindings: PlayerBindings) -> bool {
    let binding = bindings.binding(action);
    if gamepad.pressed(binding.gamepad.gamepad_button()) {
        return true;
    }
    match action {
        InputAction::Left => {
            gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0) < -GAMEPAD_AXIS_THRESHOLD
        }
        InputAction::Right => {
            gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0) > GAMEPAD_AXIS_THRESHOLD
        }
        InputAction::Up => {
            gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0) > GAMEPAD_AXIS_THRESHOLD
        }
        InputAction::Down => {
            gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0) < -GAMEPAD_AXIS_THRESHOLD
        }
        _ => false,
    }
}

fn refresh_gamepad_assignments(
    assignments: &mut GamepadAssignments,
    gamepads: &Query<(Entity, &Gamepad)>,
) {
    for assigned in &mut assignments.0 {
        if assigned.is_some_and(|entity| gamepads.get(entity).is_err()) {
            *assigned = None;
        }
    }

    let mut connected: Vec<Entity> = gamepads.iter().map(|(entity, _)| entity).collect();
    connected.sort_by_key(|entity| entity.to_bits());
    for entity in connected {
        if assignments.0.contains(&Some(entity)) {
            continue;
        }
        if let Some(slot) = assignments.0.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(entity);
        }
    }
}

fn update_action_state(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    bindings: Res<InputBindings>,
    mut assignments: ResMut<GamepadAssignments>,
    mut state: ResMut<ActionState>,
) {
    refresh_gamepad_assignments(&mut assignments, &gamepads);
    for player in PlayerSlot::ALL {
        let player_index = player.index();
        let player_bindings = bindings.0[player_index];
        let gamepad = assignments.0[player_index].and_then(|entity| gamepads.get(entity).ok());
        for action in InputAction::ALL {
            let keyboard = player_bindings.binding(action).keyboard.key_code();
            let next = keys.pressed(keyboard)
                || gamepad.is_some_and(|(_, pad)| {
                    gamepad_action_pressed(pad, action, player_bindings)
                });
            let button = &mut state.players[player_index].buttons[action.index()];
            button.just_pressed = next && !button.pressed;
            button.just_released = !next && button.pressed;
            button.pressed = next;
        }
    }
}

pub struct ActionInputPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionInputSet;

impl Plugin for ActionInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputBindings>()
            .init_resource::<ActionState>()
            .init_resource::<GamepadAssignments>()
            .add_systems(
                PreUpdate,
                update_action_state
                    .in_set(ActionInputSet)
                    .after(InputSystems),
            );
    }
}

pub struct PlayerInput {
    pub move_dir: Vec2,
    pub fire: bool,
    pub jump: bool,
}

/// 新输入层的兼容数据形状，供旧玩法系统逐个迁移。
pub fn action_input_for(actions: &ActionState, player: usize) -> PlayerInput {
    let player = PlayerSlot::from_index(player);
    PlayerInput {
        move_dir: actions.movement(player),
        fire: actions.pressed(player, InputAction::Primary),
        jump: actions.just_pressed(player, InputAction::Secondary),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_combines_opposite_directions() {
        let mut state = ActionState::default();
        let buttons = &mut state.players[0].buttons;
        buttons[InputAction::Left.index()].pressed = true;
        buttons[InputAction::Right.index()].pressed = true;
        buttons[InputAction::Up.index()].pressed = true;
        assert_eq!(state.movement(PlayerSlot::One), Vec2::Y);
    }

    #[test]
    fn compatibility_view_uses_primary_and_secondary() {
        let mut state = ActionState::default();
        state.players[0].buttons[InputAction::Primary.index()].pressed = true;
        state.players[0].buttons[InputAction::Secondary.index()].just_pressed = true;
        let input = action_input_for(&state, 0);
        assert!(input.fire);
        assert!(input.jump);
    }
}
