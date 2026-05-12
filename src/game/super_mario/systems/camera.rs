use bevy::prelude::*;

use crate::common::constants::ARENA_W;

use super::super::components::*;
use super::super::constants::CAMERA_FOLLOW_OFFSET;
use super::super::geometry::level_world_max_x;

pub fn mario_camera_follow(
    player_q: Query<&Transform, (With<MarioPlayer>, Without<Camera>, Without<MarioBackground>)>,
    mut cam_q: Query<&mut Transform, (With<Camera>, Without<MarioPlayer>, Without<MarioBackground>)>,
    mut bg_q: Query<&mut Transform, (With<MarioBackground>, Without<Camera>, Without<MarioPlayer>)>,
) {
    let Ok(ptr) = player_q.single() else { return };
    let Ok(mut ctr) = cam_q.single_mut() else {
        return;
    };
    let cam_min = ARENA_W * 0.5;
    let cam_max = level_world_max_x() - ARENA_W * 0.5;
    let target = (ptr.translation.x + CAMERA_FOLLOW_OFFSET).clamp(cam_min, cam_max);
    if target > ctr.translation.x {
        ctr.translation.x = target;
    } else if ctr.translation.x < cam_min {
        ctr.translation.x = cam_min;
    }
    ctr.translation.y = 0.0;
    for mut btr in &mut bg_q {
        btr.translation.x = ctr.translation.x;
        btr.translation.y = ctr.translation.y;
    }
}
