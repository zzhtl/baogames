//! 十种独立题型的纯规则。生成器接收 RNG，便于固定种子验证可解性。
use rand::{Rng, seq::SliceRandom};

use crate::game::model::{AgeTier, GameKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Action {
    Move(i32, i32),
    Pick(usize),
    Primary,
    Secondary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Quiet,
    Changed,
    Solved,
    Mistake(&'static str),
}

#[derive(Clone, Debug)]
pub(super) struct Pattern {
    pub sequence: Vec<u8>,
    pub options: Vec<u8>,
    pub answer: u8,
    pub numeric: bool,
}

#[derive(Clone, Debug)]
pub(super) struct GridRecall {
    pub n: usize,
    pub target: Vec<bool>,
    pub marked: Vec<bool>,
    pub preview: f32,
}

#[derive(Clone, Debug)]
pub(super) struct Mirror {
    pub n: usize,
    pub source: Vec<bool>,
    pub marked: Vec<bool>,
}

#[derive(Clone, Debug)]
pub(super) struct Reaction {
    pub green: bool,
    pub round_shape: bool,
    pub shape_rule: bool,
    pub remaining: f32,
}

impl Reaction {
    pub fn should_press(&self) -> bool {
        self.green && (!self.shape_rule || self.round_shape)
    }
}

#[derive(Clone, Debug)]
pub(super) struct Balance {
    pub weights: Vec<u8>,
    pub selected: Vec<bool>,
    pub target: u16,
}

impl Balance {
    pub fn sum(&self) -> u16 {
        self.weights
            .iter()
            .zip(&self.selected)
            .filter_map(|(&w, &on)| on.then_some(w as u16))
            .sum()
    }
}

#[derive(Clone, Debug)]
pub(super) struct Hanoi {
    pub disks: usize,
    pub towers: [Vec<usize>; 3],
    pub held: Option<usize>,
    pub history: Vec<(usize, usize)>,
}

impl Hanoi {
    pub fn new(disks: usize) -> Self {
        Self {
            disks,
            towers: [(1..=disks).rev().collect(), vec![], vec![]],
            held: None,
            history: vec![],
        }
    }

    pub fn pick(&mut self, to: usize) -> Outcome {
        let Some(from) = self.held else {
            if self.towers[to].is_empty() {
                return Outcome::Quiet;
            }
            self.held = Some(to);
            return Outcome::Changed;
        };
        if from == to {
            self.held = None;
            return Outcome::Changed;
        }
        let Some(&disk) = self.towers[from].last() else {
            self.held = None;
            return Outcome::Quiet;
        };
        if self.towers[to].last().is_some_and(|&top| top < disk) {
            return Outcome::Mistake("大圆盘不能放在小圆盘上，再选一根柱子");
        }
        self.towers[from].pop();
        self.towers[to].push(disk);
        self.held = None;
        self.history.push((from, to));
        if self.towers[2].len() == self.disks {
            Outcome::Solved
        } else {
            Outcome::Changed
        }
    }

    pub fn undo(&mut self) {
        self.held = None;
        if let Some((from, to)) = self.history.pop()
            && let Some(disk) = self.towers[to].pop()
        {
            self.towers[from].push(disk);
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Lights {
    pub n: usize,
    pub on: Vec<bool>,
    pub history: Vec<usize>,
}

pub(super) fn toggle_cross(board: &mut [bool], n: usize, at: usize) {
    let (x, y) = ((at % n) as i32, (at / n) as i32);
    for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (cx, cy) = (x + dx, y + dy);
        if cx >= 0 && cy >= 0 && cx < n as i32 && cy < n as i32 {
            let i = cy as usize * n + cx as usize;
            board[i] = !board[i];
        }
    }
}

pub(super) fn lights_board(rng: &mut impl Rng, n: usize, steps: usize) -> (Vec<bool>, Vec<usize>) {
    loop {
        let mut moves: Vec<usize> = (0..n * n).collect();
        moves.shuffle(rng);
        moves.truncate(steps.min(n * n));
        let mut board = vec![false; n * n];
        for &at in &moves {
            toggle_cross(&mut board, n, at);
        }
        if board.iter().any(|&on| on) {
            return (board, moves);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];
    pub fn delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Up => "↑",
            Self::Right => "→",
            Self::Down => "↓",
            Self::Left => "←",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Coding {
    pub n: usize,
    pub blocked: Vec<bool>,
    pub program: Vec<Direction>,
    pub robot: usize,
    pub step: usize,
    pub running: bool,
    pub clock: f32,
}

impl Coding {
    pub fn append(&mut self, direction: Direction) -> Outcome {
        if self.running {
            return Outcome::Quiet;
        }
        if self.program.len() >= self.n * 2 + 4 {
            return Outcome::Mistake("指令槽满了，先撤销多余的步骤");
        }
        self.program.push(direction);
        self.robot = 0;
        Outcome::Changed
    }

    pub fn advance(&mut self) -> Outcome {
        if !self.running {
            return Outcome::Quiet;
        }
        let Some(direction) = self.program.get(self.step) else {
            self.running = false;
            return if self.robot == self.n * self.n - 1 {
                Outcome::Solved
            } else {
                Outcome::Mistake("还没到终点，继续补充或调整指令")
            };
        };
        let (dx, dy) = direction.delta();
        let (x, y) = (
            (self.robot % self.n) as i32 + dx,
            (self.robot / self.n) as i32 + dy,
        );
        if x < 0
            || y < 0
            || x >= self.n as i32
            || y >= self.n as i32
            || self.blocked[y as usize * self.n + x as usize]
        {
            self.running = false;
            return Outcome::Mistake("撞到边界或障碍了，调整指令再试一次");
        }
        self.robot = y as usize * self.n + x as usize;
        self.step += 1;
        Outcome::Changed
    }
}

pub(super) fn coding_board(
    rng: &mut impl Rng,
    n: usize,
    density: f64,
) -> (Vec<bool>, Vec<Direction>) {
    let mut path = vec![Direction::Right; n - 1];
    path.extend(vec![Direction::Down; n - 1]);
    path.shuffle(rng);
    let mut board: Vec<bool> = (0..n * n).map(|_| rng.gen_bool(density)).collect();
    board[0] = false;
    let mut at = 0;
    for dir in &path {
        at += if *dir == Direction::Right { 1 } else { n };
        board[at] = false;
    }
    (board, path)
}

#[derive(Clone, Debug)]
pub(super) struct Rotation {
    pub n: usize,
    pub target: Vec<bool>,
    pub current: Vec<bool>,
}

pub(super) fn rotate(board: &[bool], n: usize) -> Vec<bool> {
    let mut result = vec![false; n * n];
    for y in 0..n {
        for x in 0..n {
            result[x * n + n - 1 - y] = board[y * n + x];
        }
    }
    result
}

pub(super) fn reflect(board: &[bool], n: usize) -> Vec<bool> {
    (0..n * n)
        .map(|i| board[(i / n) * n + n - 1 - i % n])
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Feature {
    pub shape: u8,
    pub color: u8,
}

#[derive(Clone, Debug)]
pub(super) struct Detective {
    pub cols: usize,
    pub rows: usize,
    pub target: Feature,
    pub items: Vec<Feature>,
    pub found: Vec<bool>,
}

#[derive(Clone, Debug)]
pub(super) enum Challenge {
    Pattern(Pattern),
    Flash(GridRecall),
    Mirror(Mirror),
    Reaction(Reaction),
    Balance(Balance),
    Hanoi(Hanoi),
    Lights(Lights),
    Coding(Coding),
    Rotation(Rotation),
    Detective(Detective),
}

fn random_cells(rng: &mut impl Rng, n: usize, count: usize) -> Vec<bool> {
    let mut indices: Vec<usize> = (0..n * n).collect();
    indices.shuffle(rng);
    let mut board = vec![false; n * n];
    for &i in indices.iter().take(count) {
        board[i] = true;
    }
    board
}

impl Challenge {
    pub fn new(kind: GameKind, level: u8, age: AgeTier, rng: &mut impl Rng) -> Self {
        let junior = age.is_junior();
        let level = level.clamp(1, 10) as usize;
        match kind {
            GameKind::Pattern => {
                let (sequence, answer, numeric) = if junior {
                    let period = if level < 5 { 2 } else { 3 };
                    let mut symbols = [0, 1, 2, 3];
                    symbols.shuffle(rng);
                    (
                        (0..5).map(|i| symbols[i % period]).collect(),
                        symbols[5 % period],
                        false,
                    )
                } else {
                    let start: u8 = rng.gen_range(1..=9);
                    let step: u8 = rng.gen_range(1..=2 + (level / 3) as u8);
                    let mut terms = vec![start];
                    for i in 0..5 {
                        terms.push(terms[i] + step + if level >= 6 { (i % 2) as u8 } else { 0 });
                    }
                    (terms[..5].to_vec(), terms[5], true)
                };
                let mut options = if numeric {
                    vec![answer, answer + 1, answer + 2, answer.saturating_sub(1)]
                } else {
                    vec![0, 1, 2, 3]
                };
                options.shuffle(rng);
                Self::Pattern(Pattern {
                    sequence,
                    options,
                    answer,
                    numeric,
                })
            }
            GameKind::FlashGrid => {
                let n = if junior { 3 } else { 4 };
                let count = if junior { 3 + level / 4 } else { 4 + level / 2 };
                Self::Flash(GridRecall {
                    n,
                    target: random_cells(rng, n, count),
                    marked: vec![false; n * n],
                    preview: if junior {
                        4.5
                    } else {
                        3.2 - level as f32 * 0.08
                    },
                })
            }
            GameKind::Mirror => {
                let n = if junior {
                    3 + usize::from(level >= 8)
                } else {
                    4 + usize::from(level >= 6)
                };
                let source = loop {
                    let board = random_cells(rng, n, n + level / 3);
                    if reflect(&board, n) != board {
                        break board;
                    }
                };
                Self::Mirror(Mirror {
                    n,
                    source,
                    marked: vec![false; n * n],
                })
            }
            GameKind::GoNoGo => Self::Reaction(Reaction {
                green: rng.gen_bool(0.6),
                round_shape: rng.gen_bool(0.7),
                shape_rule: !junior && level >= 6,
                remaining: if junior {
                    2.8
                } else {
                    2.0 - level as f32 * 0.07
                },
            }),
            GameKind::Balance => {
                let count = if junior { 6 } else { 8 };
                let weights: Vec<u8> = (0..count)
                    .map(|_| {
                        rng.gen_range(
                            1..=if junior {
                                4 + level as u8 / 5
                            } else {
                                9 + level as u8
                            },
                        )
                    })
                    .collect();
                let mut indices: Vec<usize> = (0..count).collect();
                indices.shuffle(rng);
                let take = if junior {
                    2
                } else {
                    3 + usize::from(level >= 6)
                };
                let target = indices.iter().take(take).map(|&i| weights[i] as u16).sum();
                Self::Balance(Balance {
                    weights,
                    selected: vec![false; count],
                    target,
                })
            }
            GameKind::Hanoi => Self::Hanoi(Hanoi::new(if junior {
                2 + usize::from(level >= 4) + usize::from(level >= 8)
            } else {
                3 + level / 4
            })),
            GameKind::LightsOut => {
                let n = if junior || level < 5 { 3 } else { 4 };
                let (on, _) =
                    lights_board(rng, n, if junior { 1 + level / 3 } else { 2 + level / 2 });
                Self::Lights(Lights {
                    n,
                    on,
                    history: vec![],
                })
            }
            GameKind::Coding => {
                let n = if junior {
                    4 + usize::from(level >= 6)
                } else {
                    5 + usize::from(level >= 5)
                };
                let (blocked, _) = coding_board(rng, n, if junior { 0.25 } else { 0.40 });
                Self::Coding(Coding {
                    n,
                    blocked,
                    program: vec![],
                    robot: 0,
                    step: 0,
                    running: false,
                    clock: 0.0,
                })
            }
            GameKind::Rotation => {
                let n = if junior { 3 } else { 4 };
                let target = loop {
                    let board = random_cells(rng, n, n + 1 + level / 4);
                    let turn = rotate(&board, n);
                    if turn != board && rotate(&turn, n) != board {
                        break board;
                    }
                };
                let mut current = target.clone();
                for _ in 0..rng.gen_range(1..=3) {
                    current = rotate(&current, n);
                }
                Self::Rotation(Rotation { n, target, current })
            }
            GameKind::Detective => {
                let (cols, rows, colors, shapes) = if junior { (4, 3, 2, 2) } else { (6, 4, 3, 4) };
                let target = Feature {
                    shape: rng.gen_range(0..shapes),
                    color: rng.gen_range(0..colors),
                };
                let targets = if junior { 3 + level / 5 } else { 5 + level / 4 };
                let mut items = vec![target; targets];
                while items.len() < cols * rows {
                    let item = Feature {
                        shape: rng.gen_range(0..shapes),
                        color: rng.gen_range(0..colors),
                    };
                    if item != target {
                        items.push(item);
                    }
                }
                items.shuffle(rng);
                Self::Detective(Detective {
                    cols,
                    rows,
                    target,
                    items,
                    found: vec![false; cols * rows],
                })
            }
            _ => unreachable!("only training games construct a challenge"),
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Self::Pattern(_) => (4, 1),
            Self::Flash(g) => (g.n, g.n),
            Self::Mirror(g) => (g.n, g.n),
            Self::Balance(g) => (g.weights.len() / 2, 2),
            Self::Hanoi(_) => (3, 1),
            Self::Lights(g) => (g.n, g.n),
            Self::Detective(g) => (g.cols, g.rows),
            _ => (1, 1),
        }
    }

    pub fn countdown(&self) -> u32 {
        match self {
            Self::Flash(g) => g.preview.ceil() as u32,
            Self::Reaction(g) => g.remaining.ceil() as u32,
            _ => 0,
        }
    }

    pub fn tick(&mut self, dt: f32) -> Outcome {
        match self {
            Self::Flash(g) if g.preview > 0.0 => {
                g.preview = (g.preview - dt).max(0.0);
                if g.preview == 0.0 {
                    Outcome::Changed
                } else {
                    Outcome::Quiet
                }
            }
            Self::Reaction(g) => {
                g.remaining = (g.remaining - dt).max(0.0);
                if g.remaining > 0.0 {
                    Outcome::Quiet
                } else if g.should_press() {
                    Outcome::Mistake("绿灯要及时按下，下次再试")
                } else {
                    Outcome::Solved
                }
            }
            Self::Coding(g) if g.running => {
                g.clock -= dt;
                if g.clock > 0.0 {
                    return Outcome::Quiet;
                }
                g.clock = 0.35;
                g.advance()
            }
            _ => Outcome::Quiet,
        }
    }

    pub fn act(&mut self, action: Action, cursor: &mut usize) -> Outcome {
        match action {
            Action::Move(dx, dy) => {
                if let Self::Coding(g) = self {
                    return g.append(if dx > 0 {
                        Direction::Right
                    } else if dx < 0 {
                        Direction::Left
                    } else if dy > 0 {
                        Direction::Down
                    } else {
                        Direction::Up
                    });
                }
                if let Self::Rotation(g) = self {
                    for _ in 0..if dx < 0 { 3 } else { 1 } {
                        g.current = rotate(&g.current, g.n);
                    }
                    return Outcome::Changed;
                }
                let (cols, rows) = self.dimensions();
                let x = (*cursor % cols) as i32;
                let y = (*cursor / cols) as i32;
                *cursor = (y + dy).clamp(0, rows as i32 - 1) as usize * cols
                    + (x + dx).clamp(0, cols as i32 - 1) as usize;
                return Outcome::Changed;
            }
            Action::Pick(index) => {
                if let Self::Coding(g) = self {
                    return Direction::ALL
                        .get(index)
                        .map_or(Outcome::Quiet, |&dir| g.append(dir));
                }
                if matches!(self, Self::Rotation(_)) {
                    return self.act(Action::Move(if index == 0 { -1 } else { 1 }, 0), cursor);
                }
                let (cols, rows) = self.dimensions();
                if index >= cols * rows {
                    return Outcome::Quiet;
                }
                *cursor = index;
            }
            _ => {}
        }
        let primary = action != Action::Secondary;
        match self {
            Self::Pattern(g) if primary => {
                if g.options[*cursor] == g.answer {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("再观察一下相邻几项的变化")
                }
            }
            Self::Flash(g) => {
                if g.preview > 0.0 {
                    return Outcome::Quiet;
                }
                if primary {
                    g.marked[*cursor] = !g.marked[*cursor];
                    Outcome::Changed
                } else if g.marked == g.target {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("还有格子不一致，可以调整后再提交")
                }
            }
            Self::Mirror(g) => {
                if primary {
                    g.marked[*cursor] = !g.marked[*cursor];
                    Outcome::Changed
                } else if g.marked == reflect(&g.source, g.n) {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("留意每个格子离中间镜子的距离")
                }
            }
            Self::Reaction(g) if primary => {
                if g.should_press() {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("这次要等待，不用按键")
                }
            }
            Self::Balance(g) => {
                if primary {
                    g.selected[*cursor] = !g.selected[*cursor];
                    Outcome::Changed
                } else if g.sum() == g.target {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("两边还不一样，调整砝码后再提交")
                }
            }
            Self::Hanoi(g) => {
                if primary {
                    g.pick(*cursor)
                } else {
                    g.undo();
                    Outcome::Changed
                }
            }
            Self::Lights(g) => {
                if primary {
                    toggle_cross(&mut g.on, g.n, *cursor);
                    g.history.push(*cursor);
                } else if let Some(at) = g.history.pop() {
                    toggle_cross(&mut g.on, g.n, at);
                }
                if g.on.iter().all(|on| !on) {
                    Outcome::Solved
                } else {
                    Outcome::Changed
                }
            }
            Self::Coding(g) => {
                if g.running {
                    return Outcome::Quiet;
                }
                if primary {
                    if g.program.is_empty() {
                        return Outcome::Mistake("先添加方向指令，再运行机器人");
                    }
                    g.robot = 0;
                    g.step = 0;
                    g.running = true;
                    g.clock = 0.35;
                } else {
                    g.program.pop();
                    g.robot = 0;
                }
                Outcome::Changed
            }
            Self::Rotation(g) if primary => {
                if g.current == g.target {
                    Outcome::Solved
                } else {
                    Outcome::Mistake("方向还没对齐，左右旋转后再提交")
                }
            }
            Self::Detective(g) if primary => {
                if g.found[*cursor] {
                    return Outcome::Quiet;
                }
                if g.items[*cursor] != g.target {
                    return Outcome::Mistake("颜色和形状都要与目标相同");
                }
                g.found[*cursor] = true;
                if g.items
                    .iter()
                    .zip(&g.found)
                    .all(|(item, &found)| *item != g.target || found)
                {
                    Outcome::Solved
                } else {
                    Outcome::Changed
                }
            }
            _ => Outcome::Quiet,
        }
    }
}

pub(super) fn rounds(kind: GameKind, level: u8) -> u8 {
    match kind {
        GameKind::Pattern => 4 + level / 3,
        GameKind::FlashGrid => 2 + level / 5,
        GameKind::GoNoGo => 8 + level / 2,
        GameKind::Balance | GameKind::Rotation => 3,
        _ => 1,
    }
}
