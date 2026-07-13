use super::constants::{LEVEL_GRID, LEVEL_TIME, PAIR_CHARS};
use super::setup::shuffled_pair_ids;
use super::systems::match_score;

#[test]
fn level_grids_have_even_cells_and_enough_chars() {
    assert_eq!(LEVEL_GRID.len(), LEVEL_TIME.len());
    for (i, (cols, rows)) in LEVEL_GRID.iter().enumerate() {
        let total = cols * rows;
        assert!(
            total % 2 == 0,
            "level {} has odd cell count {}x{}={}",
            i + 1,
            cols,
            rows,
            total
        );
        let pairs = total / 2;
        assert!(
            pairs as usize <= PAIR_CHARS.len(),
            "level {} needs {} pair chars but only {} provided",
            i + 1,
            pairs,
            PAIR_CHARS.len()
        );
    }
}

#[test]
fn shuffled_pair_ids_balanced() {
    let pairs = 12;
    let ids = shuffled_pair_ids(pairs);
    assert_eq!(ids.len() as u32, pairs * 2);
    for pid in 0..pairs {
        let count = ids.iter().filter(|&&v| v == pid).count();
        assert_eq!(count, 2, "pair_id {} appears {} times", pid, count);
    }
}

#[test]
fn consecutive_matches_gain_a_small_streak_bonus() {
    assert_eq!(match_score(1), 10);
    assert_eq!(match_score(2), 15);
    assert_eq!(match_score(4), 25);
}
