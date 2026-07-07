use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::resources::ContraStage;

pub fn contra_hud_update(
    session: Res<GameSession>,
    stage: Res<ContraStage>,
    player_q: Query<&ContraPlayer>,
    boss_q: Query<&ContraBoss>,
    mut hud_text_q: Query<(&mut Text2d, &ContraHud), Without<Sprite>>,
    mut hud_sprite_q: Query<(&mut Sprite, &ContraHud), Without<Text2d>>,
    mut life_icons_q: Query<(&ContraHudLifeIcon, &mut Visibility)>,
) {
    let weapon = player_q.single().map(|p| p.weapon).unwrap_or(Weapon::M);
    let boss_hp = boss_q.single().ok().map(|b| (b.hp, b.die_t));
    let top_shown = stage.top_score.max(session.score);

    // 文本颜色在 spawn 时就固定了，这里只更新内容（set_text 值未变时零开销）
    for (mut t, hud) in &mut hud_text_q {
        match hud.kind {
            ContraHudKind::Score => set_text(&mut t, &format!("1P-{:06}", session.score)),
            ContraHudKind::TopScore => set_text(&mut t, &format!("TOP-{:06}", top_shown)),
            ContraHudKind::Lives => set_text(&mut t, &format!("x{}", session.lives.max(0))),
            ContraHudKind::WeaponLetter => set_text(&mut t, weapon.letter()),
            ContraHudKind::World => set_text(&mut t, &format!("STAGE 1-{}", stage.level)),
            ContraHudKind::BossHp => match boss_hp {
                Some((hp, die_t)) if die_t <= 0.0 => {
                    set_text(&mut t, &format!("BOSS {:>2}", hp.max(0)));
                }
                _ => set_text(&mut t, ""),
            },
            ContraHudKind::Weapon => {}
        }
    }

    for (mut sprite, hud) in &mut hud_sprite_q {
        if hud.kind == ContraHudKind::Weapon && sprite.color != weapon.pickup_color() {
            sprite.color = weapon.pickup_color();
        }
    }

    let lives = session.lives.max(0);
    for (icon, mut vis) in &mut life_icons_q {
        let target = if icon.idx < lives {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != target {
            *vis = target;
        }
    }
}
