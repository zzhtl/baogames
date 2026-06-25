use bevy::prelude::*;

use crate::common::constants::ARENA_W;
use crate::game::model::GameSession;

use super::super::resources::{ContraStage, EnemySpawnMark, FalconMark};
use super::super::setup_actors::{spawn_boss, spawn_enemy, spawn_falcon};

pub fn contra_spawner(
    session: Res<GameSession>,
    mut commands: Commands,
    cam_q: Query<&Transform, With<Camera>>,
    mut stage: ResMut<ContraStage>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(cam) = cam_q.single() else { return };
    let cam_right = cam.translation.x + ARENA_W * 0.5;
    while stage.spawn_idx < stage.spawn_marks.len()
        && stage.spawn_marks[stage.spawn_idx].trigger_x <= cam_right
    {
        let mark_pos = stage.spawn_marks[stage.spawn_idx].pos;
        let mark_kind = stage.spawn_marks[stage.spawn_idx].kind;
        let mark_facing = stage.spawn_marks[stage.spawn_idx].facing;
        spawn_enemy(
            &mut commands,
            &EnemySpawnMark {
                trigger_x: 0.0,
                pos: mark_pos,
                kind: mark_kind,
                facing: mark_facing,
            },
        );
        stage.spawn_idx += 1;
    }
    while stage.falcon_idx < stage.falcon_marks.len()
        && stage.falcon_marks[stage.falcon_idx].trigger_x <= cam_right
    {
        let m = &stage.falcon_marks[stage.falcon_idx];
        spawn_falcon(
            &mut commands,
            &FalconMark {
                trigger_x: 0.0,
                start: m.start,
                vx: m.vx,
                weapon: m.weapon,
            },
        );
        stage.falcon_idx += 1;
    }
    if !stage.boss_spawned && cam_right >= stage.boss_x - 60.0 {
        spawn_boss(&mut commands, stage.boss_x, stage.boss_hp);
        stage.boss_spawned = true;
    }
}
