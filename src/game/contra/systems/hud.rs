use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::resources::ContraStage;

pub fn contra_hud_update(
    session: Res<GameSession>,
    stage: Res<ContraStage>,
    player_q: Query<&ContraPlayer>,
    boss_q: Query<&ContraBoss>,
    mut hud_text_q: Query<(&mut Text2d, &mut TextColor, &ContraHud), Without<Sprite>>,
    mut hud_sprite_q: Query<(&mut Sprite, &ContraHud), Without<Text2d>>,
    mut life_icons_q: Query<(&ContraHudLifeIcon, &mut Visibility)>,
) {
    let weapon = player_q.single().map(|p| p.weapon).unwrap_or(Weapon::M);
    let boss_hp = boss_q.single().ok().map(|b| (b.hp, b.die_t));
    let top_shown = stage.top_score.max(session.score);

    for (mut t, mut tc, hud) in &mut hud_text_q {
        match hud.kind {
            ContraHudKind::Score => {
                *t = Text2d::new(format!("1P-{:06}", session.score));
                tc.0 = Color::srgb(1.0, 0.95, 0.30);
            }
            ContraHudKind::TopScore => {
                *t = Text2d::new(format!("TOP-{:06}", top_shown));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::Lives => {
                *t = Text2d::new(format!("x{}", session.lives.max(0)));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::WeaponLetter => {
                *t = Text2d::new(weapon.letter().to_string());
                tc.0 = Color::srgb(0.0, 0.0, 0.0);
            }
            ContraHudKind::World => {
                *t = Text2d::new(format!("STAGE 1-{}", stage.level));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::BossHp => match boss_hp {
                Some((hp, die_t)) if die_t <= 0.0 => {
                    *t = Text2d::new(format!("BOSS {:>2}", hp.max(0)));
                    tc.0 = Color::srgb(1.0, 0.4, 0.32);
                }
                _ => *t = Text2d::new(String::new()),
            },
            ContraHudKind::Status => {
                if session.finished {
                    *t = Text2d::new(session.status.clone());
                    tc.0 = Color::srgb(1.0, 0.95, 0.30);
                } else if session.paused {
                    *t = Text2d::new("PAUSED  Esc 继续 / Backspace 返回".to_string());
                    tc.0 = Color::srgb(1.0, 1.0, 1.0);
                } else {
                    *t = Text2d::new(String::new());
                }
            }
            ContraHudKind::Weapon => {}
        }
    }

    for (mut sprite, hud) in &mut hud_sprite_q {
        if hud.kind == ContraHudKind::Weapon {
            sprite.color = weapon.pickup_color();
        }
    }

    let lives = session.lives.max(0);
    for (icon, mut vis) in &mut life_icons_q {
        *vis = if icon.idx < lives {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
