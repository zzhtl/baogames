use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::grid::aim_dir;
use super::super::resources::{BubbleAssets, BubbleStage};
use super::super::setup::spawn_flying_bubble;

pub fn bubble_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    assets: Res<BubbleAssets>,
    mut stage: ResMut<BubbleStage>,
    mut barrel_q: Query<&mut Transform, (With<CannonBarrel>, Without<LoadedBubble>)>,
    mut loaded_q: Query<
        (Entity, &mut Transform),
        (With<LoadedBubble>, Without<CannonBarrel>),
    >,
) {
    if session.kind != GameKind::BubbleBobble || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    if stage.message_clock > 0.0 {
        stage.message_clock = (stage.message_clock - dt).max(0.0);
    }
    if stage.flash_clock > 0.0 {
        stage.flash_clock = (stage.flash_clock - dt).max(0.0);
    }
    let mut aim = stage.aim;
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        aim -= AIM_SPEED * dt;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        aim += AIM_SPEED * dt;
    }
    aim = aim.clamp(-MAX_AIM, MAX_AIM);
    stage.aim = aim;

    let dir = aim_dir(aim);
    if let Ok(mut t) = barrel_q.single_mut() {
        let center = Vec2::new(CANNON_X, CANNON_Y) + dir * 22.0;
        t.translation.x = center.x;
        t.translation.y = center.y;
        t.rotation = Quat::from_rotation_z(-aim);
    }
    // 装填的泡泡只随炮口平移；本体不旋转，让高光始终在左上，像稳定的球。
    let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + dir * 6.0;
    for (_e, mut t) in &mut loaded_q {
        t.translation.x = loaded_pos.x;
        t.translation.y = loaded_pos.y;
    }

    // 发射
    let shoot = !stage.shot_active
        && (keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::KeyJ)
            || keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::KeyW));
    if shoot {
        let muzzle = Vec2::new(CANNON_X, CANNON_Y) + dir * 36.0;
        spawn_flying_bubble(
            &mut commands,
            &assets,
            muzzle,
            dir * SHOT_SPEED,
            stage.current,
        );
        // 销毁炮口处的装填实体（连同子高光一起递归销毁）
        for (e, _t) in &loaded_q {
            commands.entity(e).despawn();
        }
        stage.shot_active = true;
    }
}
