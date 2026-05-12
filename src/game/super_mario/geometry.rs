use bevy::math::Vec2;

use super::constants::{FLOOR_Y, LEVEL_COLS, TILE};

pub fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

pub fn tile_center(col: i32, row_from_bottom: i32) -> Vec2 {
    Vec2::new(
        col as f32 * TILE + TILE * 0.5,
        FLOOR_Y + row_from_bottom as f32 * TILE,
    )
}

pub fn level_cols() -> i32 {
    LEVEL_COLS
}

pub fn level_world_max_x() -> f32 {
    level_cols() as f32 * TILE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_basic() {
        let s = Vec2::splat(10.0);
        assert!(aabb_overlap(Vec2::ZERO, s, Vec2::new(5.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(20.0, 0.0), s));
    }

    #[test]
    fn tile_center_first_col_zero_row() {
        let c = tile_center(0, 0);
        assert_eq!(c.x, TILE * 0.5);
        assert_eq!(c.y, FLOOR_Y);
    }

    #[test]
    fn world_max_x_matches_cols() {
        assert_eq!(level_world_max_x(), level_cols() as f32 * TILE);
    }
}
