use bevy::prelude::*;

use super::components::EnemyKind;

#[derive(Resource)]
pub struct SpaceState {
    pub power: u8,
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
