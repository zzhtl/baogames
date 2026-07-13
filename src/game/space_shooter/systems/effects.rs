use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession, Lifetime};

use super::super::components::{
    EnemyKind, SpaceEnemy, SpaceExplosionParticle, SpaceMuzzleFlash, SpacePowerUp,
};

type SpaceEffectQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Lifetime,
        &'static mut Sprite,
        &'static mut Transform,
        Option<&'static SpaceExplosionParticle>,
    ),
    Or<(With<SpaceExplosionParticle>, With<SpaceMuzzleFlash>)>,
>;

pub fn space_effects_update(
    session: Res<GameSession>,
    mut effects: SpaceEffectQuery,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (lifetime, mut sprite, mut transform, explosion) in &mut effects {
        let progress = lifetime.0.fraction();
        sprite.color.set_alpha((1.0 - progress).clamp(0.0, 1.0));
        if let Some(explosion) = explosion {
            let scale = if explosion.grows {
                0.55 + progress * 0.75
            } else {
                1.0 - progress * 0.55
            };
            transform.scale = Vec3::splat(scale.max(0.2));
        } else {
            transform.scale = Vec3::new(1.0 + progress * 0.4, 1.0 - progress * 0.75, 1.0);
        }
    }
}

pub fn space_enemy_visual_update(
    session: Res<GameSession>,
    mut enemies: Query<(&SpaceEnemy, &mut Transform)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    for (enemy, mut transform) in &mut enemies {
        let bank = match enemy.kind {
            EnemyKind::Scout => (enemy.time_alive * 3.2).sin() * 0.08,
            EnemyKind::Sniper | EnemyKind::Carrier => (enemy.time_alive * 2.2).sin() * 0.045,
            EnemyKind::Bomber => (enemy.time_alive * 1.6).sin() * 0.025,
            EnemyKind::Boss => (enemy.time_alive * 1.1).sin() * 0.012,
        };
        transform.rotation = Quat::from_rotation_z(bank);
        let hit = if enemy.hit_flash_left > 0.0 { 0.09 } else { 0.0 };
        transform.scale = Vec3::new(1.0 + hit, 1.0 - hit * 0.45, 1.0);
    }
}

pub fn space_powerup_visual_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut powerups: Query<&mut Transform, With<SpacePowerUp>>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    let t = time.elapsed_secs();
    for mut transform in &mut powerups {
        let pulse = 1.0 + (t * 7.0).sin() * 0.08;
        transform.scale = Vec3::new(pulse, pulse, 1.0);
        transform.rotation = Quat::from_rotation_z((t * 2.4).sin() * 0.08);
    }
}
