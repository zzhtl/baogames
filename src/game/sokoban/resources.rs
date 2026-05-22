use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Goal,
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
