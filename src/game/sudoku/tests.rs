use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::generator::{SudokuSpec, carve, conflicts, count_solutions, full_solution, is_legal, is_solved};

fn specs() -> [SudokuSpec; 3] {
    [
        SudokuSpec::for_size(4),
        SudokuSpec::for_size(6),
        SudokuSpec::for_size(9),
    ]
}

#[test]
fn box_shapes_tile_the_board_exactly() {
    for spec in specs() {
        assert_eq!(spec.size % spec.box_w, 0, "{spec:?} 宫宽除不尽");
        assert_eq!(spec.size % spec.box_h, 0, "{spec:?} 宫高除不尽");
        assert_eq!(spec.box_w * spec.box_h, spec.size, "{spec:?} 一个宫装不下 size 个数");
    }
    // 6 阶数独的宫是 3 宽 2 高，不是正方形
    assert_eq!(SudokuSpec::for_size(6).box_w, 3);
    assert_eq!(SudokuSpec::for_size(6).box_h, 2);
}

#[test]
fn generated_solutions_are_complete_and_legal() {
    let mut rng = StdRng::seed_from_u64(3);
    for spec in specs() {
        for _ in 0..4 {
            let board = full_solution(spec, &mut rng);
            assert_eq!(board.len(), spec.cells());
            assert!(board.iter().all(|&v| (1..=spec.size as u8).contains(&v)));
            assert!(is_solved(&board, spec), "{spec:?} 生成的解不合法");
            assert!(conflicts(&board, spec).iter().all(|hit| !hit));
        }
    }
}

#[test]
fn a_full_solution_has_exactly_one_solution() {
    let mut rng = StdRng::seed_from_u64(5);
    for spec in specs() {
        let board = full_solution(spec, &mut rng);
        assert_eq!(count_solutions(&board, spec, 3), 1);
    }
}

/// 挖空后必须仍然唯一可解 —— 多解的数独是没法判对错的。
#[test]
fn carved_puzzles_keep_a_unique_solution() {
    let mut rng = StdRng::seed_from_u64(9);
    for spec in specs() {
        let solution = full_solution(spec, &mut rng);
        let (puzzle, removed) = carve(&solution, spec, max_holes(spec.size), &mut rng);
        assert_eq!(count_solutions(&puzzle, spec, 3), 1, "{spec:?} 挖出了多解题");
        assert!(removed > 0, "{spec:?} 一格都没挖掉");
        // 留下的给定格必须与原解一致
        for (index, &value) in puzzle.iter().enumerate() {
            if value != 0 {
                assert_eq!(value, solution[index]);
            }
        }
    }
}

#[test]
fn illegal_placements_are_rejected() {
    let spec = SudokuSpec::for_size(4);
    let mut board = vec![0u8; spec.cells()];
    board[spec.index(0, 0)] = 1;
    assert!(!is_legal(&board, spec, spec.index(3, 0), 1), "同行重复应判非法");
    assert!(!is_legal(&board, spec, spec.index(0, 3), 1), "同列重复应判非法");
    assert!(!is_legal(&board, spec, spec.index(1, 1), 1), "同宫重复应判非法");
    assert!(is_legal(&board, spec, spec.index(2, 2), 1), "不同行列宫应合法");
    // 0 是空格，永远合法
    assert!(is_legal(&board, spec, spec.index(3, 0), 0));
}

#[test]
fn conflicts_flag_both_offenders() {
    let spec = SudokuSpec::for_size(4);
    let mut board = vec![0u8; spec.cells()];
    board[spec.index(0, 0)] = 2;
    board[spec.index(3, 0)] = 2;
    let hits = conflicts(&board, spec);
    assert!(hits[spec.index(0, 0)] && hits[spec.index(3, 0)], "冲突双方都要标红");
    assert!(!hits[spec.index(1, 1)], "空格不该被标红");
}

#[test]
fn junior_tier_never_reaches_nine_by_nine() {
    for level in 1..=10u8 {
        assert!(size_for(level, AgeTier::Junior) <= JUNIOR_MAX_SIZE);
        assert!(holes_for(level, AgeTier::Junior) <= max_holes(size_for(level, AgeTier::Junior)));
    }
    assert_eq!(size_for(10, AgeTier::Senior), 9);
    assert_eq!(size_for(1, AgeTier::Senior), 4);
}

/// 低龄档被压回 6×6 后要靠多挖空补难度，否则第 7-10 关会比第 6 关还简单。
#[test]
fn junior_tier_keeps_getting_harder_after_the_size_cap() {
    let holes: Vec<usize> = (6..=10u8).map(|l| holes_for(l, AgeTier::Junior)).collect();
    for pair in holes.windows(2) {
        assert!(pair[1] >= pair[0], "低龄档挖空数出现回落：{holes:?}");
    }
    assert!(holes[holes.len() - 1] > holes[0], "低龄档后段没有变难：{holes:?}");
}

#[test]
fn every_level_has_a_sane_time_budget() {
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let size = size_for(level, age);
            let seconds = time_for(level, age);
            let per_hole = seconds / holes_for(level, age).max(1) as f32;
            assert!(per_hole >= 6.0, "第 {level} 关 {age:?} 档平均每空只有 {per_hole:.1} 秒");
            assert!(seconds <= 900.0, "第 {level} 关 {size}×{size} 限时过长");
        }
    }
}

/// 9×9 每局现生成，必须在一帧里跑完 —— 太慢会让进关卡顿。
#[test]
fn generating_the_hardest_puzzle_is_fast_enough() {
    let spec = SudokuSpec::for_size(9);
    let mut rng = StdRng::seed_from_u64(21);
    let start = std::time::Instant::now();
    let solution = full_solution(spec, &mut rng);
    let (puzzle, removed) = carve(&solution, spec, 54, &mut rng);
    let elapsed = start.elapsed();
    assert_eq!(count_solutions(&puzzle, spec, 2), 1);
    assert!(removed >= 40, "9×9 只挖掉了 {removed} 格，题目太简单");
    assert!(elapsed.as_secs() < 8, "9×9 生成耗时 {elapsed:?}，进关会明显卡顿");
}
