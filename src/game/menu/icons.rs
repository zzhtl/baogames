use super::view::*;
use super::super::model::GameKind;
use bevy::prelude::*;

pub(super) fn cover(view: &mut View, kind: GameKind, center: Vec2, scale: f32) {
    let accent = kind.accent();
    let dark = Color::srgb(0.10, 0.12, 0.16);
    let light = Color::srgb(0.96, 0.93, 0.85);
    let parts: &[(f32, f32, f32, f32, Color)] = match kind {
        GameKind::Tank => &[
            (0.0, -2.0, 11.0, 7.0, accent),
            (0.0, 3.0, 3.0, 6.0, accent),
            (-6.0, -1.0, 2.0, 11.0, dark),
            (6.0, -1.0, 2.0, 11.0, dark),
        ],
        GameKind::BombMaze => &[
            (0.0, -1.0, 11.0, 11.0, accent),
            (2.0, 6.0, 2.0, 4.0, dark),
            (4.0, 8.0, 3.0, 2.0, Color::srgb(1.0, 0.85, 0.3)),
        ],
        GameKind::SpaceShooter => &[
            (0.0, 1.0, 4.0, 13.0, accent),
            (0.0, -1.0, 14.0, 4.0, accent),
            (0.0, -7.0, 3.0, 3.0, Color::srgb(1.0, 0.72, 0.2)),
        ],
        GameKind::SuperMario => &[
            (0.0, 5.0, 10.0, 3.0, accent),
            (0.0, 1.0, 8.0, 5.0, Color::srgb(0.96, 0.72, 0.45)),
            (-2.0, -5.0, 3.0, 6.0, Color::srgb(0.24, 0.42, 0.9)),
            (2.0, -5.0, 3.0, 6.0, Color::srgb(0.24, 0.42, 0.9)),
        ],
        GameKind::Contra => &[
            (-2.0, 2.0, 6.0, 10.0, accent),
            (4.0, 3.0, 8.0, 2.0, Color::srgb(0.74, 0.78, 0.88)),
            (-4.0, -6.0, 3.0, 5.0, dark),
            (0.0, -6.0, 3.0, 5.0, dark),
        ],
        GameKind::BubbleBobble => &[
            (-4.0, 3.0, 7.0, 7.0, accent),
            (4.0, 3.0, 7.0, 7.0, Color::srgb(0.35, 0.8, 1.0)),
            (0.0, -4.0, 7.0, 7.0, Color::srgb(1.0, 0.75, 0.25)),
        ],
        GameKind::MemoryMatch => &[
            (-4.0, 0.0, 7.0, 11.0, accent),
            (4.0, 0.0, 7.0, 11.0, Color::srgb(0.9, 0.45, 0.55)),
            (-4.0, 0.0, 3.0, 5.0, light),
        ],
        GameKind::Sokoban => &[
            (0.0, 0.0, 13.0, 13.0, Color::srgb(0.55, 0.30, 0.12)),
            (0.0, 0.0, 9.0, 9.0, accent),
            (0.0, 0.0, 3.0, 3.0, Color::srgb(1.0, 0.93, 0.62)),
        ],
        // 3×3 方格中挑两格点亮，示意「按顺序点格子」
        GameKind::Schulte => &[
            (0.0, 0.0, 13.0, 13.0, Color::srgb(0.13, 0.20, 0.19)),
            (-4.0, 4.0, 3.0, 3.0, accent),
            (0.0, 0.0, 3.0, 3.0, accent),
            (4.0, -4.0, 3.0, 3.0, light),
        ],
        // 一个"字"压在三色条上，示意字义与墨色打架
        GameKind::Stroop => &[
            (0.0, 4.0, 13.0, 5.0, Color::srgb(0.98, 0.29, 0.29)),
            (0.0, -2.0, 13.0, 5.0, Color::srgb(0.31, 0.60, 1.00)),
            (0.0, -8.0, 13.0, 5.0, accent),
            (0.0, 0.0, 3.0, 11.0, dark),
        ],
        // 九宫格：一条竖宫线 + 一条横宫线
        GameKind::Sudoku => &[
            (0.0, 0.0, 13.0, 13.0, Color::srgb(0.11, 0.13, 0.22)),
            (-2.0, 0.0, 1.0, 13.0, accent),
            (2.0, 0.0, 1.0, 13.0, accent),
            (0.0, 0.0, 13.0, 1.0, accent),
            (-4.0, 4.0, 3.0, 3.0, light),
        ],
        // 3×3 滑块少一块，右下角空着
        GameKind::Sliding => &[
            (0.0, 0.0, 13.0, 13.0, dark),
            (-4.0, 4.0, 3.0, 3.0, accent),
            (0.0, 4.0, 3.0, 3.0, accent),
            (4.0, 4.0, 3.0, 3.0, accent),
            (-4.0, 0.0, 3.0, 3.0, accent),
            (0.0, 0.0, 3.0, 3.0, accent),
            (4.0, 0.0, 3.0, 3.0, accent),
            (-4.0, -4.0, 3.0, 3.0, accent),
            (0.0, -4.0, 3.0, 3.0, light),
        ],
        // 一条折返的通道 + 右下角的出口
        GameKind::MazeRun => &[
            (0.0, 0.0, 13.0, 13.0, Color::srgb(0.16, 0.27, 0.20)),
            (-3.0, 3.0, 7.0, 2.0, dark),
            (3.0, 0.0, 2.0, 8.0, dark),
            (-1.0, -3.0, 9.0, 2.0, dark),
            (4.0, -4.0, 3.0, 3.0, accent),
        ],
        // 两张同图案的牌，中间一条折线连起来
        GameKind::LinkUp => &[
            (-4.0, 4.0, 5.0, 5.0, accent),
            (4.0, -4.0, 5.0, 5.0, accent),
            (-4.0, -1.0, 1.0, 5.0, light),
            (0.0, -4.0, 9.0, 1.0, light),
        ],
        // 十字排列的四个音板，上面那个亮着
        GameKind::Simon => &[
            (0.0, 5.0, 6.0, 5.0, accent),
            (0.0, -5.0, 6.0, 5.0, Color::srgb(0.11, 0.24, 0.48)),
            (-5.0, 0.0, 5.0, 6.0, Color::srgb(0.44, 0.12, 0.14)),
            (5.0, 0.0, 5.0, 6.0, Color::srgb(0.45, 0.36, 0.07)),
        ],
        // 左右两张小图，右边那张少一块
        GameKind::SpotDiff => &[
            (-4.0, 0.0, 8.0, 12.0, Color::srgb(0.13, 0.15, 0.20)),
            (4.0, 0.0, 8.0, 12.0, Color::srgb(0.13, 0.15, 0.20)),
            (-4.0, 3.0, 4.0, 4.0, accent),
            (-4.0, -3.0, 4.0, 4.0, light),
            (4.0, 3.0, 4.0, 4.0, accent),
        ],
        GameKind::Pattern => &[
            (-6.0, 0.0, 3.0, 5.0, accent), (0.0, 0.0, 3.0, 9.0, accent),
            (6.0, 0.0, 3.0, 13.0, light),
        ],
        GameKind::FlashGrid => &[
            (-4.0, 4.0, 6.0, 6.0, accent), (4.0, 4.0, 6.0, 6.0, dark),
            (-4.0, -4.0, 6.0, 6.0, dark), (4.0, -4.0, 6.0, 6.0, light),
        ],
        GameKind::Mirror => &[
            (0.0, 0.0, 1.0, 16.0, light), (-5.0, 3.0, 5.0, 5.0, accent),
            (5.0, 3.0, 5.0, 5.0, accent), (-3.0, -3.0, 2.0, 4.0, accent), (3.0, -3.0, 2.0, 4.0, accent),
        ],
        GameKind::GoNoGo => &[
            (0.0, 0.0, 8.0, 17.0, dark), (0.0, 5.0, 5.0, 5.0, Color::srgb(0.9, 0.3, 0.3)),
            (0.0, -5.0, 5.0, 5.0, accent),
        ],
        GameKind::Balance => &[
            (0.0, -2.0, 2.0, 12.0, light), (0.0, 4.0, 16.0, 2.0, accent),
            (-6.0, 0.0, 5.0, 4.0, accent), (6.0, 0.0, 5.0, 4.0, accent), (0.0, -7.0, 8.0, 2.0, light),
        ],
        GameKind::Hanoi => &[
            (0.0, 0.0, 2.0, 17.0, light), (0.0, -5.0, 16.0, 3.0, accent),
            (0.0, -1.0, 12.0, 3.0, Color::srgb(0.5, 0.7, 0.95)), (0.0, 3.0, 7.0, 3.0, accent),
        ],
        GameKind::LightsOut => &[
            (0.0, 1.0, 10.0, 10.0, accent), (0.0, -6.0, 5.0, 3.0, light),
            (-7.0, 6.0, 2.0, 3.0, accent), (7.0, 6.0, 2.0, 3.0, accent),
        ],
        GameKind::Coding => &[
            (-4.0, -5.0, 6.0, 6.0, accent), (-4.0, 1.0, 2.0, 7.0, light),
            (1.0, 4.0, 12.0, 2.0, light), (6.0, 4.0, 3.0, 6.0, accent),
        ],
        GameKind::Rotation => &[
            (-3.0, 1.0, 4.0, 12.0, accent), (1.0, -3.0, 12.0, 4.0, accent),
            (5.0, 5.0, 6.0, 2.0, light), (7.0, 3.0, 2.0, 4.0, light),
        ],
        GameKind::Detective => &[
            (-2.0, 2.0, 12.0, 12.0, accent), (-2.0, 2.0, 7.0, 7.0, dark),
            (5.0, -5.0, 5.0, 5.0, light),
        ],
    };
    for (index, (dx, dy, w, h, color)) in parts.iter().copied().enumerate() {
        view.block(center + Vec2::new(dx, -dy) * scale, Vec2::new(w, h) * scale, color, 4.0 + index as f32 * 0.01);
    }
}
