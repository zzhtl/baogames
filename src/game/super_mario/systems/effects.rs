use bevy::prelude::*;

use super::super::components::*;
use super::super::constants::{FALL_MAX, GRAVITY};

pub fn mario_player_blink(
    player_q: Query<(&MarioPlayer, &Children)>,
    mut sprite_q: Query<&mut Sprite, (With<MarioVisual>, Without<MarioPlayer>)>,
    time: Res<Time>,
) {
    for (player, children) in &player_q {
        let visible = if player.invincible > 0.0 || player.transform_t > 0.0 {
            (time.elapsed_secs() * 24.0).sin() > 0.0
        } else {
            true
        };
        for c in children {
            if let Ok(mut s) = sprite_q.get_mut(*c) {
                let mut col = s.color.to_srgba();
                col.alpha = if visible { 1.0 } else { 0.25 };
                s.color = Color::Srgba(col);
            }
        }
    }
}

pub fn mario_shard_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut BrickShard)>,
) {
    let dt = time.delta_secs().min(0.033);
    for (e, mut tr, mut s) in &mut q {
        s.life -= dt;
        if s.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        s.vel.y -= GRAVITY * dt;
        s.vel.y = s.vel.y.max(-FALL_MAX);
        tr.translation.x += s.vel.x * dt;
        tr.translation.y += s.vel.y * dt;
        tr.rotation *= Quat::from_rotation_z(s.spin * dt);
    }
}
