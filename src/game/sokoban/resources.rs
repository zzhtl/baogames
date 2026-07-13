use bevy::prelude::*;

use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};

#[derive(Resource, Default)]
pub struct SokobanControls {
    held_dir: Option<(i32, i32)>,
    buffered_dir: Option<(i32, i32)>,
    last_dir: Option<(i32, i32)>,
    undo_buffered: bool,
    reset_buffered: bool,
}

impl SokobanControls {
    pub fn sample(&mut self, actions: &ActionState) {
        for (action, direction) in [
            (InputAction::Left, (-1, 0)),
            (InputAction::Right, (1, 0)),
            (InputAction::Up, (0, -1)),
            (InputAction::Down, (0, 1)),
        ] {
            if actions.just_pressed(PlayerSlot::One, action) {
                self.buffered_dir = Some(direction);
                self.last_dir = Some(direction);
            }
        }
        let movement = actions.movement(PlayerSlot::One);
        self.held_dir = held_direction(movement, self.last_dir);
        self.undo_buffered |= actions.just_pressed(PlayerSlot::One, InputAction::Secondary);
        self.reset_buffered |= actions.just_pressed(PlayerSlot::One, InputAction::Reset);
    }

    pub fn take_step(&mut self) -> Option<(i32, i32)> {
        self.buffered_dir.take()
    }

    pub fn held_dir(&self) -> Option<(i32, i32)> {
        self.held_dir
    }

    pub fn take_undo(&mut self) -> bool {
        std::mem::take(&mut self.undo_buffered)
    }

    pub fn take_reset(&mut self) -> bool {
        std::mem::take(&mut self.reset_buffered)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

fn held_direction(movement: Vec2, last: Option<(i32, i32)>) -> Option<(i32, i32)> {
    if movement == Vec2::ZERO {
        return None;
    }
    if movement.x != 0.0
        && movement.y != 0.0
        && let Some(last) = last
        && ((last.0 != 0 && movement.x.signum() == last.0 as f32)
            || (last.1 != 0 && movement.y.signum() == -last.1 as f32))
    {
        return Some(last);
    }
    if movement.x != 0.0 {
        Some((movement.x.signum() as i32, 0))
    } else {
        Some((0, -movement.y.signum() as i32))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Goal,
}

#[derive(Clone)]
pub struct MoveSnapshot {
    pub boxes: Vec<(i32, i32)>,
    pub player: (i32, i32),
    pub moves: u32,
    pub pushes: u32,
}

#[derive(Resource)]
pub struct SokobanStage {
    pub cols: u32,
    pub rows: u32,
    pub cell_size: f32,
    /// 网格 (0,0) 对应的世界中心坐标。
    pub origin: Vec2,
    /// 行优先布局：tiles[row * cols + col]。
    pub tiles: Vec<Tile>,
    pub boxes: Vec<(i32, i32)>,
    pub player: (i32, i32),
    pub moves: u32,
    pub pushes: u32,
    pub time_left: f32,
    pub move_cd: f32,
    pub initial_boxes: Vec<(i32, i32)>,
    pub initial_player: (i32, i32),
    pub initial_time: f32,
    pub message: String,
    pub message_clock: f32,
    pub history: Vec<MoveSnapshot>,
}

impl SokobanStage {
    pub fn tile_at(&self, col: i32, row: i32) -> Tile {
        if col < 0 || row < 0 || (col as u32) >= self.cols || (row as u32) >= self.rows {
            return Tile::Wall;
        }
        self.tiles[(row as u32 * self.cols + col as u32) as usize]
    }

    pub fn box_at(&self, col: i32, row: i32) -> Option<usize> {
        self.boxes.iter().position(|&(c, r)| c == col && r == row)
    }

    pub fn cell_center(&self, col: i32, row: i32) -> Vec2 {
        Vec2::new(
            self.origin.x + col as f32 * self.cell_size,
            self.origin.y - row as f32 * self.cell_size,
        )
    }

    pub fn all_boxes_done(&self) -> bool {
        !self.boxes.is_empty()
            && self
                .boxes
                .iter()
                .all(|&(c, r)| self.tile_at(c, r) == Tile::Goal)
    }
}

#[cfg(test)]
mod tests {
    use super::held_direction;
    use bevy::prelude::Vec2;

    #[test]
    fn held_diagonal_keeps_last_pressed_axis() {
        assert_eq!(held_direction(Vec2::ONE, Some((0, -1))), Some((0, -1)));
        assert_eq!(held_direction(Vec2::ONE, Some((1, 0))), Some((1, 0)));
        assert_eq!(held_direction(Vec2::X, None), Some((1, 0)));
    }
}
