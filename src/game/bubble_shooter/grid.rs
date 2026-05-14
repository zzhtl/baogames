// 泡泡龙的网格几何 + 同色/浮空算法。全部为纯函数，便于单测。

use bevy::math::Vec2;

use super::constants::*;

pub fn level_idx(level: u8) -> usize {
    (level as usize).clamp(1, 10)
}

pub fn cols_in_row(row: i32) -> i32 {
    if row.rem_euclid(2) == 1 {
        COLS_ODD
    } else {
        COLS_EVEN
    }
}

pub fn cell_valid(col: i32, row: i32) -> bool {
    row >= 0 && (row as usize) < ROWS_MAX && col >= 0 && col < cols_in_row(row)
}

pub fn cell_to_pos(col: i32, row: i32, descend: usize) -> Vec2 {
    let offset = if row.rem_euclid(2) == 1 {
        BUBBLE_R
    } else {
        0.0
    };
    let x = PLAY_LEFT + BUBBLE_R + offset + col as f32 * BUBBLE_D;
    let y = PLAY_TOP - TOP_PAD - BUBBLE_R - (row as f32 + descend as f32) * ROW_H;
    Vec2::new(x, y)
}

pub fn neighbors(col: i32, row: i32) -> [(i32, i32); 6] {
    if row.rem_euclid(2) == 0 {
        [
            (col - 1, row),
            (col + 1, row),
            (col - 1, row - 1),
            (col, row - 1),
            (col - 1, row + 1),
            (col, row + 1),
        ]
    } else {
        [
            (col - 1, row),
            (col + 1, row),
            (col, row - 1),
            (col + 1, row - 1),
            (col, row + 1),
            (col + 1, row + 1),
        ]
    }
}

pub fn aim_dir(aim: f32) -> Vec2 {
    Vec2::new(aim.sin(), aim.cos())
}

pub fn snap_nearest_empty(
    pos: Vec2,
    descend: usize,
    grid: &[Vec<Option<u8>>],
) -> Option<(i32, i32)> {
    let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - pos.y) / ROW_H - descend as f32;
    let row0 = row_f.round() as i32;
    let mut best: Option<((i32, i32), f32)> = None;
    for r in (row0 - 1)..=(row0 + 1) {
        if r < 0 || (r as usize) >= ROWS_MAX {
            continue;
        }
        let offset = if r.rem_euclid(2) == 1 {
            BUBBLE_R
        } else {
            0.0
        };
        let col_f = (pos.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
        let col0 = col_f.round() as i32;
        for c in (col0 - 1)..=(col0 + 1) {
            if !cell_valid(c, r) {
                continue;
            }
            if grid[r as usize][c as usize].is_some() {
                continue;
            }
            // 必须与至少一个邻居或顶部相邻才能放置
            let touches_top = r == 0;
            let touches_neighbor = neighbors(c, r).iter().any(|(nc, nr)| {
                cell_valid(*nc, *nr) && grid[*nr as usize][*nc as usize].is_some()
            });
            if !touches_top && !touches_neighbor {
                continue;
            }
            let center = cell_to_pos(c, r, descend);
            let d2 = (center - pos).length_squared();
            match best {
                None => best = Some(((c, r), d2)),
                Some((_, bd)) if d2 < bd => best = Some(((c, r), d2)),
                _ => {}
            }
        }
    }
    best.map(|(cell, _)| cell)
}

pub fn flood_same_color(
    start: (i32, i32),
    color: u8,
    grid: &[Vec<Option<u8>>],
) -> Vec<(i32, i32)> {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut stack = vec![start];
    while let Some((c, r)) = stack.pop() {
        if !cell_valid(c, r) {
            continue;
        }
        if visited.contains(&(c, r)) {
            continue;
        }
        if grid[r as usize][c as usize] != Some(color) {
            continue;
        }
        visited.push((c, r));
        for (nc, nr) in neighbors(c, r) {
            stack.push((nc, nr));
        }
    }
    visited
}

pub fn floating_cells(grid: &[Vec<Option<u8>>]) -> Vec<(i32, i32)> {
    let mut reached = vec![vec![false; COLS_EVEN as usize]; ROWS_MAX];
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for c in 0..cols_in_row(0) {
        if grid[0][c as usize].is_some() {
            stack.push((c, 0));
        }
    }
    while let Some((c, r)) = stack.pop() {
        if !cell_valid(c, r) {
            continue;
        }
        if reached[r as usize][c as usize] {
            continue;
        }
        if grid[r as usize][c as usize].is_none() {
            continue;
        }
        reached[r as usize][c as usize] = true;
        for (nc, nr) in neighbors(c, r) {
            stack.push((nc, nr));
        }
    }
    let mut result = Vec::new();
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if grid[r][c as usize].is_some() && !reached[r][c as usize] {
                result.push((c, r as i32));
            }
        }
    }
    result
}

pub fn available_colors(grid: &[Vec<Option<u8>>]) -> Vec<u8> {
    let mut set = [false; 6];
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if let Some(col) = grid[r][c as usize] {
                if (col as usize) < 6 {
                    set[col as usize] = true;
                }
            }
        }
    }
    (0u8..6u8).filter(|i| set[*i as usize]).collect()
}

pub fn grid_is_empty(grid: &[Vec<Option<u8>>]) -> bool {
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if grid[r][c as usize].is_some() {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_grid() -> Vec<Vec<Option<u8>>> {
        vec![vec![None; COLS_EVEN as usize]; ROWS_MAX]
    }

    #[test]
    fn cols_alternate_by_parity() {
        assert_eq!(cols_in_row(0), COLS_EVEN);
        assert_eq!(cols_in_row(1), COLS_ODD);
        assert_eq!(cols_in_row(2), COLS_EVEN);
    }

    #[test]
    fn cell_valid_respects_bounds() {
        assert!(cell_valid(0, 0));
        assert!(!cell_valid(-1, 0));
        assert!(!cell_valid(0, -1));
        assert!(!cell_valid(0, ROWS_MAX as i32));
        // 奇数行宽度更窄，最右那一列在偶数行有效、奇数行越界
        assert!(cell_valid(COLS_EVEN - 1, 0));
        assert!(!cell_valid(COLS_EVEN - 1, 1));
    }

    #[test]
    fn neighbors_returns_six() {
        // 不关心具体内容，只确认数量
        assert_eq!(neighbors(3, 0).len(), 6);
        assert_eq!(neighbors(3, 1).len(), 6);
    }

    #[test]
    fn level_idx_clamped() {
        assert_eq!(level_idx(0), 1);
        assert_eq!(level_idx(1), 1);
        assert_eq!(level_idx(10), 10);
        assert_eq!(level_idx(20), 10);
    }

    #[test]
    fn aim_dir_zero_points_up() {
        let v = aim_dir(0.0);
        assert!((v.x).abs() < 1e-5);
        assert!((v.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn flood_collects_connected_same_color() {
        let mut g = empty_grid();
        // 三连同色
        g[0][0] = Some(3);
        g[0][1] = Some(3);
        g[0][2] = Some(3);
        g[0][3] = Some(1); // 不同色
        g[0][4] = Some(3); // 不相连
        let group = flood_same_color((0, 0), 3, &g);
        assert_eq!(group.len(), 3);
        for cell in [(0, 0), (1, 0), (2, 0)] {
            assert!(group.contains(&cell));
        }
    }

    #[test]
    fn flood_stops_at_wrong_color() {
        let mut g = empty_grid();
        g[0][0] = Some(2);
        let group = flood_same_color((0, 0), 3, &g);
        assert!(group.is_empty());
    }

    #[test]
    fn floating_detects_disconnected_clusters() {
        let mut g = empty_grid();
        // 顶部一颗
        g[0][0] = Some(0);
        // 中部独立一团（不通顶）
        g[5][3] = Some(1);
        g[5][4] = Some(1);
        let floats = floating_cells(&g);
        assert_eq!(floats.len(), 2);
        assert!(floats.contains(&(3, 5)));
        assert!(floats.contains(&(4, 5)));
    }

    #[test]
    fn floating_detects_single_isolated_bubble() {
        let mut g = empty_grid();
        // 顶部连通的一串
        g[0][2] = Some(0);
        g[1][2] = Some(0);
        // 孤立在中部的单颗（与顶部不连通）
        g[6][5] = Some(2);
        let floats = floating_cells(&g);
        assert_eq!(floats, vec![(5, 6)]);
    }

    #[test]
    fn floating_detects_single_bubble_after_chain_pop() {
        // 模拟：链条被消除后，剩下的单颗与顶部失联
        let mut g = empty_grid();
        // 链条原先是 (0,0)-(0,1)-(0,2) 同色三连消除后变 None
        // 单颗（异色）原来挂在链条下方
        g[3][0] = Some(3);
        // 假设顶部其他位置仍有连通泡泡（同列右边）
        g[0][5] = Some(1);
        let floats = floating_cells(&g);
        assert!(floats.contains(&(0, 3)));
    }

    #[test]
    fn floating_keeps_connected_to_top() {
        let mut g = empty_grid();
        g[0][2] = Some(0);
        g[1][2] = Some(0);
        g[2][2] = Some(0);
        let floats = floating_cells(&g);
        assert!(floats.is_empty(), "{:?}", floats);
    }

    #[test]
    fn grid_is_empty_detects_empty_and_nonempty() {
        let mut g = empty_grid();
        assert!(grid_is_empty(&g));
        g[3][2] = Some(0);
        assert!(!grid_is_empty(&g));
    }

    #[test]
    fn available_colors_returns_distinct_existing() {
        let mut g = empty_grid();
        g[0][0] = Some(0);
        g[1][1] = Some(2);
        g[2][2] = Some(0);
        let cols = available_colors(&g);
        assert_eq!(cols, vec![0, 2]);
    }

    #[test]
    fn cell_to_pos_roundtrips_through_snap() {
        let mut g = empty_grid();
        // 顶部第一颗
        g[0][3] = Some(0);
        // 距离 (3,0) 较近的空位，且接触邻居 → 应能 snap 到附近某个邻接 cell
        let near = cell_to_pos(4, 0, 0); // 同行邻居（空）
        let snapped = snap_nearest_empty(near, 0, &g).expect("应能 snap");
        // snap 出来的格子应当与有占据邻居相邻
        let occupied_nb = neighbors(snapped.0, snapped.1)
            .iter()
            .any(|(c, r)| *c == 3 && *r == 0);
        assert!(occupied_nb, "snap 结果应靠着 (3,0)");
    }

    #[test]
    fn snap_returns_none_for_isolated_empty() {
        let g = empty_grid();
        // 网格全空，且位置远离顶部（row > 0），应无法落下
        let pos = cell_to_pos(5, 5, 0);
        assert!(snap_nearest_empty(pos, 0, &g).is_none());
    }
}
