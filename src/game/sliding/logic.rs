//! 华容道的盘面逻辑：全是纯函数，可解性与步数下界都在这里单测。

use rand::Rng;

/// 已完成的盘面：1..size²−1 依次排列，`0`（空格）在右下角。
pub fn solved_board(size: usize) -> Vec<u8> {
    let cells = size * size;
    (1..cells as u8).chain(std::iter::once(0)).collect()
}

pub fn is_solved(board: &[u8]) -> bool {
    let size = (board.len() as f32).sqrt() as usize;
    board == solved_board(size).as_slice()
}

pub fn blank_index(board: &[u8]) -> usize {
    board.iter().position(|&value| value == 0).unwrap_or(0)
}

/// 玩家按下 `dir` 时能滑动的那块的下标。
///
/// `dir` 是**块的移动方向**（网格坐标，y 向下）：按「右」= 空格左边那块往右滑，
/// 所以要找的是空格左边一格。
pub fn tile_to_slide(board: &[u8], size: usize, dir: (i32, i32)) -> Option<usize> {
    let blank = blank_index(board);
    let (bc, br) = ((blank % size) as i32, (blank / size) as i32);
    let (tc, tr) = (bc - dir.0, br - dir.1);
    if tc < 0 || tr < 0 || tc >= size as i32 || tr >= size as i32 {
        return None;
    }
    Some(tr as usize * size + tc as usize)
}

/// 按 `dir` 滑一步。返回被移动的块的值；不能滑时返回 `None`。
pub fn slide(board: &mut [u8], size: usize, dir: (i32, i32)) -> Option<u8> {
    let tile = tile_to_slide(board, size, dir)?;
    let blank = blank_index(board);
    let value = board[tile];
    board.swap(tile, blank);
    Some(value)
}

/// 从已完成状态做 `steps` 次合法随机走。
///
/// 不走「上一步的反方向」，避免刚打乱又被抵消；这样也保证结果一定可解。
pub fn shuffle<R: Rng + ?Sized>(size: usize, steps: usize, rng: &mut R) -> Vec<u8> {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut board = solved_board(size);
    let mut last: Option<(i32, i32)> = None;
    let mut done = 0;
    // 每步最多试几次方向，避免在角落里空转。
    let mut guard = steps * 8 + 32;
    while done < steps && guard > 0 {
        guard -= 1;
        let dir = DIRS[rng.gen_range(0..DIRS.len())];
        if last == Some((-dir.0, -dir.1)) {
            continue;
        }
        if slide(&mut board, size, dir).is_some() {
            last = Some(dir);
            done += 1;
        }
    }
    // 极小概率恰好走回原位，再补一步保证开局不是已完成状态。
    if is_solved(&board) {
        for dir in DIRS {
            if slide(&mut board, size, dir).is_some() {
                break;
            }
        }
    }
    board
}

/// 曼哈顿距离下界：每块回到自己的位置至少要走这么多步。
///
/// 用来判断玩家"绕了多少弯"，不是精确最优解。
pub fn manhattan_lower_bound(board: &[u8], size: usize) -> u32 {
    board
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value != 0)
        .map(|(index, &value)| {
            let goal = (value - 1) as usize;
            let (c, r) = (index % size, index / size);
            let (gc, gr) = (goal % size, goal / size);
            (c.abs_diff(gc) + r.abs_diff(gr)) as u32
        })
        .sum()
}
