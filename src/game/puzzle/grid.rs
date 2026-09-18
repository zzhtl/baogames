//! 棋盘布局与格子游标。

use bevy::prelude::*;

use crate::common::px::WORLD_PER_PX;

use super::controls::PuzzleControls;

/// 棋盘可用区域（世界单位）。
///
/// 宽度上界取 660 而不是推箱子的 880：1 画布像素 = 3 世界单位，880 世界单位
/// 折合 293 画布像素，比 4:3 画布的 240 还宽。推箱子只是因为关卡都足够高、
/// 实际被高度上界挡住才没出屏；益智游戏有 21×15 迷宫、左右双图这类宽扁版面，
/// 必须用真正安全的上界（660 世界单位 = 220 画布像素，左右各留 10 像素边距）。
pub const BOARD_W: f32 = 660.0;
/// 上下各有一条 15 画布像素的 HUD 通栏，中间留给棋盘的高度。
pub const BOARD_H: f32 = 380.0;
/// 棋盘中心相对屏幕中心的下移量，给顶栏让位。
pub const BOARD_CENTER_Y: f32 = -20.0;

/// 长按移动节奏：首次响应立即；之后每过这么多秒走一格。与推箱子手感一致。
pub const MOVE_COOLDOWN: f32 = 0.13;

/// 一个自适应大小、居中摆放的矩形网格。
#[derive(Clone, Copy, Debug)]
pub struct GridLayout {
    pub cols: i32,
    pub rows: i32,
    pub cell_size: f32,
    /// 网格 (0,0) 格中心的世界坐标。
    pub origin: Vec2,
}

impl GridLayout {
    /// 在默认棋盘区域里塞下 cols×rows 的网格。
    pub fn fit(cols: i32, rows: i32) -> Self {
        Self::fit_in(cols, rows, BOARD_W, BOARD_H, BOARD_CENTER_Y)
    }

    /// 自定义可用区域的版本（左右双图之类要把宽度对半分）。
    pub fn fit_in(cols: i32, rows: i32, board_w: f32, board_h: f32, center_y: f32) -> Self {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let raw = (board_w / cols as f32).min(board_h / rows as f32);
        let cell_size = snap_cell(raw);
        let total_w = cell_size * cols as f32;
        let total_h = cell_size * rows as f32;
        Self {
            cols,
            rows,
            cell_size,
            origin: Vec2::new(
                -total_w * 0.5 + cell_size * 0.5,
                total_h * 0.5 - cell_size * 0.5 + center_y,
            ),
        }
    }

    /// 整体平移（左右双图用同一套 fit_in 再各自挪到半屏）。
    pub fn shifted(mut self, delta: Vec2) -> Self {
        self.origin += delta;
        self
    }

    pub fn cell_center(&self, col: i32, row: i32) -> Vec2 {
        Vec2::new(
            self.origin.x + col as f32 * self.cell_size,
            self.origin.y - row as f32 * self.cell_size,
        )
    }

    pub fn contains(&self, col: i32, row: i32) -> bool {
        col >= 0 && row >= 0 && col < self.cols && row < self.rows
    }

    /// 行优先下标；越界返回 None。
    pub fn index(&self, col: i32, row: i32) -> Option<usize> {
        self.contains(col, row)
            .then(|| (row * self.cols + col) as usize)
    }

    pub fn cell_count(&self) -> usize {
        (self.cols * self.rows) as usize
    }

    pub fn total_size(&self) -> Vec2 {
        Vec2::new(
            self.cell_size * self.cols as f32,
            self.cell_size * self.rows as f32,
        )
    }
}

/// 格子游标：首次按下立即走一格，按住则按 [`MOVE_COOLDOWN`] 连发。
#[derive(Clone, Copy, Debug, Default)]
pub struct GridCursor {
    pub col: i32,
    pub row: i32,
    cooldown: f32,
}

impl GridCursor {
    pub fn new(col: i32, row: i32) -> Self {
        Self {
            col,
            row,
            cooldown: 0.0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.cooldown = (self.cooldown - dt).max(0.0);
    }

    /// 消费一次方向输入并尝试移动。返回是否真的换了格子。
    ///
    /// `wrap` 为真时走出边界会绕到对边，否则在边界处停住（并吃掉这次输入）。
    pub fn advance(
        &mut self,
        controls: &mut PuzzleControls,
        cols: i32,
        rows: i32,
        wrap: bool,
    ) -> bool {
        let buffered = controls.take_dir();
        let immediate = buffered.is_some();
        let Some((dx, dy)) = buffered.or_else(|| controls.held_dir()) else {
            return false;
        };
        if !immediate && self.cooldown > 0.0 {
            return false;
        }
        let (col, row) = step_clamped(self.col, self.row, (dx, dy), cols, rows, wrap);
        self.cooldown = MOVE_COOLDOWN;
        if col == self.col && row == self.row {
            return false;
        }
        self.col = col;
        self.row = row;
        true
    }

    pub fn clamp_into(&mut self, cols: i32, rows: i32) {
        self.col = self.col.clamp(0, (cols - 1).max(0));
        self.row = self.row.clamp(0, (rows - 1).max(0));
    }
}

/// 把格宽向下取整到**偶数个画布像素**。
///
/// 只取整到 1 像素还不够：格宽为奇数像素时 `origin = -总宽/2 + 格宽/2` 会落在
/// 半像素上，整张棋盘的描边就会时粗时细。取偶数像素后 `格宽/2` 与 `总宽/2`
/// 都是整像素，格心全部落在画布像素网格上。
fn snap_cell(raw: f32) -> f32 {
    let step = WORLD_PER_PX * 2.0;
    ((raw / step).floor() * step).max(step)
}

/// 纯函数形式的一步移动，便于单测。
pub fn step_clamped(
    col: i32,
    row: i32,
    dir: (i32, i32),
    cols: i32,
    rows: i32,
    wrap: bool,
) -> (i32, i32) {
    if cols <= 0 || rows <= 0 {
        return (col, row);
    }
    let nc = col + dir.0;
    let nr = row + dir.1;
    if wrap {
        (nc.rem_euclid(cols), nr.rem_euclid(rows))
    } else {
        (nc.clamp(0, cols - 1), nr.clamp(0, rows - 1))
    }
}
