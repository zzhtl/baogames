use bevy::prelude::*;

use super::super::components::{BMBomb, BMEnemy, BMExit, BMPlayer, BMPowerup};

pub fn bm_actor_visual_update(
    time: Res<Time>,
    mut players: Query<(&BMPlayer, &mut Transform), Without<BMEnemy>>,
    mut enemies: Query<(&BMEnemy, &mut Transform), Without<BMPlayer>>,
) {
    for (player, mut transform) in &mut players {
        let step = if player.moving {
            (player.motion_t * 18.0).sin() * 0.055
        } else {
            0.0
        };
        transform.scale = Vec3::new(1.0 - step * 0.5, 1.0 + step, 1.0);
    }
    let t = time.elapsed_secs();
    for (enemy, mut transform) in &mut enemies {
        let speed_phase = enemy.kind.speed() * 0.035;
        let bob = (t * speed_phase + enemy.change_timer).sin() * 0.045;
        transform.scale = Vec3::new(1.0 - bob * 0.5, 1.0 + bob, 1.0);
    }
}

pub fn bm_bomb_visual_update(mut bombs: Query<(&BMBomb, &mut Transform)>) {
    for (bomb, mut transform) in &mut bombs {
        let pulse = (bomb.fuse.elapsed_secs() * 9.0).sin() * 0.08;
        let trigger = if bomb.triggered { 0.16 } else { 0.0 };
        transform.scale = Vec3::new(1.0 + pulse + trigger, 1.0 - pulse + trigger, 1.0);
    }
}

pub fn bm_item_visual_update(
    time: Res<Time>,
    mut exits: Query<&mut Transform, (With<BMExit>, Without<BMPowerup>)>,
    mut powerups: Query<&mut Transform, (With<BMPowerup>, Without<BMExit>)>,
) {
    let t = time.elapsed_secs();
    for mut transform in &mut exits {
        let pulse = 1.0 + (t * 5.0).sin() * 0.07;
        transform.scale = Vec3::new(pulse, pulse, 1.0);
        transform.rotation = Quat::from_rotation_z((t * 2.5).sin() * 0.04);
    }
    for mut transform in &mut powerups {
        let pulse = 1.0 + (t * 7.0).sin() * 0.05;
        transform.scale = Vec3::new(pulse, pulse, 1.0);
    }
}
