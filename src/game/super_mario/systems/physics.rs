use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::collide::{Solid as Solid_, resolve};
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::level_world_max_x;
use super::super::setup_actors::spawn_brick_shards;

pub fn mario_physics(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &mut Transform), Without<Solid>>,
    solid_q: Query<(Entity, &Transform, &Solid, Option<&MovingPlatform>), Without<MarioPlayer>>,
    brick_q: Query<&BrickTile>,
    mut question_q: Query<&mut QuestionBlock>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((mut player, mut tr)) = player_q.single_mut() else {
        return;
    };

    if player.dead_timer > 0.0 {
        player.vel.y -= GRAVITY * dt;
        player.vel.y = player.vel.y.max(-FALL_MAX);
        tr.translation.y += player.vel.y * dt;
        return;
    }

    if player.finished {
        let mut v = player.vel;
        v.x = WALK_SPEED * 0.7;
        v.y -= GRAVITY * dt;
        v.y = v.y.max(-FALL_MAX);
        let shapes: Vec<Solid_> = solid_q
            .iter()
            .map(|(_, st, s, _)| Solid_::fixed(st.translation.truncate(), s.size))
            .collect();
        let r = resolve(tr.translation.truncate(), player.state.size(), v, dt, &shapes);
        tr.translation.x = r.pos.x;
        tr.translation.y = r.pos.y;
        player.vel = r.vel;
        player.on_ground = r.on_ground;
        return;
    }

    let g = if player.jumping && player.vel.y > 0.0 {
        JUMP_HOLD_GRAVITY
    } else {
        GRAVITY
    };
    player.vel.y -= g * dt;
    player.vel.y = player.vel.y.max(-FALL_MAX);

    let p_size = player.state.size();
    // 地形解算走公共模块：先脱困再分轴推进。
    // 之前是「无条件按 X 穿透深度整体推出」，只要 Y 方向嵌进去 1 单位
    // （变身长高 / 站上升平台 / 地形接缝错位都会），人就会被横向弹飞几十单位。
    let solids: Vec<(Entity, Solid_)> = solid_q
        .iter()
        .map(|(e, st, s, plat)| {
            (
                e,
                Solid_ {
                    center: st.translation.truncate(),
                    size: s.size,
                    // 移动平台本帧走过的位移，供解算器把站在上面的人一起带走
                    delta: plat.map_or(Vec2::ZERO, |p| Vec2::new(p.last_dx, p.last_dy)),
                },
            )
        })
        .collect();
    let shapes: Vec<Solid_> = solids.iter().map(|(_, s)| *s).collect();
    let resolved = resolve(tr.translation.truncate(), p_size, player.vel, dt, &shapes);
    let mut pos = resolved.pos;
    player.vel = resolved.vel;
    player.on_ground = resolved.on_ground;
    // 站在移动平台上要跟着走。`last_dx/last_dy` 之前全库没有任何消费者，
    // 所以平台是从人脚下滑走的，人还会因此嵌进平台再被横向弹开。
    if let Some(i) = resolved.ground {
        pos += solids[i].1.delta;
    }
    let hit_above = resolved
        .ceiling
        .map(|i| (solids[i].0, solids[i].1.center));

    // 关卡左右边界要用当前体型的半宽：大马里奥比小马里奥宽 2 单位
    let left_min = p_size.x * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let right_max = level_world_max_x() - p_size.x * 0.5;
    if pos.x > right_max {
        pos.x = right_max;
        player.vel.x = 0.0;
    }

    if let Some((e, block_pos)) = hit_above {
        if let Ok(mut q) = question_q.get_mut(e) {
            if !q.used {
                q.used = true;
                q.bump_t = 0.18;
            } else if q.bump_t <= 0.0 {
                q.bump_t = 0.10;
            }
        } else if brick_q.get(e).is_ok() && !matches!(player.state, PowerState::Small) {
            // 大/火力马里奥顶碎砖块
            commands.entity(e).despawn();
            spawn_brick_shards(&mut commands, block_pos);
            sfx.write(PlaySfx(SfxKind::Hit));
            session.score = session.score.saturating_add(50);
            player.vel.y = -120.0;
        }
    }

    tr.translation.x = pos.x;
    tr.translation.y = pos.y;

    if pos.y < FALL_DEATH_Y && player.dead_timer <= 0.0 {
        player.dead_timer = 2.0;
        player.vel = Vec2::new(0.0, 380.0);
        session.lives -= 1;
    }

    if player.invincible > 0.0 {
        player.invincible -= dt;
    }
}
