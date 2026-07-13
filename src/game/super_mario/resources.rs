use bevy::prelude::*;

use crate::common::settings::GameplayProfile;

#[derive(Resource)]
pub struct MarioStage {
    pub time_left: f32,
    pub coins: u32,
    pub level: u8,
    pub finish_timer: f32,
    pub player_spawn: Vec2,
    pub next_checkpoint_x: f32,
}

#[derive(Resource, Default)]
pub struct MarioControls {
    pub horizontal: f32,
    pub run_held: bool,
    pub jump_held: bool,
    pub jump_buffer_ticks: u8,
    pub fire_buffer_ticks: u8,
}

impl MarioControls {
    pub fn latch_jump(&mut self, profile: GameplayProfile) {
        self.jump_buffer_ticks = self
            .jump_buffer_ticks
            .max(profile.jump_buffer_ticks());
    }

    pub fn clear_for_inactive_session(&mut self) {
        self.horizontal = 0.0;
        self.run_held = false;
        self.jump_held = false;
        self.jump_buffer_ticks = 0;
        self.fire_buffer_ticks = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assist_profile_latches_six_fixed_ticks() {
        let mut controls = MarioControls::default();
        controls.latch_jump(GameplayProfile::Assist);
        assert_eq!(controls.jump_buffer_ticks, 6);
    }

    #[test]
    fn classic_profile_only_latches_current_fixed_tick() {
        let mut controls = MarioControls::default();
        controls.latch_jump(GameplayProfile::Classic);
        assert_eq!(controls.jump_buffer_ticks, 1);
    }
}
