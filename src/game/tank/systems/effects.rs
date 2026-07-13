use bevy::prelude::*;

use super::super::components::{TankFC, TankShieldVisual};

pub fn tank_motion_visual_update(mut tanks: Query<(&TankFC, &mut Transform)>) {
    for (tank, mut transform) in &mut tanks {
        let hit = (tank.hit_t / 0.12).clamp(0.0, 1.0);
        let tread = if tank.moving {
            (tank.motion_t * 26.0).sin() * 0.025
        } else {
            0.0
        };
        transform.scale = Vec3::new(1.0 + hit * 0.10 + tread, 1.0 - hit * 0.08 - tread, 1.0);
    }
}

pub fn tank_shield_visual_update(
    time: Res<Time>,
    tanks: Query<&TankFC>,
    mut shields: Query<(&TankShieldVisual, &mut Sprite, &mut Visibility)>,
) {
    let pulse = (time.elapsed_secs() * 14.0).sin() * 0.18 + 0.62;
    for (shield, mut sprite, mut visibility) in &mut shields {
        let active = tanks
            .get(shield.owner)
            .is_ok_and(|tank| tank.shield_left > 0.0);
        *visibility = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        let mut color = Color::srgb(0.42, 0.82, 1.0).to_srgba();
        color.alpha = if active { pulse } else { 0.0 };
        sprite.color = Color::Srgba(color);
    }
}
