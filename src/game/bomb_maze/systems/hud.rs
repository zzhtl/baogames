use bevy::prelude::*;

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
            BMHud::Stage => **text = format!("{}", stage.level),
            BMHud::Time => **text = format!("{:.0}", stage.time_left.max(0.0)),
            BMHud::Score => **text = format!("{}", session.score),
            BMHud::Enemies => **text = format!("{}", enemy_count),
            BMHud::P1Lives => **text = format!("x{}", stage.p1_lives.max(0)),
            BMHud::P2Lives => **text = format!("x{}", stage.p2_lives.max(0)),
            BMHud::Status => {
                let s = if session.finished {
                    if session.won {
                        "成功逃出迷宫！Enter 重玩，Esc 返回"
                    } else {
                        "迷宫之旅失败……Enter 重试，Esc 返回"
                    }
                } else if session.paused {
                    "已暂停：Esc 继续，Backspace 返回菜单"
                } else {
                    stage.status.as_str()
                };
                **text = s.to_string();
            }
        }
    }
}
