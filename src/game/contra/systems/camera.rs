use bevy::prelude::*;

use crate::common::constants::ARENA_W;

use super::super::components::*;
use super::super::constants::CAMERA_FOLLOW_OFFSET;
use super::super::resources::ContraStage;

pub fn contra_camera_follow(
    stage: Res<ContraStage>,
    player_q: Query<
        &Transform,
        (With<ContraPlayer>, Without<Camera>, Without<ContraBackground>),
    >,
    mut cam_q: Query<
        &mut Transform,
        (With<Camera>, Without<ContraPlayer>, Without<ContraBackground>),
    >,
    mut bg_q: Query<
        &mut Transform,
        (With<ContraBackground>, Without<Camera>, Without<ContraPlayer>),
    >,
) {
    let Ok(ptr) = player_q.single() else { return };
    let Ok(mut ctr) = cam_q.single_mut() else {
        return;
    };
    let cam_min = ARENA_W * 0.5;
    let cam_max = stage.world_w - ARENA_W * 0.5;
    let target = (ptr.translation.x + CAMERA_FOLLOW_OFFSET).clamp(cam_min, cam_max);
    if target > ctr.translation.x {
        ctr.translation.x = target;
    } else if ctr.translation.x < cam_min {
        ctr.translation.x = cam_min;
    }
    ctr.translation.y = 0.0;
    // HUD 挂在相机子节点上自动跟随，这里只需同步背景
    for mut btr in &mut bg_q {
        btr.translation.x = ctr.translation.x;
        btr.translation.y = ctr.translation.y;
    }
}
