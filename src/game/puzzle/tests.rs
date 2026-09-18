use super::grid::{BOARD_H, BOARD_W, GridCursor, GridLayout, step_clamped};
use crate::common::px::px;
use crate::common::settings::DisplayMode;

/// 棋盘在 4:3（可见 720 世界单位）下不能出屏 —— 这正是把宽度上界从推箱子的
/// 880 收到 660 的原因。
#[test]
fn every_grid_shape_stays_on_the_narrow_canvas() {
    let visible_w = DisplayMode::Classic4x3.world_width();
    for cols in 1..=24 {
        for rows in 1..=20 {
            let layout = GridLayout::fit(cols, rows);
            let size = layout.total_size();
            assert!(
                size.x <= visible_w - px(8.0),
                "{cols}x{rows} 宽 {} 超出 4:3 可见宽度",
                size.x
            );
            assert!(size.x <= BOARD_W + 0.01 && size.y <= BOARD_H + 0.01);
        }
    }
}

#[test]
fn cell_centers_span_the_board_symmetrically() {
    let layout = GridLayout::fit(5, 5);
    let first = layout.cell_center(0, 0);
    let last = layout.cell_center(4, 4);
    assert!((first.x + last.x).abs() < 0.001, "水平方向应左右对称");
    // row 向下 → y 递减
    assert!(last.y < first.y);
}

#[test]
fn index_rejects_out_of_range_cells() {
    let layout = GridLayout::fit(3, 4);
    assert_eq!(layout.index(2, 3), Some(11));
    assert_eq!(layout.index(3, 0), None);
    assert_eq!(layout.index(-1, 0), None);
    assert_eq!(layout.cell_count(), 12);
}

#[test]
fn step_clamps_or_wraps_at_the_border() {
    assert_eq!(step_clamped(0, 0, (-1, 0), 4, 4, false), (0, 0));
    assert_eq!(step_clamped(0, 0, (-1, 0), 4, 4, true), (3, 0));
    assert_eq!(step_clamped(3, 3, (1, 1), 4, 4, true), (0, 0));
    assert_eq!(step_clamped(3, 3, (1, 1), 4, 4, false), (3, 3));
}

#[test]
fn cursor_clamps_into_a_shrinking_grid() {
    let mut cursor = GridCursor::new(7, 9);
    cursor.clamp_into(4, 4);
    assert_eq!((cursor.col, cursor.row), (3, 3));
}

/// 格宽必须是偶数个画布像素，否则格心落在半像素上、描边会时粗时细。
#[test]
fn cell_size_and_origin_land_on_whole_canvas_pixels() {
    use crate::common::px::WORLD_PER_PX;
    for cols in 1..=24 {
        for rows in 1..=20 {
            let layout = GridLayout::fit(cols, rows);
            let px_size = layout.cell_size / WORLD_PER_PX;
            assert_eq!(px_size, px_size.round(), "{cols}x{rows} 格宽不是整像素");
            assert_eq!(px_size % 2.0, 0.0, "{cols}x{rows} 格宽不是偶数像素");
            for (value, axis) in [(layout.origin.x, "x"), (layout.origin.y + 20.0, "y")] {
                let in_px = value / WORLD_PER_PX;
                assert_eq!(in_px, in_px.round(), "{cols}x{rows} 的 origin.{axis} 落在半像素上");
            }
        }
    }
}
