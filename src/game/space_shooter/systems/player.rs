use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::{SpaceControls, SpaceState};
use super::super::setup::{spawn_bullet, spawn_muzzle_flash};

pub fn space_sample_input(
    actions: Res<ActionState>,
    session: Res<GameSession>,
    mut controls: ResMut<SpaceControls>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        controls.clear();
        return;
    }
    controls.sample(&actions);
}

pub fn space_player_input(
    mut commands: Commands,
    time: Res<Time>,
    mut controls: ResMut<SpaceControls>,
    session: Res<GameSession>,
    mut state: ResMut<SpaceState>,
    mut players: Query<(&mut Transform, &mut SpaceShipPlayer)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let mut dir = controls.movement();
    if dir.length_squared() > 1.0 {
        dir = dir.normalize();
    }
    let firing = controls.firing();
    let roll_requested = controls.take_roll();
    let fire_cd = match state.power {
        0 | 1 => PLAYER_FIRE_CD_LV1,
        2 => PLAYER_FIRE_CD_LV2,
        _ => PLAYER_FIRE_CD_LV3,
    };
    for (mut t, mut player) in &mut players {
        t.translation.x += dir.x * PLAYER_SPEED * delta;
        t.translation.y += dir.y * PLAYER_SPEED * delta;
        t.translation.x = t
            .translation
            .x
            .clamp(PLAY_LEFT + 14.0, PLAY_RIGHT - 14.0);
        t.translation.y = t.translation.y.clamp(PLAY_BOTTOM + 18.0, PLAY_TOP - 18.0);
        player.fire_cd_left = (player.fire_cd_left - delta).max(0.0);
        player.invincible_left = (player.invincible_left - delta).max(0.0);
        player.roll_left = (player.roll_left - delta).max(0.0);
        player.recoil_left = (player.recoil_left - delta).max(0.0);
        player.blink_phase += delta * 18.0;
        player.move_dir = dir;
        if roll_requested {
            if state.rolls > 0 && player.roll_left <= 0.0 {
                state.rolls -= 1;
                player.roll_left = PLAYER_ROLL_TIME;
                player.invincible_left = player.invincible_left.max(PLAYER_ROLL_INVINCIBLE);
                state.message = "回避翻滚！".to_string();
                state.message_clock = 0.9;
                sfx.write(PlaySfx(SfxKind::Flip));
            } else if state.rolls == 0 {
                state.message = "回避次数已用完".to_string();
                state.message_clock = 0.9;
                sfx.write(PlaySfx(SfxKind::Deny));
            }
        }
        if firing && player.roll_left <= 0.0 && player.fire_cd_left <= 0.0 {
            let pos = t.translation.truncate();
            fire_player_volley(&mut commands, pos, state.power);
            spawn_muzzle_flash(&mut commands, pos + Vec2::new(0.0, 21.0));
            sfx.write(PlaySfx(SfxKind::Shoot));
            player.fire_cd_left = fire_cd;
            player.recoil_left = 0.06;
        }
    }
}

fn fire_player_volley(commands: &mut Commands, pos: Vec2, power: u8) {
    let bullet_color = Color::srgb(1.0, 0.92, 0.42);
    match power {
        0 | 1 => {
            spawn_bullet(
                commands,
                pos + Vec2::new(0.0, 18.0),
                Vec2::new(0.0, PLAYER_BULLET_SPEED),
                true,
                1,
                bullet_color,
                Vec2::new(5.0, 14.0),
            );
        }
        2 => {
            for x_off in [-7.0_f32, 7.0] {
                spawn_bullet(
                    commands,
                    pos + Vec2::new(x_off, 16.0),
                    Vec2::new(0.0, PLAYER_BULLET_SPEED),
                    true,
                    1,
                    bullet_color,
                    Vec2::new(5.0, 14.0),
                );
            }
        }
        _ => {
            spawn_bullet(
                commands,
                pos + Vec2::new(0.0, 20.0),
                Vec2::new(0.0, PLAYER_BULLET_SPEED),
                true,
                1,
                bullet_color,
                Vec2::new(6.0, 16.0),
            );
            spawn_bullet(
                commands,
                pos + Vec2::new(-10.0, 14.0),
                Vec2::new(-110.0, PLAYER_BULLET_SPEED * 0.96),
                true,
                1,
                bullet_color,
                Vec2::new(5.0, 14.0),
            );
            spawn_bullet(
                commands,
                pos + Vec2::new(10.0, 14.0),
                Vec2::new(110.0, PLAYER_BULLET_SPEED * 0.96),
                true,
                1,
                bullet_color,
                Vec2::new(5.0, 14.0),
            );
        }
    }
}

pub fn space_player_blink(
    time: Res<Time>,
    session: Res<GameSession>,
    mut players: Query<(&SpaceShipPlayer, &mut Transform, &mut Visibility), Without<SpaceEngineFlame>>,
    mut flames: Query<&mut Transform, (With<SpaceEngineFlame>, Without<SpaceShipPlayer>)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (player, mut transform, mut visibility) in &mut players {
        if player.roll_left > 0.0 {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = if player.invincible_left > 0.0 && player.blink_phase.sin() < 0.0 {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
        let bank = -player.move_dir.x * 0.11;
        let roll = if player.roll_left > 0.0 {
            (1.0 - player.roll_left / PLAYER_ROLL_TIME) * std::f32::consts::TAU
        } else {
            0.0
        };
        transform.rotation = Quat::from_rotation_z(bank + roll);
        let recoil = if player.recoil_left > 0.0 { 0.08 } else { 0.0 };
        transform.scale = Vec3::new(1.0 + recoil, 1.0 - recoil, 1.0);
    }
    let pulse = 0.85 + (time.elapsed_secs() * 24.0).sin() * 0.18;
    for mut transform in &mut flames {
        transform.scale = Vec3::new(1.0, pulse, 1.0);
    }
}
