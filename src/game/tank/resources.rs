use bevy::prelude::*;

use crate::common::settings::{PLAYER_COUNT, PlayerSlot};

#[derive(Clone, Copy, Default)]
struct TankPlayerControls {
    movement: Vec2,
    fire_held: bool,
}

#[derive(Resource)]
pub struct TankControls {
    players: [TankPlayerControls; PLAYER_COUNT],
}

impl Default for TankControls {
    fn default() -> Self {
        Self {
            players: [TankPlayerControls::default(); PLAYER_COUNT],
        }
    }
}

impl TankControls {
    pub fn movement(&self, player_id: usize) -> Vec2 {
        self.players[PlayerSlot::from_index(player_id).index()].movement
    }

    pub fn fire_held(&self, player_id: usize) -> bool {
        self.players[PlayerSlot::from_index(player_id).index()].fire_held
    }

    pub fn set(&mut self, player: PlayerSlot, movement: Vec2, fire_held: bool) {
        self.players[player.index()] = TankPlayerControls {
            movement,
            fire_held,
        };
    }

    pub fn clear(&mut self) {
        self.players = [TankPlayerControls::default(); PLAYER_COUNT];
    }
}

#[derive(Resource)]
pub struct TankStage {
    pub remaining_to_spawn: u8,
    pub spawn_timer: f32,
    pub spawn_idx: usize,
    #[allow(dead_code)]
    pub stage_num: u8,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub base_alive: bool,
    pub kills: u8,
    pub two_player: bool,
    pub mode_selected: bool,
    pub freeze_timer: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controls_keep_player_slots_independent() {
        let mut controls = TankControls::default();
        controls.set(PlayerSlot::One, Vec2::X, true);
        controls.set(PlayerSlot::Two, Vec2::Y, false);
        assert_eq!(controls.movement(0), Vec2::X);
        assert_eq!(controls.movement(1), Vec2::Y);
        assert!(controls.fire_held(0));
        assert!(!controls.fire_held(1));
    }

    #[test]
    fn clear_releases_both_players() {
        let mut controls = TankControls::default();
        controls.set(PlayerSlot::One, Vec2::X, true);
        controls.clear();
        assert_eq!(controls.movement(0), Vec2::ZERO);
        assert!(!controls.fire_held(0));
    }
}
