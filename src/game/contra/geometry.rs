use bevy::math::Vec2;

use super::components::AimDir;
use super::constants::{PLAYER_H, PLAYER_W, PRONE_H};

pub fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

pub fn player_size(prone: bool) -> Vec2 {
    Vec2::new(PLAYER_W, if prone { PRONE_H } else { PLAYER_H })
}

pub fn muzzle_offset(prone: bool, aim: AimDir, facing: f32) -> Vec2 {
    let body_h = if prone { PRONE_H } else { PLAYER_H };
    let mid_y = if prone { 0.0 } else { 4.0 };
    let dir = aim.vec();
    let half = body_h * 0.5;
    Vec2::new(
        dir.x * (PLAYER_W * 0.5 + 8.0) + if dir.x.abs() < 0.01 { facing * 2.0 } else { 0.0 },
        dir.y * (half + 6.0) + mid_y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_basic() {
        let s = Vec2::splat(10.0);
        assert!(aabb_overlap(Vec2::ZERO, s, Vec2::new(5.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(20.0, 0.0), s));
        assert!(!aabb_overlap(Vec2::ZERO, s, Vec2::new(10.0, 0.0), s));
    }

    #[test]
    fn player_size_shrinks_when_prone() {
        let stand = player_size(false);
        let prone = player_size(true);
        assert_eq!(stand.x, prone.x);
        assert!(prone.y < stand.y);
    }

    #[test]
    fn muzzle_extends_along_aim() {
        // 朝右站立：muzzle 应在身体右侧外
        let m = muzzle_offset(false, AimDir::Right, 1.0);
        assert!(m.x > PLAYER_W * 0.5);
        // 朝上：muzzle x 应几乎为 0，y 在身体上方
        let m = muzzle_offset(false, AimDir::Up, 1.0);
        assert!(m.x.abs() < 5.0);
        assert!(m.y > PLAYER_H * 0.5);
    }
}
