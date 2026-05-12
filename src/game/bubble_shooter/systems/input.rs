use bevy::prelude::*;

use crate::game::model::{GameEntity, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::grid::aim_dir;
use super::super::resources::BubbleStage;

pub fn bubble_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<BubbleStage>,
    mut barrel_q: Query<&mut Transform, (With<CannonBarrel>, Without<LoadedBubble>)>,
    mut loaded_q: Query<
        (Entity, &mut Transform, Option<&BubbleColor>),
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
    let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + dir * 6.0;
    let highlight_local = dir.perp() * -7.0 + dir * 6.0;
    for (_e, mut t, color) in &mut loaded_q {
        if color.is_some() {
            t.translation.x = loaded_pos.x;
            t.translation.y = loaded_pos.y;
        } else {
            // 高光
            t.translation.x = loaded_pos.x + highlight_local.x;
            t.translation.y = loaded_pos.y + highlight_local.y;
        }
    }

    // 发射
    let shoot = !stage.shot_active
        && (keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::KeyJ)
            || keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::KeyW));
    if shoot {
        let muzzle = Vec2::new(CANNON_X, CANNON_Y) + dir * 36.0;
        let color_id = stage.current;
        // 飞行泡泡
        commands.spawn((
            Sprite::from_color(palette(color_id), Vec2::splat(BUBBLE_D - 4.0)),
            Transform::from_translation(muzzle.extend(Z_FLYING)),
            FlyingBubble {
                vel: dir * SHOT_SPEED,
            },
            BubbleColor(color_id),
            GameEntity,
        ));
        // 销毁炮口处的装填精灵（主体+高光）
        for (e, _t, _c) in &loaded_q {
            commands.entity(e).despawn();
        }
        stage.shot_active = true;
    }
}
