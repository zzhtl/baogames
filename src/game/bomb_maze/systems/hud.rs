use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::resources::BMStage;

pub fn bm_hud_update(
    session: Res<GameSession>,
    stage: Option<Res<BMStage>>,
    enemies: Query<&BMEnemy>,
    players: Query<&BMPlayer>,
    mut hud: Query<(&BMHud, &mut Text2d)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let Some(stage) = stage else { return };
    let enemy_count = enemies.iter().count();
    let mut power = [None; 2];
    for player in &players {
        if player.id < power.len() {
            power[player.id] = Some((player.max_bombs, player.bomb_range));
        }
    }
    for (kind, mut text) in &mut hud {
        match kind {
            BMHud::Stage => set_text(&mut text, &format!("{}", stage.level)),
            BMHud::Time => set_text(&mut text, &format!("{:.0}", stage.time_left.max(0.0))),
            BMHud::Score => set_text(&mut text, &format!("{}", session.score)),
            BMHud::Enemies => set_text(&mut text, &format!("{}", enemy_count)),
            BMHud::P1Lives => set_text(&mut text, &format!("x{}", stage.p1_lives.max(0))),
            BMHud::P2Lives => set_text(&mut text, &format!("x{}", stage.p2_lives.max(0))),
            BMHud::P1Power => set_text(&mut text, &power_text(power[0])),
            BMHud::P2Power => set_text(&mut text, &power_text(power[1])),
            // 暂停/结束由统一覆盖层显示，这里只放玩法提示
            BMHud::Status => {
                let s = if session.finished { "" } else { stage.status.as_str() };
                set_text(&mut text, s);
            }
        }
    }
}

fn power_text(power: Option<(u8, i32)>) -> String {
    power
        .map(|(bombs, range)| format!("弹{bombs} 火{range}"))
        .unwrap_or_else(|| "弹- 火-".to_string())
}

#[cfg(test)]
mod tests {
    use super::power_text;

    #[test]
    fn power_hud_reports_capacity_and_range() {
        assert_eq!(power_text(Some((3, 5))), "弹3 火5");
        assert_eq!(power_text(None), "弹- 火-");
    }
}
