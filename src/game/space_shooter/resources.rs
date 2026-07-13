use bevy::prelude::*;

use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};

use super::components::EnemyKind;

#[derive(Resource, Default)]
pub struct SpaceControls {
    movement: Vec2,
    firing: bool,
    roll_buffered: bool,
}

impl SpaceControls {
    pub fn sample(&mut self, actions: &ActionState) {
        self.movement = actions.movement(PlayerSlot::One);
        self.firing = actions.pressed(PlayerSlot::One, InputAction::Primary);
        self.roll_buffered |= actions.just_pressed(PlayerSlot::One, InputAction::Secondary);
    }

    pub fn movement(&self) -> Vec2 {
        self.movement
    }

    pub fn firing(&self) -> bool {
        self.firing
    }

    pub fn take_roll(&mut self) -> bool {
        std::mem::take(&mut self.roll_buffered)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Resource)]
pub struct SpaceState {
    pub power: u8,
    pub rolls: u8,
    pub wave_idx: usize,
    pub wave_clock: f32,
    pub pending: Vec<PendingSpawn>,
    pub wave_in_progress: bool,
    pub between_wave_clock: f32,
    pub boss_spawned: bool,
    pub boss_defeated: bool,
    pub boss_hp_max: i32,
    pub message: String,
    pub message_clock: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PendingSpawn {
    pub delay: f32,
    pub kind: EnemyKind,
    pub pos: Vec2,
    pub drops_power: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_buffer_is_consumed_once() {
        let mut controls = SpaceControls {
            roll_buffered: true,
            ..Default::default()
        };
        assert!(controls.take_roll());
        assert!(!controls.take_roll());
    }
}
