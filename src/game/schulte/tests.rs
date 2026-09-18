use crate::game::model::AgeTier;

use super::constants::*;
use super::resources::{SchulteMode, build_board};

fn sorted_values(board: &[(u32, u8)]) -> Vec<(u32, u8)> {
    let mut copy = board.to_vec();
    copy.sort_unstable();
    copy
}

#[test]
fn every_number_appears_exactly_once_on_the_board() {
    for mode in [
        SchulteMode::Ascending,
        SchulteMode::Descending,
        SchulteMode::DualColor,
    ] {
        for size in 3..=6 {
            let (cells, order) = build_board(size, mode);
            assert_eq!(cells.len(), (size * size) as usize, "{mode:?} {size} 格数不对");
            assert_eq!(order.len(), cells.len(), "{mode:?} {size} 目标序列长度不对");
            assert_eq!(
                sorted_values(&cells),
                sorted_values(&order),
                "{mode:?} {size} 目标序列与题面对不上"
            );
            let mut unique = sorted_values(&cells);
            unique.dedup();
            assert_eq!(unique.len(), cells.len(), "{mode:?} {size} 出现了重复数字");
        }
    }
}

#[test]
fn ascending_and_descending_are_exact_reverses() {
    let (_, up) = build_board(4, SchulteMode::Ascending);
    let (_, down) = build_board(4, SchulteMode::Descending);
    let mut reversed = down.clone();
    reversed.reverse();
    assert_eq!(up, reversed);
    assert_eq!(up.first(), Some(&(1, 0)));
    assert_eq!(down.first(), Some(&(16, 0)));
}

#[test]
fn dual_color_alternates_and_ends_on_warm_when_odd() {
    // 25 格 = 13 红 + 12 蓝，交替序列必须以红色收尾。
    let (cells, order) = build_board(5, SchulteMode::DualColor);
    assert_eq!(cells.iter().filter(|(_, c)| *c == 0).count(), 13);
    assert_eq!(cells.iter().filter(|(_, c)| *c == 1).count(), 12);
    assert_eq!(&order[..4], &[(1, 0), (1, 1), (2, 0), (2, 1)]);
    assert_eq!(order.last(), Some(&(13, 0)));

    // 36 格是偶数，红蓝各 18，以蓝色收尾。
    let (_, even) = build_board(6, SchulteMode::DualColor);
    assert_eq!(even.last(), Some(&(18, 1)));
}

#[test]
fn junior_tier_caps_the_grid_and_keeps_the_simple_mode() {
    for level in 1..=10u8 {
        assert!(size_for(level, AgeTier::Junior) <= JUNIOR_MAX_SIZE);
        assert_eq!(mode_for(level, AgeTier::Junior), SchulteMode::Ascending);
    }
    assert_eq!(size_for(10, AgeTier::Senior), 6);
    assert_eq!(mode_for(10, AgeTier::Senior), SchulteMode::DualColor);
    assert_eq!(mode_for(5, AgeTier::Senior), SchulteMode::Descending);
}

/// 低龄档每格的平均时间必须比高龄档宽松，否则"简单档"名不副实。
#[test]
fn junior_tier_gives_more_time_per_cell() {
    for level in 1..=10u8 {
        let junior = time_for(level, AgeTier::Junior) / (size_for(level, AgeTier::Junior).pow(2)) as f32;
        let senior = time_for(level, AgeTier::Senior) / (size_for(level, AgeTier::Senior).pow(2)) as f32;
        assert!(junior > senior * 1.4, "第 {level} 关低龄档不够宽松");
    }
}

#[test]
fn streak_bonus_grows_then_caps() {
    assert_eq!(hit_score(1), 10);
    assert_eq!(hit_score(2), 13);
    assert_eq!(hit_score(10), 37);
    assert_eq!(hit_score(99), 37);
}
