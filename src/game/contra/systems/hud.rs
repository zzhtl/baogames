use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::BOSS_GAUGE_SEGMENTS;
use super::super::resources::ContraStage;

fn boss_gauge_segments(hp: i32, max_hp: i32, total: u8) -> u8 {
    if hp <= 0 || max_hp <= 0 || total == 0 {
        return 0;
    }
    let filled = (hp * total as i32 + max_hp - 1) / max_hp;
    filled.clamp(1, total as i32) as u8
}

pub fn contra_hud_update(
    session: Res<GameSession>,
    stage: Res<ContraStage>,
    player_q: Query<&ContraPlayer>,
    boss_q: Query<&ContraBoss>,
    mut hud_text_q: Query<(&mut Text2d, &ContraHud), Without<Sprite>>,
    mut hud_sprite_q: Query<(&mut Sprite, &ContraHud, Option<&mut Visibility>), Without<Text2d>>,
    // Without<ContraHud>：和 hud_sprite_q 同样要 &mut Visibility，
    // 不显式拆开的话 Bevy 判定为冲突，系统初始化就 panic(B0001)。
    mut life_icons_q: Query<(&ContraHudLifeIcon, &mut Visibility), Without<ContraHud>>,
) {
    let weapon = player_q.single().map(|p| p.weapon).unwrap_or(Weapon::M);
    let boss_hp = boss_q.single().ok().map(|b| (b.hp, b.die_t));
    let top_shown = stage.top_score.max(session.score);
    let filled = boss_hp
        .filter(|(_, die_t)| *die_t <= 0.0)
        .map(|(hp, _)| boss_gauge_segments(hp, stage.boss_hp, BOSS_GAUGE_SEGMENTS));

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
                    set_text(&mut t, &format!("CORE {:>2}", hp.max(0)));
                }
                _ => set_text(&mut t, ""),
            },
            ContraHudKind::Weapon | ContraHudKind::BossGauge(_) => {}
        }
    }

    for (mut sprite, hud, visibility) in &mut hud_sprite_q {
        match hud.kind {
            ContraHudKind::Weapon if sprite.color != weapon.pickup_color() => {
                sprite.color = weapon.pickup_color();
            }
            ContraHudKind::BossGauge(segment) => {
                if let Some(mut visibility) = visibility {
                    let show = filled.is_some_and(|filled| {
                        segment.is_none_or(|segment| segment < filled)
                    });
                    *visibility = if show {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };
                }
            }
            _ => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boss_gauge_keeps_one_segment_for_the_last_hit_point() {
        assert_eq!(boss_gauge_segments(30, 30, 12), 12);
        assert_eq!(boss_gauge_segments(1, 30, 12), 1);
        assert_eq!(boss_gauge_segments(0, 30, 12), 0);
    }
}
