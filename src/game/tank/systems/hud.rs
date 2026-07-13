use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession};

use super::super::components::{BaseFC, EnemyTankFC, TankHud, TankHudKind};
use super::super::constants::STAGE_TOTAL_ENEMIES;
use super::super::resources::TankStage;

const BASE_DANGER_RADIUS: f32 = 120.0;

fn base_in_danger(base: Option<Vec2>, mut enemies: impl Iterator<Item = Vec2>) -> bool {
    let Some(base) = base else { return false };
    enemies.any(|enemy| enemy.distance_squared(base) <= BASE_DANGER_RADIUS.powi(2))
}

pub fn tank_hud_update(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Option<Res<TankStage>>,
    base_q: Query<&Transform, With<BaseFC>>,
    enemy_q: Query<&Transform, With<EnemyTankFC>>,
    mut hud: Query<(&mut Text2d, &mut TextColor, &TankHud)>,
) {
    if session.kind != GameKind::Tank {
        return;
    }
    let Some(stage) = stage else { return };
    let remaining = STAGE_TOTAL_ENEMIES.saturating_sub(stage.kills);
    let base_pos = base_q.single().ok().map(|t| t.translation.truncate());
    let danger = stage.base_alive
        && base_in_danger(base_pos, enemy_q.iter().map(|t| t.translation.truncate()));
    let danger_blink = (time.elapsed_secs() * 8.0).sin() > 0.0;
    for (mut text, mut color, hud) in &mut hud {
        match hud.kind {
            TankHudKind::Enemies => set_text(&mut text, &remaining.to_string()),
            TankHudKind::P1Lives => set_text(&mut text, &format!("×{}", stage.p1_lives.max(0))),
            TankHudKind::P2Lives => set_text(&mut text, &format!("×{}", stage.p2_lives.max(0))),
            TankHudKind::Base => {
                let (value, next_color) = if !stage.base_alive {
                    ("BASE LOST", Color::srgb(1.0, 0.22, 0.16))
                } else if danger {
                    (
                        "BASE !!",
                        if danger_blink {
                            Color::srgb(1.0, 0.28, 0.16)
                        } else {
                            Color::srgb(1.0, 0.86, 0.24)
                        },
                    )
                } else {
                    ("BASE OK", Color::srgb(0.42, 0.86, 0.46))
                };
                set_text(&mut text, value);
                color.0 = next_color;
            }
            TankHudKind::Freeze => {
                if stage.freeze_timer > 0.0 {
                    set_text(&mut text, &format!("TIME {:>2}", stage.freeze_timer.ceil() as u8));
                } else {
                    set_text(&mut text, "");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_warning_uses_enemy_distance() {
        let base = Some(Vec2::ZERO);
        assert!(base_in_danger(base, [Vec2::new(100.0, 0.0)].into_iter()));
        assert!(!base_in_danger(
            base,
            [Vec2::new(200.0, 0.0)].into_iter()
        ));
        assert!(!base_in_danger(None, [Vec2::ZERO].into_iter()));
    }
}
