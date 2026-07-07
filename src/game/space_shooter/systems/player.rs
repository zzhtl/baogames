use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::input_for;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::SpaceState;
use super::super::setup::spawn_bullet;

pub fn space_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    state: Res<SpaceState>,
    mut players: Query<(&mut Transform, &mut SpaceShipPlayer)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let input = input_for(&keys, 0);
    let mut dir = input.move_dir;
    if dir.length_squared() > 1.0 {
        dir = dir.normalize();
    }
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
        player.blink_phase += delta * 18.0;
        if input.fire && player.fire_cd_left <= 0.0 {
            let pos = t.translation.truncate();
            fire_player_volley(&mut commands, pos, state.power);
            sfx.write(PlaySfx(SfxKind::Shoot));
            player.fire_cd_left = fire_cd;
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
    session: Res<GameSession>,
    mut players: Query<(&SpaceShipPlayer, &mut Sprite)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (p, mut sprite) in &mut players {
        if p.invincible_left > 0.0 {
            let visible = (p.blink_phase.sin() > 0.0) as i32 as f32;
            sprite.color.set_alpha(0.35 + visible * 0.55);
        } else {
            sprite.color.set_alpha(1.0);
        }
    }
}
