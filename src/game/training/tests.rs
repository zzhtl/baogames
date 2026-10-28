use super::logic::*;
use super::*;
use rand::{SeedableRng, rngs::StdRng};

#[test]
fn all_ten_games_generate_valid_boards_for_both_ages_and_every_level() {
    assert_eq!(GameKind::TRAINING.len(), 10);
    let mut rng = StdRng::seed_from_u64(2741);
    for &kind in GameKind::TRAINING {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            for level in 1..=10 {
                for _ in 0..12 {
                    let game = Challenge::new(kind, level, age, &mut rng);
                    let (cols, rows) = game.dimensions();
                    assert!(cols > 0 && rows > 0);
                    match game {
                        Challenge::Pattern(g) => {
                            assert_eq!(g.options.iter().filter(|&&v| v == g.answer).count(), 1);
                            let mut unique = g.options.clone();
                            unique.sort();
                            unique.dedup();
                            assert_eq!(unique.len(), 4);
                            assert!(g.sequence.len() >= 5);
                        }
                        Challenge::Flash(g) => {
                            assert!(g.target.iter().any(|&on| on));
                            assert!(g.target.iter().any(|&on| !on));
                            assert!(g.preview >= 2.0);
                        }
                        Challenge::Mirror(g) => assert_ne!(g.source, reflect(&g.source, g.n)),
                        Challenge::Reaction(g) => assert!(g.remaining >= 1.0),
                        Challenge::Balance(g) => {
                            assert!(
                                (1..1usize << g.weights.len()).any(|mask| {
                                    g.weights
                                        .iter()
                                        .enumerate()
                                        .filter_map(|(i, &w)| {
                                            (mask & (1 << i) != 0).then_some(w as u16)
                                        })
                                        .sum::<u16>()
                                        == g.target
                                }),
                                "every balance target must be attainable"
                            );
                        }
                        Challenge::Hanoi(g) => assert!((2..=5).contains(&g.disks)),
                        Challenge::Lights(g) => assert!(g.on.iter().any(|&on| on)),
                        Challenge::Coding(g) => {
                            assert!(!g.blocked[0] && !g.blocked[g.n * g.n - 1]);
                            assert!(find_route(&g.blocked, g.n).is_some());
                        }
                        Challenge::Rotation(g) => {
                            assert_ne!(g.target, g.current);
                            let mut rotated = g.current.clone();
                            assert!((0..3).any(|_| {
                                rotated = rotate(&rotated, g.n);
                                rotated == g.target
                            }));
                        }
                        Challenge::Detective(g) => {
                            let targets = g.items.iter().filter(|&&i| i == g.target).count();
                            assert!(targets >= 3 && targets < g.items.len());
                        }
                    }
                }
            }
        }
    }
}

fn find_route(board: &[bool], n: usize) -> Option<Vec<Direction>> {
    let mut queue = std::collections::VecDeque::from([0]);
    let mut previous = vec![None; n * n];
    let mut seen = vec![false; n * n];
    seen[0] = true;
    while let Some(at) = queue.pop_front() {
        if at == n * n - 1 {
            let mut path = vec![];
            let mut end = at;
            while let Some((from, dir)) = previous[end] {
                path.push(dir);
                end = from;
            }
            path.reverse();
            return Some(path);
        }
        for dir in Direction::ALL {
            let (dx, dy) = dir.delta();
            let (x, y) = ((at % n) as i32 + dx, (at / n) as i32 + dy);
            if x < 0 || y < 0 || x >= n as i32 || y >= n as i32 {
                continue;
            }
            let next = y as usize * n + x as usize;
            if !board[next] && !seen[next] {
                seen[next] = true;
                previous[next] = Some((at, dir));
                queue.push_back(next);
            }
        }
    }
    None
}

#[test]
fn lights_scramble_can_always_be_reversed() {
    let mut rng = StdRng::seed_from_u64(19);
    for n in [3, 4] {
        for steps in 1..=7 {
            for _ in 0..40 {
                let (mut board, moves) = lights_board(&mut rng, n, steps);
                for at in moves {
                    toggle_cross(&mut board, n, at);
                }
                assert!(board.iter().all(|&on| !on));
            }
        }
    }
}

#[test]
fn mirror_and_rotation_require_the_transformed_picture() {
    let source = vec![true, true, false, false, false, false, false, false, true];
    assert_eq!(
        reflect(&source, 3),
        vec![false, true, true, false, false, false, true, false, false]
    );
    let mut copy = source.clone();
    for _ in 0..4 {
        copy = rotate(&copy, 3);
    }
    assert_eq!(copy, source);
    let mut game = Challenge::Mirror(Mirror {
        n: 3,
        source: source.clone(),
        marked: source.clone(),
    });
    assert!(matches!(
        game.act(Action::Secondary, &mut 0),
        Outcome::Mistake(_)
    ));
    if let Challenge::Mirror(g) = &mut game {
        g.marked = reflect(&source, 3);
    }
    assert_eq!(game.act(Action::Secondary, &mut 0), Outcome::Solved);
}

#[test]
fn flash_grid_hides_before_it_accepts_input() {
    let mut game = Challenge::Flash(GridRecall {
        n: 2,
        target: vec![true, false, false, true],
        marked: vec![false; 4],
        preview: 2.0,
    });
    assert_eq!(game.act(Action::Pick(0), &mut 0), Outcome::Quiet);
    game.tick(2.1);
    game.act(Action::Pick(0), &mut 0);
    assert!(matches!(
        game.act(Action::Secondary, &mut 0),
        Outcome::Mistake(_)
    ));
    game.act(Action::Pick(3), &mut 0);
    assert_eq!(game.act(Action::Secondary, &mut 0), Outcome::Solved);
}

#[test]
fn reaction_rewards_waiting_and_rejects_distractors() {
    for green in [false, true] {
        for round_shape in [false, true] {
            for shape_rule in [false, true] {
                let stimulus = Reaction {
                    green,
                    round_shape,
                    shape_rule,
                    remaining: 1.0,
                };
                let should_press = stimulus.should_press();
                let mut press = Challenge::Reaction(stimulus.clone());
                let mut wait = Challenge::Reaction(stimulus);
                assert_eq!(
                    press.act(Action::Primary, &mut 0) == Outcome::Solved,
                    should_press
                );
                assert_eq!(wait.tick(1.1) == Outcome::Solved, !should_press);
            }
        }
    }
}

#[test]
fn hanoi_validates_disk_sizes_and_undoes_moves() {
    let mut game = Hanoi::new(3);
    game.pick(0);
    game.pick(2);
    assert_eq!(game.towers[2], vec![1]);
    game.pick(0);
    assert!(matches!(game.pick(2), Outcome::Mistake(_)));
    assert_eq!(game.towers[0], vec![3, 2]);
    game.undo();
    assert_eq!(game.towers[0], vec![3, 2, 1]);
    fn solve(game: &mut Hanoi, n: usize, from: usize, to: usize, spare: usize) {
        if n == 0 {
            return;
        }
        solve(game, n - 1, from, spare, to);
        game.pick(from);
        game.pick(to);
        solve(game, n - 1, spare, to, from);
    }
    solve(&mut game, 3, 0, 2, 1);
    assert_eq!(game.towers[2], vec![3, 2, 1]);
    assert_eq!(game.history.len(), 7);
}

#[test]
fn coding_executes_generated_routes_and_catches_collisions() {
    let mut rng = StdRng::seed_from_u64(73);
    for n in 4..=6 {
        for _ in 0..80 {
            let (blocked, program) = coding_board(&mut rng, n, 0.4);
            let mut game = Coding {
                n,
                blocked,
                program,
                robot: 0,
                step: 0,
                running: true,
                clock: 0.0,
            };
            for _ in 0..game.program.len() {
                assert_eq!(game.advance(), Outcome::Changed);
            }
            assert_eq!(game.advance(), Outcome::Solved);
        }
    }
    let mut game = Coding {
        n: 3,
        blocked: vec![false; 9],
        program: vec![Direction::Left],
        robot: 0,
        step: 0,
        running: true,
        clock: 0.0,
    };
    assert!(matches!(game.advance(), Outcome::Mistake(_)));
    assert!(!game.running);
}

#[test]
fn detective_requires_both_features_and_ignores_repeated_marks() {
    let target = Feature { shape: 0, color: 1 };
    let mut game = Challenge::Detective(Detective {
        cols: 2,
        rows: 2,
        target,
        items: vec![
            target,
            Feature { shape: 1, color: 1 },
            Feature { shape: 0, color: 2 },
            target,
        ],
        found: vec![false; 4],
    });
    assert!(matches!(
        game.act(Action::Pick(1), &mut 0),
        Outcome::Mistake(_)
    ));
    assert!(matches!(
        game.act(Action::Pick(2), &mut 0),
        Outcome::Mistake(_)
    ));
    assert_eq!(game.act(Action::Pick(0), &mut 0), Outcome::Changed);
    assert_eq!(game.act(Action::Pick(0), &mut 0), Outcome::Quiet);
    assert_eq!(game.act(Action::Pick(3), &mut 0), Outcome::Solved);
}

#[test]
fn sequence_answers_and_weight_selection_use_the_actual_choices() {
    let mut pattern = Challenge::Pattern(Pattern {
        sequence: vec![2, 4, 6, 8, 10],
        options: vec![11, 13, 12, 14],
        answer: 12,
        numeric: true,
    });
    assert!(matches!(
        pattern.act(Action::Pick(0), &mut 0),
        Outcome::Mistake(_)
    ));
    assert_eq!(pattern.act(Action::Pick(2), &mut 0), Outcome::Solved);
    let mut balance = Challenge::Balance(Balance {
        weights: vec![1, 2, 3, 4, 5, 6],
        selected: vec![false; 6],
        target: 4,
    });
    balance.act(Action::Pick(0), &mut 0);
    assert!(matches!(
        balance.act(Action::Secondary, &mut 0),
        Outcome::Mistake(_)
    ));
    balance.act(Action::Pick(2), &mut 0);
    assert_eq!(balance.act(Action::Secondary, &mut 0), Outcome::Solved);
    balance.act(Action::Pick(2), &mut 0);
    assert!(matches!(
        balance.act(Action::Secondary, &mut 0),
        Outcome::Mistake(_)
    ));
}

#[test]
fn corner_lights_do_not_wrap_and_undo_restores_board() {
    let mut board = vec![false; 9];
    toggle_cross(&mut board, 3, 0);
    assert_eq!(
        board,
        vec![true, true, false, true, false, false, false, false, false]
    );
    let mut game = Challenge::Lights(Lights {
        n: 3,
        on: board.clone(),
        history: vec![],
    });
    game.act(Action::Pick(8), &mut 0);
    game.act(Action::Secondary, &mut 0);
    if let Challenge::Lights(g) = &game {
        assert_eq!(g.on, board);
    }
    assert_eq!(game.act(Action::Pick(0), &mut 0), Outcome::Solved);
}

#[test]
fn rotation_buttons_and_keyboard_can_reach_the_target() {
    let target = vec![true, true, false, false, false, false, false, false, true];
    let mut game = Challenge::Rotation(Rotation {
        n: 3,
        current: rotate(&target, 3),
        target,
    });
    assert!(matches!(
        game.act(Action::Primary, &mut 0),
        Outcome::Mistake(_)
    ));
    game.act(Action::Pick(0), &mut 0);
    assert_eq!(game.act(Action::Primary, &mut 0), Outcome::Solved);
}
