use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::logic::{blank_index, is_solved, manhattan_lower_bound, shuffle, slide, solved_board, tile_to_slide};

/// 逆序数奇偶 + 空格行号决定可解性，这是 15-puzzle 的标准判据。
/// 只用来给测试当独立参照 —— 玩法代码靠"从已完成状态随机走"保证可解。
fn is_solvable(board: &[u8], size: usize) -> bool {
    let tiles: Vec<u8> = board.iter().copied().filter(|&v| v != 0).collect();
    let mut inversions = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            if tiles[i] > tiles[j] {
                inversions += 1;
            }
        }
    }
    if size % 2 == 1 {
        inversions % 2 == 0
    } else {
        let blank_row_from_bottom = size - blank_index(board) / size;
        (inversions + blank_row_from_bottom) % 2 == 1
    }
}

#[test]
fn solved_board_puts_the_blank_last() {
    for size in 3..=5 {
        let board = solved_board(size);
        assert_eq!(board.len(), size * size);
        assert_eq!(*board.last().unwrap(), 0);
        assert_eq!(board[0], 1);
        assert!(is_solved(&board));
    }
}

/// 方向键的语义是「块往哪边滑」：按右，动的是空格**左边**那块。
#[test]
fn direction_moves_the_tile_not_the_blank() {
    let size = 3;
    let mut board = solved_board(size);
    // 空格在下标 8（右下角）
    assert_eq!(blank_index(&board), 8);
    // 按「右」：空格左边（下标 7，值 8）的块往右滑
    assert_eq!(tile_to_slide(&board, size, (1, 0)), Some(7));
    assert_eq!(slide(&mut board, size, (1, 0)), Some(8));
    assert_eq!(blank_index(&board), 7);
    // 按「下」：空格上边（下标 4）的块往下滑
    assert_eq!(slide(&mut board, size, (0, 1)), Some(5));
}

#[test]
fn sliding_off_the_board_is_rejected() {
    let size = 3;
    let mut board = solved_board(size);
    // 空格在右下角，右边和下边都没有块可以滑进来
    assert_eq!(tile_to_slide(&board, size, (-1, 0)), None);
    assert_eq!(tile_to_slide(&board, size, (0, -1)), None);
    assert_eq!(slide(&mut board, size, (-1, 0)), None);
    assert_eq!(board, solved_board(size), "非法滑动不能改动盘面");
}

/// 直接随机排列有一半概率是死局，所以打乱必须从已完成状态走出来。
#[test]
fn shuffled_boards_are_always_solvable_and_not_already_done() {
    let mut rng = StdRng::seed_from_u64(31);
    for size in 3..=5 {
        for steps in [12, 40, 200, 400] {
            let board = shuffle(size, steps, &mut rng);
            assert_eq!(board.len(), size * size);
            let mut sorted = board.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, (0..(size * size) as u8).collect::<Vec<_>>(), "块的编号有缺失或重复");
            assert!(is_solvable(&board, size), "{size}×{size} 打乱出了死局");
            assert!(!is_solved(&board), "{size}×{size} 打乱后仍是完成状态");
        }
    }
}

#[test]
fn undoing_every_move_restores_the_board() {
    let mut rng = StdRng::seed_from_u64(37);
    let size = 4;
    let start = shuffle(size, 60, &mut rng);
    let mut board = start.clone();
    let mut history = Vec::new();
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 0), (0, 1)] {
        if slide(&mut board, size, dir).is_some() {
            history.push(dir);
        }
    }
    assert!(!history.is_empty());
    while let Some((dx, dy)) = history.pop() {
        assert!(slide(&mut board, size, (-dx, -dy)).is_some(), "撤销必须总能滑回去");
    }
    assert_eq!(board, start);
}

#[test]
fn lower_bound_is_zero_only_when_solved() {
    for size in 3..=5 {
        assert_eq!(manhattan_lower_bound(&solved_board(size), size), 0);
    }
    let mut rng = StdRng::seed_from_u64(41);
    let board = shuffle(4, 120, &mut rng);
    assert!(manhattan_lower_bound(&board, 4) > 0);
}

#[test]
fn junior_tier_caps_size_and_halves_the_shuffle() {
    for level in 1..=10u8 {
        assert!(size_for(level, AgeTier::Junior) <= JUNIOR_MAX_SIZE);
        assert!(
            shuffle_for(level, AgeTier::Junior) < shuffle_for(level, AgeTier::Senior),
            "第 {level} 关低龄档没有变简单"
        );
        assert!(shuffle_for(level, AgeTier::Junior) >= 12);
    }
    assert_eq!(size_for(10, AgeTier::Senior), 5);
    assert_eq!(size_for(1, AgeTier::Senior), 3);
}

#[test]
fn efficiency_bonus_rewards_short_solutions() {
    // 下界 20 步 → 50 步以内拿满分
    assert_eq!(efficiency_bonus(30, 20), EFFICIENCY_BONUS);
    assert_eq!(efficiency_bonus(50, 20), EFFICIENCY_BONUS);
    assert_eq!(efficiency_bonus(60, 20), EFFICIENCY_BONUS - 10);
    // 绕得太离谱也只是归零，不会反向加分
    assert_eq!(efficiency_bonus(9999, 20), 0);
}
