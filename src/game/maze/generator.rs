//! 迷宫生成：随机深度优先挖通道。
//!
//! 坐标有两套，别混：
//! - **格坐标**（cell）：迷宫的房间，`cells_w × cells_h`。
//! - **瓦片坐标**（tile）：真正画出来的网格，`(2·cells_w+1) × (2·cells_h+1)`，
//!   奇数行列是房间，偶数行列是墙或被挖开的通道。

use rand::Rng;
use rand::seq::SliceRandom;

/// 瓦片网格的边长。
pub const fn tile_dims(cells_w: usize, cells_h: usize) -> (usize, usize) {
    (cells_w * 2 + 1, cells_h * 2 + 1)
}

/// 房间 (cx, cy) 对应的瓦片坐标。
pub const fn cell_to_tile(cx: usize, cy: usize) -> (usize, usize) {
    (cx * 2 + 1, cy * 2 + 1)
}

/// 生成一张完美迷宫，返回 `walls[row * w + col]`（`true` = 墙）。
pub fn generate<R: Rng + ?Sized>(cells_w: usize, cells_h: usize, rng: &mut R) -> Vec<bool> {
    let cells_w = cells_w.max(1);
    let cells_h = cells_h.max(1);
    let (w, h) = tile_dims(cells_w, cells_h);
    let mut walls = vec![true; w * h];
    let mut visited = vec![false; cells_w * cells_h];
    let mut stack = vec![(0usize, 0usize)];
    visited[0] = true;
    let (sx, sy) = cell_to_tile(0, 0);
    walls[sy * w + sx] = false;

    while let Some(&(cx, cy)) = stack.last() {
        let mut candidates: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        candidates.shuffle(rng);
        let next = candidates.into_iter().find_map(|(dx, dy)| {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx < 0 || ny < 0 || nx >= cells_w as isize || ny >= cells_h as isize {
                return None;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            (!visited[ny * cells_w + nx]).then_some((nx, ny))
        });
        match next {
            Some((nx, ny)) => {
                visited[ny * cells_w + nx] = true;
                let (tx, ty) = cell_to_tile(nx, ny);
                let (fx, fy) = cell_to_tile(cx, cy);
                // 打通两个房间之间那面墙
                walls[((fy + ty) / 2) * w + (fx + tx) / 2] = false;
                walls[ty * w + tx] = false;
                stack.push((nx, ny));
            }
            None => {
                stack.pop();
            }
        }
    }
    walls
}

/// 从 `start` 出发到每个可通行瓦片的步数；走不到的是 `None`。
pub fn distances(walls: &[bool], w: usize, h: usize, start: (usize, usize)) -> Vec<Option<u32>> {
    let mut dist = vec![None; w * h];
    if walls[start.1 * w + start.0] {
        return dist;
    }
    let mut queue = std::collections::VecDeque::from([start]);
    dist[start.1 * w + start.0] = Some(0);
    while let Some((x, y)) = queue.pop_front() {
        let step = dist[y * w + x].unwrap_or(0);
        for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if walls[ny * w + nx] || dist[ny * w + nx].is_some() {
                continue;
            }
            dist[ny * w + nx] = Some(step + 1);
            queue.push_back((nx, ny));
        }
    }
    dist
}

/// 挑一个放钥匙的房间：离起点和出口都足够远，别让人顺路就捡到。
pub fn pick_key_tile(
    walls: &[bool],
    w: usize,
    h: usize,
    start: (usize, usize),
    exit: (usize, usize),
) -> Option<(usize, usize)> {
    let from_start = distances(walls, w, h, start);
    let from_exit = distances(walls, w, h, exit);
    (0..w * h)
        .filter(|&i| !walls[i] && (i % w) % 2 == 1 && (i / w) % 2 == 1)
        .filter(|&i| (i % w, i / w) != start && (i % w, i / w) != exit)
        .max_by_key(|&i| {
            let a = from_start[i].unwrap_or(0);
            let b = from_exit[i].unwrap_or(0);
            // 取两段距离里较短的那段最大 —— 也就是离两端都尽量远
            a.min(b)
        })
        .map(|i| (i % w, i / w))
}
