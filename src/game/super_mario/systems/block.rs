use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{GameEntity, GameSession};

use super::super::components::*;
use super::super::constants::*;
use super::super::palette::*;
use super::super::resources::MarioStage;
use super::super::setup_actors::spawn_powerup;

pub fn mario_block_anim(
    time: Res<Time>,
    mut q_blocks: Query<(&mut Transform, &mut QuestionBlock, &mut Sprite, &Children)>,
    mut child_sprites: Query<&mut Sprite, Without<QuestionBlock>>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<&MarioPlayer>,
    mut commands: Commands,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs();
    let player_state = player_q
        .single()
        .ok()
        .map(|p| p.state)
        .unwrap_or(PowerState::Small);

    for (mut tr, mut qb, mut sprite, children) in &mut q_blocks {
        if qb.bump_t > 0.0 {
            qb.bump_t -= dt;
            let total = 0.18_f32;
            let t = (total - qb.bump_t.max(0.0)) / total;
            let off = (t * std::f32::consts::PI).sin() * 10.0;
            tr.translation.y = qb.base_y + off;
            if qb.bump_t <= 0.0 {
                tr.translation.y = qb.base_y;
            }
        }
        if qb.used {
            sprite.color = COLOR_QUESTION_USED;
            for c in children {
                if let Ok(mut cs) = child_sprites.get_mut(*c) {
                    if cs.color == COLOR_M_BLACK {
                        cs.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                    } else if cs.color == COLOR_QUESTION_DARK {
                        cs.color = Color::srgb(0.32, 0.20, 0.02);
                    }
                }
            }
        }
        if qb.used && !qb.spawn_done {
            qb.spawn_done = true;
            let block_pos = Vec2::new(tr.translation.x, qb.base_y);
            match qb.content {
                QuestionContent::Coin => {
                    stage.coins += 1;
                    session.score = session.score.saturating_add(200);
                    sfx.write(PlaySfx(SfxKind::Coin));
                    commands.spawn((
                        Sprite::from_color(COLOR_COIN, Vec2::new(10.0, 14.0)),
                        Transform::from_translation(Vec3::new(
                            block_pos.x,
                            qb.base_y + 32.0,
                            Z_COIN,
                        )),
                        CoinPopup {
                            timer: 0.5,
                            vy: 220.0,
                        },
                        GameEntity,
                    ));
                }
                QuestionContent::Mushroom => {
                    let kind = if matches!(player_state, PowerState::Small) {
                        PowerUpKind::Mushroom
                    } else {
                        PowerUpKind::FireFlower
                    };
                    spawn_powerup(&mut commands, block_pos, kind);
                }
                QuestionContent::Star => {
                    spawn_powerup(&mut commands, block_pos, PowerUpKind::Star);
                }
                QuestionContent::OneUp => {
                    spawn_powerup(&mut commands, block_pos, PowerUpKind::OneUp);
                }
            }
        }
    }
}

