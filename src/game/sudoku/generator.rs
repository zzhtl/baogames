//! 数独题面生成：回溯求解 + 挖空校验唯一解。
//!
//! 全部是纯函数，方便单测「解合法」「挖空后仍唯一」这两条最容易写错的性质。

use rand::Rng;
use rand::seq::SliceRandom;

/// 盘面规格。`size` 是边长，宫是 `box_w × box_h`。
///
/// 6×6 的宫是 3 宽 2 高（横 2 宫、竖 3 宫），不是正方形 —— 这是 6 阶数独的标准分宫。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SudokuSpec {
    pub size: usize,
    pub box_w: usize,
    pub box_h: usize,
}

impl SudokuSpec {
    pub fn for_size(size: usize) -> Self {
        match size {
            4 => Self { size: 4, box_w: 2, box_h: 2 },
            6 => Self { size: 6, box_w: 3, box_h: 2 },
            _ => Self { size: 9, box_w: 3, box_h: 3 },
        }
    }

    pub const fn cells(self) -> usize {
        self.size * self.size
    }

    pub const fn index(self, col: usize, row: usize) -> usize {
        row * self.size + col
    }
}

/// 在 `index` 处填 `value`（1 起）是否不与已有数字冲突。
pub fn is_legal(board: &[u8], spec: SudokuSpec, index: usize, value: u8) -> bool {
    if value == 0 {
        return true;
    }
    let size = spec.size;
    let (col, row) = (index % size, index / size);
    for other in 0..size {
        if other != col && board[spec.index(other, row)] == value {
            return false;
        }
        if other != row && board[spec.index(col, other)] == value {
            return false;
        }
    }
    let (bc, br) = (col / spec.box_w * spec.box_w, row / spec.box_h * spec.box_h);
    for r in br..br + spec.box_h {
        for c in bc..bc + spec.box_w {
            let i = spec.index(c, r);
            if i != index && board[i] == value {
                return false;
            }
        }
    }
    true
}

/// 盘面上所有与同行 / 同列 / 同宫重复的格子。用来实时标红。
pub fn conflicts(board: &[u8], spec: SudokuSpec) -> Vec<bool> {
    (0..spec.cells())
        .map(|index| board[index] != 0 && !is_legal(board, spec, index, board[index]))
        .collect()
}

pub fn is_solved(board: &[u8], spec: SudokuSpec) -> bool {
    board.iter().all(|&value| value != 0) && conflicts(board, spec).iter().all(|hit| !hit)
}

/// 生成一张完整解。候选顺序随机，所以每局的题面都不一样。
pub fn full_solution<R: Rng + ?Sized>(spec: SudokuSpec, rng: &mut R) -> Vec<u8> {
    let mut board = vec![0u8; spec.cells()];
    let mut values: Vec<u8> = (1..=spec.size as u8).collect();
    fill(&mut board, spec, 0, &mut values, rng);
    board
}

fn fill<R: Rng + ?Sized>(
    board: &mut [u8],
    spec: SudokuSpec,
    index: usize,
    values: &mut Vec<u8>,
    rng: &mut R,
) -> bool {
    if index >= board.len() {
        return true;
    }
    values.shuffle(rng);
    let candidates = values.clone();
    for value in candidates {
        if is_legal(board, spec, index, value) {
            board[index] = value;
            if fill(board, spec, index + 1, values, rng) {
                return true;
            }
            board[index] = 0;
        }
    }
    false
}

/// 数盘面有多少组解，最多数到 `limit` 就提前收手。
///
/// 挖空时只关心「是不是恰好一组」，数到 2 就够了，别把 9×9 的全解空间跑完。
pub fn count_solutions(board: &[u8], spec: SudokuSpec, limit: usize) -> usize {
    let mut work = board.to_vec();
    let mut found = 0;
    count_from(&mut work, spec, 0, limit, &mut found);
    found
}

fn count_from(board: &mut [u8], spec: SudokuSpec, from: usize, limit: usize, found: &mut usize) {
    if *found >= limit {
        return;
    }
    let Some(index) = (from..board.len()).find(|&i| board[i] == 0) else {
        *found += 1;
        return;
    };
    for value in 1..=spec.size as u8 {
        if is_legal(board, spec, index, value) {
            board[index] = value;
            count_from(board, spec, index + 1, limit, found);
            board[index] = 0;
            if *found >= limit {
                return;
            }
        }
    }
}

/// 从完整解里挖空，只保留仍然唯一可解的挖法。
///
/// 返回 `(题面, 实际挖掉的格数)` —— 实际值可能小于 `holes`：挖到后面每一格都会
/// 破坏唯一性，这时停手比硬挖出一道多解题好。
pub fn carve<R: Rng + ?Sized>(
    solution: &[u8],
    spec: SudokuSpec,
    holes: usize,
    rng: &mut R,
) -> (Vec<u8>, usize) {
    let mut puzzle = solution.to_vec();
    let mut order: Vec<usize> = (0..spec.cells()).collect();
    order.shuffle(rng);
    let mut removed = 0;
    for index in order {
        if removed >= holes {
            break;
        }
        let backup = puzzle[index];
        puzzle[index] = 0;
        if count_solutions(&puzzle, spec, 2) == 1 {
            removed += 1;
        } else {
            puzzle[index] = backup;
        }
    }
    (puzzle, removed)
}
