use bevy::prelude::*;

use super::super::components::*;
use super::super::palette::*;

pub fn contra_player_blink(
    time: Res<Time>,
    mut q: Query<(&ContraPlayer, &mut Visibility)>,
) {
    for (player, mut visibility) in &mut q {
        let show = if player.invincible > 0.0 {
            (time.elapsed_secs() * 24.0).sin() > 0.0
        } else {
            true
        };
        *visibility = if show {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn contra_explosion_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraExplosion, &mut Sprite, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (e, mut x, mut sprite, mut tr) in &mut q {
        x.t += dt;
        let p = (x.t / x.max_t).clamp(0.0, 1.0);
        let sz = x.size * (1.0 + p * 0.8);
        sprite.custom_size = Some(Vec2::splat(sz));
        let col = if p < 0.4 {
            COLOR_EXPL_HOT
        } else if p < 0.75 {
            COLOR_EXPL_MID
        } else {
            COLOR_EXPL_OUT
        };
        let mut c = col.to_srgba();
        c.alpha = 1.0 - p;
        sprite.color = Color::Srgba(c);
        tr.scale = Vec3::splat(1.0);
        if x.t >= x.max_t {
            commands.entity(e).despawn();
        }
    }
}
