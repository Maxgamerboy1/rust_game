pub mod models;

use models::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::enemy::models::Enemy;

pub fn move_bullet(
    mut q_bullet: Query<(&mut Transform, &mut BulletLifespan), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (mut bullet_transform, mut bullet_lifespan) in q_bullet.iter_mut() {
        bullet_transform.translation = local_transform_by_offset(&bullet_transform, 0.0, 10.0);
        bullet_lifespan.1 += res_time.delta_seconds();
    }
}

pub fn local_transform_by_offset(transform: &Transform, x_value: f32, y_value: f32) -> Vec3 {
    let x_offset = Vec2::dot(
        transform.translation.truncate(),
        transform.right().truncate(),
    ) + x_value;
    let y_offset = Vec2::dot(transform.translation.truncate(), transform.up().truncate()) - y_value;

    x_offset * transform.right() + y_offset * transform.up()
}

pub fn check_bullet_hit_enemy(
    mut commands: Commands,
    q_bullet: Query<(Entity, &Transform), With<Bullet>>,
    q_enemy: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_ent, bullet_transform) in q_bullet.iter() {
        for (enemy_ent, enemy_transform) in q_enemy.iter() {
            if let Some(_) = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            ) {
                commands.entity(enemy_ent).despawn();
                commands.entity(bullet_ent).despawn();
            }
        }
    }
}

pub fn check_bullet_lifespan(
    mut commands: Commands,
    q_bullet: Query<(Entity, &BulletLifespan), With<Bullet>>,
) {
    for (bullet_entity, bullet_lifespan) in q_bullet.iter() {
        if bullet_lifespan.1 > bullet_lifespan.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}
