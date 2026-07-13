use bevy::prelude::*;

use crate::common::settings::GameplayProfile;

use super::components::{EnemyKind, Weapon};

#[derive(Resource, Default)]
pub struct ContraControls {
    pub movement: Vec2,
    pub fire_held: bool,
    pub jump_buffer_ticks: u8,
}

impl ContraControls {
    pub fn latch_jump(&mut self, profile: GameplayProfile) {
        self.jump_buffer_ticks = self
            .jump_buffer_ticks
            .max(profile.jump_buffer_ticks());
    }

    pub fn consume_jump(&mut self, ready: bool) -> bool {
        if ready && self.jump_buffer_ticks > 0 {
            self.jump_buffer_ticks = 0;
            true
        } else {
            self.jump_buffer_ticks = self.jump_buffer_ticks.saturating_sub(1);
            false
        }
    }

    pub fn clear(&mut self) {
        self.movement = Vec2::ZERO;
        self.fire_held = false;
        self.jump_buffer_ticks = 0;
    }
}

pub struct EnemySpawnMark {
    pub trigger_x: f32,
    pub pos: Vec2,
    pub kind: EnemyKind,
    pub facing: f32,
}

pub struct FalconMark {
    pub trigger_x: f32,
    pub start: Vec2,
    pub vx: f32,
    pub weapon: Weapon,
}

#[derive(Resource)]
pub struct ContraStage {
    pub level: u8,
    pub world_w: f32,
    pub player_spawn: Vec2,
    pub spawn_marks: Vec<EnemySpawnMark>,
    pub spawn_idx: usize,
    pub falcon_marks: Vec<FalconMark>,
    pub falcon_idx: usize,
    pub boss_x: f32,
    pub boss_hp: i32,
    pub boss_spawned: bool,
    pub boss_dead: bool,
    pub top_score: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assist_jump_buffer_survives_until_a_fixed_tick_can_use_it() {
        let mut controls = ContraControls::default();
        controls.latch_jump(GameplayProfile::Assist);
        assert_eq!(controls.jump_buffer_ticks, 6);
        assert!(!controls.consume_jump(false));
        assert_eq!(controls.jump_buffer_ticks, 5);
        assert!(controls.consume_jump(true));
        assert_eq!(controls.jump_buffer_ticks, 0);
    }

    #[test]
    fn classic_jump_buffer_only_lasts_one_fixed_tick() {
        let mut controls = ContraControls::default();
        controls.latch_jump(GameplayProfile::Classic);
        assert!(!controls.consume_jump(false));
        assert_eq!(controls.jump_buffer_ticks, 0);
    }
}
