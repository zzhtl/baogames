//! 连线判定：拐弯不超过两次的通路。
//!
//! 棋盘外面有一圈"虚拟空格"，所以贴边的图案可以从外面绕过去 —— 这是连连看的
//! 标准规则，少了它角落的图案经常配不掉。坐标范围因此是 `-1..=cols`。

/// `tiles[row * cols + col]`，`None` 表示空格。
pub type Board = [Option<u8>];

fn free(tiles: &Board, cols: i32, rows: i32, x: i32, y: i32) -> bool {
    if x < -1 || y < -1 || x > cols || y > rows {
        return false;
    }
    if x < 0 || y < 0 || x >= cols || y >= rows {
        return true; // 棋盘外那一圈永远可通行
    }
    tiles[(y * cols + x) as usize].is_none()
}

/// `a` 与 `b` 同行或同列，且**中间**的格子全是空的（不含两个端点）。
fn segment_clear(tiles: &Board, cols: i32, rows: i32, a: (i32, i32), b: (i32, i32)) -> bool {
    if a.0 != b.0 && a.1 != b.1 {
        return false;
    }
    let step = (
        (b.0 - a.0).signum(),
        (b.1 - a.1).signum(),
    );
    let mut p = (a.0 + step.0, a.1 + step.1);
    while p != b {
        if !free(tiles, cols, rows, p.0, p.1) {
            return false;
        }
        p = (p.0 + step.0, p.1 + step.1);
    }
    true
}

fn direct(tiles: &Board, cols: i32, rows: i32, a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 == b.0 || a.1 == b.1) && segment_clear(tiles, cols, rows, a, b)
}

fn one_turn(tiles: &Board, cols: i32, rows: i32, a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    for corner in [(a.0, b.1), (b.0, a.1)] {
        if corner == a || corner == b {
            continue;
        }
        if free(tiles, cols, rows, corner.0, corner.1)
            && segment_clear(tiles, cols, rows, a, corner)
            && segment_clear(tiles, cols, rows, corner, b)
        {
            return Some(corner);
        }
    }
    None
}

/// 找一条从 `a` 到 `b`、拐弯不超过两次的通路。
///
/// 返回的是**折点序列**（含首尾），可以直接连成折线画出来；连不上时返回 `None`。
pub fn find_path(
    tiles: &Board,
    cols: i32,
    rows: i32,
    a: (i32, i32),
    b: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    if a == b {
        return None;
    }
    if direct(tiles, cols, rows, a, b) {
        return Some(vec![a, b]);
    }
    if let Some(corner) = one_turn(tiles, cols, rows, a, b) {
        return Some(vec![a, corner, b]);
    }
    // 两次拐弯：先从 a 沿一个方向直走到某个空格 c，再用「直连或一次拐弯」接到 b。
    let row_candidates = (-1..=cols).map(|x| (x, a.1));
    let col_candidates = (-1..=rows).map(|y| (a.0, y));
    for c in row_candidates.chain(col_candidates) {
        if c == a || !free(tiles, cols, rows, c.0, c.1) || !segment_clear(tiles, cols, rows, a, c) {
            continue;
        }
        if direct(tiles, cols, rows, c, b) {
            return Some(vec![a, c, b]);
        }
        if let Some(corner) = one_turn(tiles, cols, rows, c, b) {
            return Some(vec![a, c, corner, b]);
        }
    }
    None
}

/// 盘面上是否还存在能消的一对。没有就是死局，要洗牌。
pub fn has_any_move(tiles: &Board, cols: i32, rows: i32) -> bool {
    find_any_move(tiles, cols, rows).is_some()
}

/// 找出任意一对可消的图案，顺带给"卡住了"时的自动洗牌当校验。
pub fn find_any_move(tiles: &Board, cols: i32, rows: i32) -> Option<((i32, i32), (i32, i32))> {
    let cells: Vec<(i32, i32, u8)> = tiles
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| {
            tile.map(|pattern| ((index as i32) % cols, (index as i32) / cols, pattern))
        })
        .collect();
    for (i, &(ax, ay, pa)) in cells.iter().enumerate() {
        for &(bx, by, pb) in &cells[i + 1..] {
            if pa == pb && find_path(tiles, cols, rows, (ax, ay), (bx, by)).is_some() {
                return Some(((ax, ay), (bx, by)));
            }
        }
    }
    None
}

/// 消除后让图案往下掉（落到列底）。
pub fn apply_gravity(tiles: &mut [Option<u8>], cols: i32, rows: i32) {
    for col in 0..cols {
        let mut write = rows - 1;
        for row in (0..rows).rev() {
            let index = (row * cols + col) as usize;
            if let Some(pattern) = tiles[index] {
                tiles[index] = None;
                tiles[(write * cols + col) as usize] = Some(pattern);
                write -= 1;
            }
        }
    }
}

/// 把还剩下的图案重新打散，位置不变、内容重排。
pub fn reshuffle<R: rand::Rng + ?Sized>(tiles: &mut [Option<u8>], rng: &mut R) {
    use rand::seq::SliceRandom;
    let mut remaining: Vec<u8> = tiles.iter().filter_map(|tile| *tile).collect();
    remaining.shuffle(rng);
    let mut next = remaining.into_iter();
    for tile in tiles.iter_mut() {
        if tile.is_some() {
            *tile = next.next();
        }
    }
}
