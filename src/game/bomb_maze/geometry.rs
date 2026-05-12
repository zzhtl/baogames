// 炸弹迷宫的纯几何与网格工具。

use bevy::math::{Vec2, Vec3};

use super::components::Dir4;
use super::constants::{
    BM_COLS, BM_OFFSET_X, BM_OFFSET_Y, BM_PLAY_H, BM_PLAY_W, BM_ROWS, BM_TILE, P1_SPAWN, P2_SPAWN,
};

pub fn tile_center(col: i32, row: i32) -> Vec2 {
    Vec2::new(
        BM_OFFSET_X - BM_PLAY_W * 0.5 + col as f32 * BM_TILE + BM_TILE * 0.5,
        BM_OFFSET_Y + BM_PLAY_H * 0.5 - row as f32 * BM_TILE - BM_TILE * 0.5,
    )
}

pub fn world_to_tile(pos: Vec2) -> (i32, i32) {
    let local_x = pos.x - (BM_OFFSET_X - BM_PLAY_W * 0.5);
    let local_y = (BM_OFFSET_Y + BM_PLAY_H * 0.5) - pos.y;
    let col = (local_x / BM_TILE).floor() as i32;
    let row = (local_y / BM_TILE).floor() as i32;
    (col.clamp(0, BM_COLS - 1), row.clamp(0, BM_ROWS - 1))
}

pub fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

pub fn in_safe_zone(col: i32, row: i32) -> bool {
    // 玩家出生附近 L 形 3 格 留空，避免一开局被困
    let zones = [
        (P1_SPAWN.0, P1_SPAWN.1),
        (P1_SPAWN.0 + 1, P1_SPAWN.1),
        (P1_SPAWN.0, P1_SPAWN.1 + 1),
        (P2_SPAWN.0, P2_SPAWN.1),
        (P2_SPAWN.0 + 1, P2_SPAWN.1),
        (P2_SPAWN.0, P2_SPAWN.1 - 1),
    ];
    zones.iter().any(|(c, r)| *c == col && *r == row)
}

pub fn is_pillar(col: i32, row: i32) -> bool {
    col % 2 == 0 && row % 2 == 0
}

pub fn is_border(col: i32, row: i32) -> bool {
    col == 0 || col == BM_COLS - 1 || row == 0 || row == BM_ROWS - 1
}

/// 移动方向与轴正交的轴吸附到 tile 中线（用于敌人变向时）。
pub fn snap_perpendicular(t: &mut Vec3, dir: Dir4) {
    match dir {
        Dir4::Up | Dir4::Down => {
            let (col, _) = world_to_tile(Vec2::new(t.x, t.y));
            t.x = tile_center(col, 0).x;
        }
        Dir4::Left | Dir4::Right => {
            let (_, row) = world_to_tile(Vec2::new(t.x, t.y));
            t.y = tile_center(0, row).y;
        }
    }
}

/// 在一组点中找到离 `from` 最近的那个。`points` 必须非空。
pub fn nearest_point(points: &[Vec2], from: Vec2) -> Vec2 {
    let mut best = points[0];
    let mut best_d = best.distance_squared(from);
    for p in &points[1..] {
        let d = p.distance_squared(from);
        if d < best_d {
            best_d = d;
            best = *p;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_center_roundtrip_via_world_to_tile() {
        for c in 0..BM_COLS {
            for r in 0..BM_ROWS {
                let p = tile_center(c, r);
                assert_eq!(world_to_tile(p), (c, r), "tile ({},{}) roundtrip", c, r);
            }
        }
    }

    #[test]
    fn world_to_tile_clamps_outside_field() {
        let far_left = Vec2::new(-10000.0, 0.0);
        let (c, _) = world_to_tile(far_left);
        assert_eq!(c, 0);
        let far_right = Vec2::new(10000.0, 0.0);
        let (c, _) = world_to_tile(far_right);
        assert_eq!(c, BM_COLS - 1);
    }

    #[test]
    fn aabb_overlap_basic() {
        let s = Vec2::splat(10.0);
        assert!(aabb_overlap(Vec2::ZERO, s, Vec2::new(5.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(20.0, 0.0), s));
        // 相切不算撞
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(10.0, 0.0), s));
    }

    #[test]
    fn is_border_marks_outer_rows_and_cols() {
        assert!(is_border(0, 5));
        assert!(is_border(BM_COLS - 1, 5));
        assert!(is_border(7, 0));
        assert!(is_border(7, BM_ROWS - 1));
        assert!(!is_border(1, 1));
    }

    #[test]
    fn is_pillar_marks_even_intersections() {
        assert!(is_pillar(2, 2));
        assert!(is_pillar(4, 6));
        assert!(!is_pillar(1, 2));
        assert!(!is_pillar(2, 3));
    }

    #[test]
    fn safe_zone_covers_spawn_neighbourhoods() {
        assert!(in_safe_zone(P1_SPAWN.0, P1_SPAWN.1));
        assert!(in_safe_zone(P1_SPAWN.0 + 1, P1_SPAWN.1));
        assert!(in_safe_zone(P2_SPAWN.0, P2_SPAWN.1));
        assert!(!in_safe_zone(7, 5));
    }

    #[test]
    fn nearest_point_picks_closest() {
        let pts = [Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), Vec2::new(5.0, 5.0)];
        assert_eq!(nearest_point(&pts, Vec2::new(11.0, 0.0)), Vec2::new(10.0, 0.0));
        assert_eq!(nearest_point(&pts, Vec2::new(0.0, 0.0)), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn snap_perpendicular_aligns_to_grid() {
        let center = tile_center(3, 4);
        let mut p = Vec3::new(center.x + 3.0, center.y + 7.0, 0.0);
        snap_perpendicular(&mut p, Dir4::Up);
        // 朝上下走时，x 应该被吸附；y 不动
        assert!((p.x - center.x).abs() < 1e-3);
        assert!((p.y - (center.y + 7.0)).abs() < 1e-3);
    }
}
