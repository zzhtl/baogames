use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::palette::*;
use super::super::setup_actors::rebuild_player_visual;

pub fn contra_player_pose_sync(
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraPlayer)>,
) {
    let Ok((entity, mut player)) = q.single_mut() else {
        return;
    };
    let pose = if player.dead_timer > 0.0 || !player.on_ground {
        ContraPlayerPose::Flip
    } else if player.prone {
        ContraPlayerPose::Prone
    } else {
        ContraPlayerPose::Stand
    };
    if pose != player.pose {
        rebuild_player_visual(&mut commands, entity, pose);
        player.pose = pose;
    }
}

pub fn contra_player_animation(mut q: Query<(&ContraPlayer, &mut Transform)>) {
    let Ok((player, mut tr)) = q.single_mut() else {
        return;
    };
    let facing = if player.facing >= 0.0 { 1.0 } else { -1.0 };
    let landing = (player.landing_t / 0.11).clamp(0.0, 1.0);
    let walk = if player.pose == ContraPlayerPose::Stand
        && player.on_ground
        && player.vel.x.abs() > 1.0
    {
        (player.visual_t * 18.0).sin() * 0.045
    } else {
        0.0
    };
    tr.scale = Vec3::new(
        facing * (1.0 + landing * 0.10),
        1.0 + walk - landing * 0.16,
        1.0,
    );
    tr.rotation = if player.pose == ContraPlayerPose::Flip {
        Quat::from_rotation_z(-player.visual_t * 10.0 * facing)
    } else {
        Quat::IDENTITY
    };
}

pub fn contra_boss_flash_update(
    boss_q: Query<&ContraBoss>,
    mut boss_flash_q: Query<&mut Sprite, With<ContraBossFlash>>,
) {
    if let (Ok(boss), Ok(mut flash)) = (boss_q.single(), boss_flash_q.single_mut()) {
        let alpha = if boss.die_t > 0.0 {
            (boss.die_t * 24.0).sin().abs()
        } else if boss.flash_t > 0.0 {
            0.9
        } else {
            0.0
        };
        flash.color = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

pub fn contra_turret_flash_update(mut turret_q: Query<(&ContraTurret, &mut Sprite)>) {
    for (turret, mut sprite) in &mut turret_q {
        sprite.color = if turret.hit_t > 0.0 {
            Color::WHITE
        } else {
            COLOR_TURRET
        };
    }
}

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

pub fn contra_muzzle_flash_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraMuzzleFlash, &mut Sprite, &mut Transform)>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (entity, mut flash, mut sprite, mut tr) in &mut q {
        flash.life -= dt;
        let p = (1.0 - flash.life / flash.max_life).clamp(0.0, 1.0);
        tr.scale.x = 1.0 + p * 0.7;
        let mut color = COLOR_EXPL_HOT.to_srgba();
        color.alpha = 1.0 - p;
        sprite.color = Color::Srgba(color);
        if flash.life <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
