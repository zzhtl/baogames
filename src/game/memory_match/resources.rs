use bevy::prelude::*;

use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};

#[derive(Resource, Default)]
pub struct MemoryControls {
    move_buffered: Option<(i32, i32)>,
    flip_buffered: bool,
}

impl MemoryControls {
    pub fn sample(&mut self, actions: &ActionState) {
        for (action, direction) in [
            (InputAction::Left, (-1, 0)),
            (InputAction::Right, (1, 0)),
            (InputAction::Up, (0, -1)),
            (InputAction::Down, (0, 1)),
        ] {
            if actions.just_pressed(PlayerSlot::One, action) {
                self.move_buffered = Some(direction);
            }
        }
        self.flip_buffered |= actions.just_pressed(PlayerSlot::One, InputAction::Primary);
    }

    pub fn take_move(&mut self) -> Option<(i32, i32)> {
        self.move_buffered.take()
    }

    pub fn take_flip(&mut self) -> bool {
        std::mem::take(&mut self.flip_buffered)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Resource)]
pub struct MemoryStage {
    pub cols: u32,
    pub rows: u32,
    pub cell_size: f32,
    /// 棋盘左上角第一张卡（col=0, row=0）的中心坐标。
    pub origin: Vec2,
    pub pairs_total: u32,
    pub pairs_done: u32,
    pub flips: u32,
    pub time_left: f32,
    pub cursor_col: i32,
    pub cursor_row: i32,
    pub first_pick: Option<Entity>,
    pub second_pick: Option<Entity>,
    /// 两张牌翻开后停留对比的剩余时间。
    pub resolve_timer: f32,
    pub message: String,
    pub message_clock: f32,
    pub preview_timer: f32,
    pub combo_streak: u8,
    pub best_combo: u8,
}

impl MemoryStage {
    pub fn cell_center(&self, col: i32, row: i32) -> Vec2 {
        Vec2::new(
            self.origin.x + col as f32 * self.cell_size,
            self.origin.y - row as f32 * self.cell_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryControls;

    #[test]
    fn flip_buffer_is_consumed_once() {
        let mut controls = MemoryControls {
            flip_buffered: true,
            ..Default::default()
        };
        assert!(controls.take_flip());
        assert!(!controls.take_flip());
    }
}
