use bevy::prelude::*;

use crate::common::settings::{PLAYER_COUNT, PlayerSlot};

#[derive(Clone, Copy, Default)]
struct BMPlayerControls {
    movement: Vec2,
    place_ticks: u8,
    detonate_ticks: u8,
}

#[derive(Resource)]
pub struct BMControls {
    players: [BMPlayerControls; PLAYER_COUNT],
}

impl Default for BMControls {
    fn default() -> Self {
        Self {
            players: [BMPlayerControls::default(); PLAYER_COUNT],
        }
    }
}

impl BMControls {
    pub fn movement(&self, player_id: usize) -> Vec2 {
        self.players[PlayerSlot::from_index(player_id).index()].movement
    }

    pub fn sample(
        &mut self,
        player: PlayerSlot,
        movement: Vec2,
        place_pressed: bool,
        detonate_pressed: bool,
    ) {
        let state = &mut self.players[player.index()];
        state.movement = movement;
        if place_pressed {
            state.place_ticks = state.place_ticks.max(2);
        }
        if detonate_pressed {
            state.detonate_ticks = state.detonate_ticks.max(2);
        }
    }

    pub fn take_place(&mut self, player_id: usize) -> bool {
        let state = &mut self.players[PlayerSlot::from_index(player_id).index()];
        let requested = state.place_ticks > 0;
        state.place_ticks = 0;
        requested
    }

    pub fn take_detonate(&mut self, player_id: usize) -> bool {
        let state = &mut self.players[PlayerSlot::from_index(player_id).index()];
        let requested = state.detonate_ticks > 0;
        state.detonate_ticks = 0;
        requested
    }

    pub fn clear(&mut self) {
        self.players = [BMPlayerControls::default(); PLAYER_COUNT];
    }
}

#[derive(Resource)]
pub struct BMStage {
    pub level: u8,
    pub time_left: f32,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub all_enemies_dead_msg_shown: bool,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placement_is_edge_triggered_and_consumed_once() {
        let mut controls = BMControls::default();
        controls.sample(PlayerSlot::One, Vec2::X, true, false);
        assert!(controls.take_place(0));
        assert!(!controls.take_place(0));
    }

    #[test]
    fn player_buffers_are_independent() {
        let mut controls = BMControls::default();
        controls.sample(PlayerSlot::Two, Vec2::Y, false, true);
        assert_eq!(controls.movement(0), Vec2::ZERO);
        assert_eq!(controls.movement(1), Vec2::Y);
        assert!(!controls.take_detonate(0));
        assert!(controls.take_detonate(1));
    }
}
