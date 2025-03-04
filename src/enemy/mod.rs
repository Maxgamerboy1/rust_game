pub mod models;

use bevy::{prelude::*, sprite::collide_aabb};
use models::*;
use rand::Rng;

use crate::person::models::Person;

pub fn setup(mut commands: Commands, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        for _ in 0i8..8i8 {
            let width = window.width();
            let mut rnd = rand::thread_rng();
            let random_x = rnd.gen_range(-width / 2.0..width / 2.0);

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(random_x, window.height() / 2.0, 0.0)
                        .with_scale(Vec3::new(20.0, 20.0, 1.0)),
                    ..Default::default()
                })
                .insert(EnemySpeed(rnd.gen_range(0.632..2.0)))
                .insert(Enemy);
        }
    } else {
        print!("Cannot find primary window");
    }
}

pub fn move_enemy(
    windows: Res<Windows>,
    mut q_enemy: Query<(&mut EnemySpeed, &mut Transform), With<Enemy>>,
) {
    for (mut enemy_speed, mut enemy_transform) in q_enemy.iter_mut() {
        enemy_transform.translation.y -= enemy_speed.0;

        if let Some(window) = windows.get_primary() {
            let window_height = window.height();
            if enemy_transform.translation.y < -window_height / 2.0 {
                enemy_transform.translation.y = window_height;

                let mut rng = rand::thread_rng();
                let window_width = window.width();
                enemy_transform.translation.x =
                    rng.gen_range(-window_width / 2.0..window_width / 2.0);
                enemy_speed.0 = rng.gen_range(0.632..2.0);
            }
        }
    }
}

pub fn check_player_collision(
    q_enemy: Query<&Transform, With<Enemy>>,
    q_player: Query<(&Transform, Entity), With<Person>>,
    mut commands: Commands,
) {
    if let Ok((player_transform, player_entity)) = q_player.get_single() {
        for enemy_transform in q_enemy.iter() {
            if let Some(_) = collide_aabb::collide(
                player_transform.translation,
                player_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            ) {
                commands.entity(player_entity).despawn();
            }
        }
    }
}
