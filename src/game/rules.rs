use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

pub(super) fn aabb(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) * 0.5
        && (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) * 0.5
}

pub(super) fn circle_aabb(circle: Vec2, radius: f32, rect_pos: Vec2, rect_size: Vec2) -> bool {
    let half = rect_size * 0.5;
    let closest = Vec2::new(
        circle.x.clamp(rect_pos.x - half.x, rect_pos.x + half.x),
        circle.y.clamp(rect_pos.y - half.y, rect_pos.y + half.y),
    );
    circle.distance_squared(closest) <= radius * radius
}

pub(super) fn bomb_tiles(origin: (i32, i32), range: i32) -> Vec<(i32, i32)> {
    let mut tiles = vec![origin];
    for i in 1..=range {
        tiles.push((origin.0 + i, origin.1));
        tiles.push((origin.0 - i, origin.1));
        tiles.push((origin.0, origin.1 + i));
        tiles.push((origin.0, origin.1 - i));
    }
    tiles
}

pub(super) fn bubble_cluster(
    start: Entity,
    color_id: usize,
    bubbles: &[(Entity, Vec2, usize)],
) -> Vec<Entity> {
    let mut found = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(entity) = queue.pop_front() {
        if !visited.insert(entity) {
            continue;
        }
        let Some((_, pos, color)) = bubbles.iter().find(|(e, _, _)| *e == entity) else {
            continue;
        };
        if *color != color_id {
            continue;
        }
        found.push(entity);
        for (next, next_pos, next_color) in bubbles {
            if *next_color == color_id && !visited.contains(next) && pos.distance(*next_pos) < 70.0
            {
                queue.push_back(*next);
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bomb_range_is_cross_shaped() {
        let tiles = bomb_tiles((2, 3), 2);
        assert_eq!(tiles.len(), 9);
        assert!(tiles.contains(&(2, 3)));
        assert!(tiles.contains(&(4, 3)));
        assert!(tiles.contains(&(2, 1)));
        assert!(!tiles.contains(&(3, 4)));
    }

    #[test]
    fn circle_aabb_hits_near_edge() {
        assert!(circle_aabb(
            Vec2::new(12.0, 0.0),
            4.0,
            Vec2::ZERO,
            Vec2::new(20.0, 20.0)
        ));
        assert!(!circle_aabb(
            Vec2::new(30.0, 0.0),
            4.0,
            Vec2::ZERO,
            Vec2::new(20.0, 20.0)
        ));
    }
}
