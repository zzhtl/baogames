use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::link::{apply_gravity, find_any_move, find_path, has_any_move, reshuffle};
use super::palette;
use super::resources::build_tiles;

/// `.` = 空格，其余字符按出现顺序映射成图案编号。
fn board(rows: &[&str]) -> (Vec<Option<u8>>, i32, i32) {
    let cols = rows[0].chars().count() as i32;
    let mut ids: Vec<char> = Vec::new();
    let tiles = rows
        .iter()
        .flat_map(|line| line.chars())
        .map(|ch| {
            (ch != '.').then(|| {
                let pos = ids.iter().position(|&c| c == ch).unwrap_or_else(|| {
                    ids.push(ch);
                    ids.len() - 1
                });
                pos as u8
            })
        })
        .collect();
    (tiles, cols, rows.len() as i32)
}

#[test]
fn adjacent_and_straight_lines_connect_directly() {
    let (tiles, cols, rows) = board(&["AA.."]);
    assert_eq!(find_path(&tiles, cols, rows, (0, 0), (1, 0)), Some(vec![(0, 0), (1, 0)]));

    // 中间全空，直线可达
    let (tiles, cols, rows) = board(&["A..A"]);
    assert_eq!(find_path(&tiles, cols, rows, (0, 0), (3, 0)), Some(vec![(0, 0), (3, 0)]));

    // 被自己人围死才是真的连不上 —— 单纯挡住直线是不够的，还能从棋盘外绕
    let (tiles, cols, rows) = board(&["BBBBB", "BABAB", "BBBBB"]);
    assert!(find_path(&tiles, cols, rows, (1, 1), (3, 1)).is_none());
}

#[test]
fn one_and_two_turns_are_accepted() {
    // 一次拐弯：(0,0) 往下再往右
    let (tiles, cols, rows) = board(&["A...", "...A"]);
    let path = find_path(&tiles, cols, rows, (0, 0), (3, 1)).expect("一次拐弯应该能连上");
    assert_eq!(path.len(), 3);

    // 两次拐弯：中间隔着一堵墙，要绕一下
    let (tiles, cols, rows) = board(&["A.B.", ".BB.", "..A."]);
    let path = find_path(&tiles, cols, rows, (0, 0), (2, 2)).expect("两次拐弯应该能连上");
    assert!(path.len() <= 4, "折点超过两次拐弯：{path:?}");
}

/// 棋盘外面那一圈必须能走，否则贴边、尤其是对角的图案永远配不掉。
#[test]
fn paths_may_route_around_the_outside_of_the_board() {
    // 同一列上下两个 A，中间被挡死，只能从左边那圈外面绕过去
    let (tiles, cols, rows) = board(&["AB", "BB", "BB", "AB"]);
    let path = find_path(&tiles, cols, rows, (0, 0), (0, 3)).expect("应该能从棋盘外绕过去");
    assert!(
        path.iter().any(|&(x, y)| x < 0 || y < 0 || x >= cols || y >= rows),
        "路径没有走到棋盘外：{path:?}"
    );
}

/// 返回的路径必须真的是「拐弯不超过两次的折线」：
/// 至多 4 个折点、每段都是水平或垂直、中间格子全空。
#[test]
fn returned_paths_are_always_valid_two_turn_polylines() {
    let mut rng = StdRng::seed_from_u64(227);
    let (cols, rows) = (8, 6);
    for _ in 0..40 {
        let mut tiles = build_tiles(cols, rows, 8, &mut rng);
        // 随机挖掉一些格子，制造各种绕行形状
        for tile in tiles.iter_mut() {
            if rng.gen_bool(0.35) {
                *tile = None;
            }
        }
        for a in 0..tiles.len() {
            for b in a + 1..tiles.len() {
                if tiles[a].is_none() || tiles[a] != tiles[b] {
                    continue;
                }
                let pa = ((a as i32) % cols, (a as i32) / cols);
                let pb = ((b as i32) % cols, (b as i32) / cols);
                let Some(path) = find_path(&tiles, cols, rows, pa, pb) else {
                    continue;
                };
                assert_eq!(path.first(), Some(&pa));
                assert_eq!(path.last(), Some(&pb));
                assert!(path.len() <= 4, "拐了超过两次：{path:?}");
                for pair in path.windows(2) {
                    let (p, q) = (pair[0], pair[1]);
                    assert!(p.0 == q.0 || p.1 == q.1, "有一段不是直线：{path:?}");
                    let step = ((q.0 - p.0).signum(), (q.1 - p.1).signum());
                    let mut cur = (p.0 + step.0, p.1 + step.1);
                    while cur != q {
                        let inside = cur.0 >= 0 && cur.1 >= 0 && cur.0 < cols && cur.1 < rows;
                        if inside {
                            assert!(
                                tiles[(cur.1 * cols + cur.0) as usize].is_none(),
                                "路径穿过了没消掉的图案：{path:?}"
                            );
                        }
                        cur = (cur.0 + step.0, cur.1 + step.1);
                    }
                }
            }
        }
    }
}

#[test]
fn a_tile_never_links_to_itself() {
    let (tiles, cols, rows) = board(&["A..A"]);
    assert!(find_path(&tiles, cols, rows, (0, 0), (0, 0)).is_none());
}

#[test]
fn generated_boards_are_fully_pairable() {
    let mut rng = StdRng::seed_from_u64(211);
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cols, rows) = grid_for(level, age);
            assert_eq!((cols * rows) % 2, 0, "第 {level} 关格数是奇数，配不成整对");
            let tiles = build_tiles(cols, rows, patterns_for(level, age), &mut rng);
            assert_eq!(tiles.len(), (cols * rows) as usize);
            for pattern in 0..patterns_for(level, age) as u8 {
                let count = tiles.iter().filter(|t| **t == Some(pattern)).count();
                assert_eq!(count % 2, 0, "图案 {pattern} 是奇数张");
            }
        }
    }
}

#[test]
fn patterns_never_exceed_the_palette_or_the_pair_count() {
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cols, rows) = grid_for(level, age);
            let patterns = patterns_for(level, age);
            assert!(patterns <= palette::PATTERN_CHARS.len());
            assert!(patterns <= palette::PATTERN_COLORS.len());
            assert!(patterns <= (cols * rows / 2) as usize, "第 {level} 关图案种类比对数还多");
            assert!(patterns >= 1);
        }
    }
}

#[test]
fn gravity_packs_every_column_to_the_bottom() {
    let (mut tiles, cols, rows) = board(&["AB", "..", "C."]);
    apply_gravity(&mut tiles, cols, rows);
    // A 落到第 0 列底下 C 的上面，B 落到第 1 列底
    assert_eq!(tiles[(2 * cols) as usize], Some(2), "C 应该还在列底");
    assert_eq!(tiles[cols as usize], Some(0), "A 应该落到 C 上面");
    assert_eq!(tiles[(2 * cols + 1) as usize], Some(1), "B 应该落到列底");
    assert!(tiles[0].is_none() && tiles[1].is_none(), "顶部应该空出来");
}

#[test]
fn reshuffle_keeps_the_same_multiset_of_patterns() {
    let mut rng = StdRng::seed_from_u64(223);
    let (mut tiles, _, _) = board(&["AABB", "CC..", "DDEE"]);
    let mut before: Vec<u8> = tiles.iter().filter_map(|t| *t).collect();
    let holes: Vec<bool> = tiles.iter().map(|t| t.is_none()).collect();
    reshuffle(&mut tiles, &mut rng);
    let mut after: Vec<u8> = tiles.iter().filter_map(|t| *t).collect();
    before.sort_unstable();
    after.sort_unstable();
    assert_eq!(before, after, "洗牌不能改变图案的数量");
    let holes_after: Vec<bool> = tiles.iter().map(|t| t.is_none()).collect();
    assert_eq!(holes, holes_after, "洗牌不能填掉已经消掉的格子");
}

/// 有没有可消的对必须查得准，否则自动洗牌要么不触发、要么乱触发。
#[test]
fn available_moves_are_detected_consistently() {
    // 相邻的同图案一定能消
    let (tiles, cols, rows) = board(&["AB", "BA", "AB", "BA"]);
    assert!(has_any_move(&tiles, cols, rows));
    let found = find_any_move(&tiles, cols, rows).expect("应该找得到一对");
    assert_eq!(
        tiles[(found.0.1 * cols + found.0.0) as usize],
        tiles[(found.1.1 * cols + found.1.0) as usize],
        "找出来的两张图案必须一样"
    );
    assert!(find_path(&tiles, cols, rows, found.0, found.1).is_some());

    // 空盘面没有可消的对
    let empty: Vec<Option<u8>> = vec![None; 16];
    assert!(!has_any_move(&empty, 4, 4));
    assert!(find_any_move(&empty, 4, 4).is_none());

    // 只剩一张（不成对）也没得消
    let mut lonely: Vec<Option<u8>> = vec![None; 16];
    lonely[5] = Some(0);
    assert!(!has_any_move(&lonely, 4, 4));
}

#[test]
fn junior_tier_is_smaller_simpler_and_never_drops_tiles() {
    for level in 1..=10u8 {
        let (w, h) = grid_for(level, AgeTier::Junior);
        assert!(w <= JUNIOR_MAX_GRID.0 && h <= JUNIOR_MAX_GRID.1);
        assert!(patterns_for(level, AgeTier::Junior) <= JUNIOR_MAX_PATTERNS);
        assert!(!gravity_for(level, AgeTier::Junior), "低龄档不该开下落");
    }
    assert!(!gravity_for(5, AgeTier::Senior));
    assert!(gravity_for(6, AgeTier::Senior));
    assert_eq!(grid_for(10, AgeTier::Senior), (10, 6));
}

#[test]
fn junior_tier_gets_more_time_per_pair() {
    for level in 1..=10u8 {
        let (jw, jh) = grid_for(level, AgeTier::Junior);
        let (sw, sh) = grid_for(level, AgeTier::Senior);
        let junior = time_for(level, AgeTier::Junior) / (jw * jh) as f32;
        let senior = time_for(level, AgeTier::Senior) / (sw * sh) as f32;
        assert!(junior > senior * 1.4, "第 {level} 关低龄档不够宽松");
    }
}

#[test]
fn streak_bonus_grows_then_caps() {
    assert_eq!(hit_score(1), 10);
    assert_eq!(hit_score(10), 37);
    assert_eq!(hit_score(99), 37);
}
