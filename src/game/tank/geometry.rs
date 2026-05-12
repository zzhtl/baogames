// 坦克大战所有「纯几何」工具：坐标转换、AABB 碰撞、转向时的轴吸附。
//
// 这些函数没有任何 ECS 副作用，所以单独成文件并配单元测试。

use bevy::math::{Vec2, Vec3};

use super::components::TankDir;
use super::constants::{PLAY_OFFSET_X, PLAY_OFFSET_Y, PLAY_SIZE, SUBTILE, TILE};

pub fn tile_center(col: i32, row: i32) -> Vec2 {
    // row 0 在顶部，row 12 在底部。tile 是 2 个子格 = 32px。
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5 + col as f32 * 32.0 + 16.0,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - row as f32 * 32.0 - 16.0,
    )
}

pub fn subtile_center(sx: i32, sy: i32) -> Vec2 {
    // sy 0 在顶部，sy 25 在底部。
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5 + sx as f32 * SUBTILE + SUBTILE * 0.5,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - sy as f32 * SUBTILE - SUBTILE * 0.5,
    )
}

pub fn play_min() -> Vec2 {
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5,
        PLAY_OFFSET_Y - PLAY_SIZE * 0.5,
    )
}

pub fn play_max() -> Vec2 {
    Vec2::new(
        PLAY_OFFSET_X + PLAY_SIZE * 0.5,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5,
    )
}

pub fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

/// 转向时把与移动方向垂直的轴吸附到最近的 tile 中线，
/// 避免穿越宽度恰为一个 tile 的巷道时因半格偏移而卡住。
pub fn snap_perpendicular(t: &mut Vec3, dir: TankDir) {
    let origin = play_min();
    match dir {
        TankDir::Up | TankDir::Down => {
            let rel = t.x - origin.x - TILE * 0.5;
            let snapped = (rel / TILE).round() * TILE + TILE * 0.5 + origin.x;
            t.x = snapped;
        }
        TankDir::Left | TankDir::Right => {
            let rel = t.y - origin.y - TILE * 0.5;
            let snapped = (rel / TILE).round() * TILE + TILE * 0.5 + origin.y;
            t.y = snapped;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_min_max_form_expected_box() {
        let mn = play_min();
        let mx = play_max();
        assert!((mx.x - mn.x - PLAY_SIZE).abs() < 1e-3);
        assert!((mx.y - mn.y - PLAY_SIZE).abs() < 1e-3);
    }

    #[test]
    fn tile_center_top_left_is_inside_play_area_and_offset_by_half_tile() {
        let mn = play_min();
        let mx = play_max();
        let c = tile_center(0, 0);
        // tile_center(0,0) 应位于左上格中心：min.x + 16, max.y - 16
        assert!((c.x - (mn.x + 16.0)).abs() < 1e-3);
        assert!((c.y - (mx.y - 16.0)).abs() < 1e-3);
    }

    #[test]
    fn subtile_center_grid_lines_up() {
        // 子格 0,0 应是 tile 0,0 中心向左下偏 8,8
        let t = tile_center(0, 0);
        let s = subtile_center(0, 0);
        assert!((s.x - (t.x - 8.0)).abs() < 1e-3);
        assert!((s.y - (t.y + 8.0)).abs() < 1e-3);
    }

    #[test]
    fn aabb_overlap_disjoint_returns_false() {
        let a = Vec2::ZERO;
        let b = Vec2::new(20.0, 0.0);
        let size = Vec2::splat(10.0);
        assert!(!aabb_overlap(a, size, b, size));
    }

    #[test]
    fn aabb_overlap_touching_edges_does_not_count() {
        // 严格 < ：刚好接触不算碰撞
        let a = Vec2::ZERO;
        let b = Vec2::new(10.0, 0.0);
        let size = Vec2::splat(10.0);
        assert!(!aabb_overlap(a, size, b, size));
    }

    #[test]
    fn aabb_overlap_intersecting_returns_true() {
        let a = Vec2::ZERO;
        let b = Vec2::new(5.0, 0.0);
        let size = Vec2::splat(10.0);
        assert!(aabb_overlap(a, size, b, size));
    }

    #[test]
    fn snap_perpendicular_horizontal_aligns_x() {
        // 朝上下走时，吸附的是 x 轴
        let mut p = Vec3::new(tile_center(3, 0).x + 4.0, 0.0, 0.0);
        snap_perpendicular(&mut p, TankDir::Up);
        let expected = tile_center(3, 0).x;
        assert!((p.x - expected).abs() < 1e-3);
    }

    #[test]
    fn snap_perpendicular_vertical_aligns_y() {
        let mut p = Vec3::new(0.0, tile_center(0, 5).y + 4.0, 0.0);
        snap_perpendicular(&mut p, TankDir::Right);
        let expected = tile_center(0, 5).y;
        assert!((p.y - expected).abs() < 1e-3);
    }
}
