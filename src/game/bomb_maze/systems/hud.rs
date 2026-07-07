use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::resources::BMStage;

pub fn bm_hud_update(
    session: Res<GameSession>,
    stage: Option<Res<BMStage>>,
    enemies: Query<&BMEnemy>,
    mut hud: Query<(&BMHud, &mut Text2d)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let Some(stage) = stage else { return };
    let enemy_count = enemies.iter().count();
    for (kind, mut text) in &mut hud {
        match kind {
            BMHud::Stage => set_text(&mut text, &format!("{}", stage.level)),
            BMHud::Time => set_text(&mut text, &format!("{:.0}", stage.time_left.max(0.0))),
            BMHud::Score => set_text(&mut text, &format!("{}", session.score)),
            BMHud::Enemies => set_text(&mut text, &format!("{}", enemy_count)),
            BMHud::P1Lives => set_text(&mut text, &format!("x{}", stage.p1_lives.max(0))),
            BMHud::P2Lives => set_text(&mut text, &format!("x{}", stage.p2_lives.max(0))),
            // 暂停/结束由统一覆盖层显示，这里只放玩法提示
            BMHud::Status => {
                let s = if session.finished { "" } else { stage.status.as_str() };
                set_text(&mut text, s);
            }
        }
    }
}
