use super::constants::{LEVEL_TIME, LEVELS};
use super::resources::{SokobanStage, Tile};
use super::setup::parse_level;
use super::systems::try_move;
use bevy::prelude::*;

fn make_stage(lines: &[&str]) -> SokobanStage {
    let (cols, rows, tiles, boxes, player) = parse_level(lines);
    SokobanStage {
        cols,
        rows,
        cell_size: 1.0,
        origin: Vec2::ZERO,
        tiles,
        initial_boxes: boxes.clone(),
        initial_player: player,
        initial_time: 60.0,
        boxes,
        player,
        moves: 0,
        pushes: 0,
        time_left: 60.0,
        move_cd: 0.0,
        message: String::new(),
        message_clock: 0.0,
    }
}

#[test]
fn level_count_matches_time() {
    assert_eq!(LEVELS.len(), LEVEL_TIME.len());
}

#[test]
fn levels_have_matching_boxes_and_goals() {
    for (i, lvl) in LEVELS.iter().enumerate() {
        let (_, _, tiles, boxes, _) = parse_level(lvl);
        let goals = tiles.iter().filter(|t| **t == Tile::Goal).count();
        assert!(
            !boxes.is_empty(),
            "level {} has no boxes",
            i + 1
        );
        assert_eq!(
            boxes.len(),
            goals,
            "level {} boxes ({}) != goals ({})",
            i + 1,
            boxes.len(),
            goals
        );
    }
}

#[test]
fn levels_have_exactly_one_player() {
    for (i, lvl) in LEVELS.iter().enumerate() {
        let count: usize = lvl
            .iter()
            .map(|s| s.chars().filter(|c| *c == '@' || *c == '+').count())
            .sum();
        assert_eq!(count, 1, "level {} player markers = {}", i + 1, count);
    }
}

#[test]
fn cannot_walk_into_wall() {
    let lines = [
        "###",
        "#@#",
        "###",
    ];
    let mut s = make_stage(&lines);
    assert!(!try_move(&mut s, (1, 0)));
    assert!(!try_move(&mut s, (-1, 0)));
    assert!(!try_move(&mut s, (0, 1)));
    assert!(!try_move(&mut s, (0, -1)));
    assert_eq!(s.moves, 0);
}

#[test]
fn push_box_onto_goal_completes() {
    let lines = [
        "#####",
        "#   #",
        "#@$.#",
        "#   #",
        "#####",
    ];
    let mut s = make_stage(&lines);
    assert!(try_move(&mut s, (1, 0))); // 推箱子向右一格
    assert_eq!(s.pushes, 1);
    assert!(s.all_boxes_done(), "box should be on goal");
}

#[test]
fn cannot_push_two_boxes_in_a_row() {
    let lines = [
        "######",
        "#@$$.#",
        "######",
    ];
    let mut s = make_stage(&lines);
    assert!(!try_move(&mut s, (1, 0)));
    assert_eq!(s.pushes, 0);
    assert_eq!(s.moves, 0);
}

#[test]
fn cannot_push_box_into_wall() {
    let lines = [
        "####",
        "#@$#",
        "####",
    ];
    let mut s = make_stage(&lines);
    assert!(!try_move(&mut s, (1, 0)));
    assert_eq!(s.pushes, 0);
}
