use bevy::prelude::*;

use crate::game::puzzle::grid::GridLayout;

#[derive(Resource)]
pub struct MazeStage {
    pub layout: GridLayout,
    pub cols: i32,
    pub rows: i32,
    /// 行优先，`true` = 墙。
    pub walls: Vec<bool>,
    /// 走过的瓦片，用来画脚印和算回头步。
    pub visited: Vec<bool>,
    pub player: (i32, i32),
    pub start: (i32, i32),
    pub exit: (i32, i32),
    pub key: Option<(i32, i32)>,
    pub has_key: bool,
    pub fog_radius: Option<i32>,
    pub steps: u32,
    pub backtracks: u32,
    pub move_cd: f32,
    pub time_left: f32,
    pub message: String,
    pub message_clock: f32,
}

impl MazeStage {
    pub fn is_wall(&self, col: i32, row: i32) -> bool {
        if col < 0 || row < 0 || col >= self.cols || row >= self.rows {
            return true;
        }
        self.walls[(row * self.cols + col) as usize]
    }

    pub fn index(&self, col: i32, row: i32) -> Option<usize> {
        (col >= 0 && row >= 0 && col < self.cols && row < self.rows)
            .then(|| (row * self.cols + col) as usize)
    }

    /// 出口是否已经打开（不需要钥匙，或者钥匙已到手）。
    pub fn exit_open(&self) -> bool {
        self.key.is_none() || self.has_key
    }

    pub fn is_cleared(&self) -> bool {
        self.player == self.exit && self.exit_open()
    }

    /// 雾视野下这块瓦片是否可见。没开雾时恒为真。
    pub fn is_visible(&self, col: i32, row: i32) -> bool {
        let Some(radius) = self.fog_radius else {
            return true;
        };
        (col - self.player.0).abs().max((row - self.player.1).abs()) <= radius
    }
}
