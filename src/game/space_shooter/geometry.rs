use bevy::math::Vec2;

pub fn aabb(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    (a.x - b.x).abs() * 2.0 < a_size.x + b_size.x
        && (a.y - b.y).abs() * 2.0 < a_size.y + b_size.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_basic() {
        let s = Vec2::splat(10.0);
        assert!(aabb(Vec2::ZERO, s, Vec2::new(5.0, 0.0), s));
        assert!(!aabb(Vec2::ZERO, s, Vec2::new(20.0, 0.0), s));
        assert!(!aabb(Vec2::ZERO, s, Vec2::new(10.0, 0.0), s));
    }

    #[test]
    fn aabb_handles_asymmetric_sizes() {
        let big = Vec2::new(40.0, 10.0);
        let small = Vec2::new(6.0, 6.0);
        assert!(aabb(Vec2::ZERO, big, Vec2::new(15.0, 0.0), small));
        assert!(!aabb(Vec2::ZERO, big, Vec2::new(40.0, 0.0), small));
    }
}
