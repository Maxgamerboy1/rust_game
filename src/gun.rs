use bevy::{
    hierarchy::Parent,
    input::{mouse::MouseButtonInput, keyboard::KeyboardInput},
    math::{Quat, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        Bundle, Camera, Color, Commands, Component, Entity, EventReader, GlobalTransform,
        MouseButton, Query, Res, Transform, With, KeyCode,
    },
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
    window::Windows, time::Time,
};

use crate::{enemy::Enemy, main_scene::MainScene, new_person::Person, wall::Wall};

#[derive(Component)]
pub struct Gun;

#[derive(Bundle)]
pub struct GunBundle {
    #[bundle]
    pub display: SpriteBundle,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
/// lifespan_boundary (s), lifespan (s)
pub struct BulletLifespan(f32, f32);

#[derive(Bundle)]
struct BulletBundle {
    #[bundle]
    display: SpriteBundle,
}

pub fn move_bullet(
    mut q_bullet: Query<(&mut Transform, &mut BulletLifespan), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (mut bullet_transform, mut bullet_lifespan) in q_bullet.iter_mut() {
        bullet_transform.translation = local_transform_by_offset(&bullet_transform, 0.0, 1.0);
        bullet_lifespan.1 += res_time.delta_seconds();
    }
}

fn local_transform_by_offset(transform: &Transform, x_value: f32, y_value: f32) -> Vec3 {
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

pub fn check_bullet_hit_wall(
    mut commands: Commands,
    q_bullet: Query<(Entity, &Transform), With<Bullet>>,
    q_wall: Query<&Transform, With<Wall>>,
) {
    for (bullet_entity, bullet_transform) in q_bullet.iter() {
        for wall_transform in q_wall.iter() {
            if let Some(_) = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                wall_transform.translation,
                wall_transform.scale.truncate(),
            ) {
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    mut mouse_button: EventReader<MouseButtonInput>,
    q_gun: Query<&GlobalTransform, With<Gun>>,
) {
    let gun_transform = q_gun.single();
    for item in mouse_button.iter() {
        match item.button {
            MouseButton::Left => {
                if item.state.is_pressed() {
                    let mut transform: Transform = gun_transform
                        .compute_transform()
                        .with_scale(Vec3::new(12.0, 20.0, 1.0));

                    transform.translation = local_transform_by_offset(&transform, 0.0, 35.0);

                    commands
                        .spawn_bundle(BulletBundle {
                            display: SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                transform,
                                ..Default::default()
                            },
                        })
                        .insert(BulletLifespan(1.33, 0.0))
                        .insert(Bullet);
                }
            }
            _ => {}
        }
    }
}

pub fn handle_aim(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut q_gun_child: Query<(&Parent, &mut Transform), With<Gun>>,
) {
    let mut dir = Vec3::new(0.0, 1.0, 0.0);
    for key in keyboard_event.iter() {
        match key.key_code {
            Some(key_code) => {
                match key_code {
                    KeyCode::Up => dir.x = 0.75,
                    _ => {}
                }
            },
            None => {},
        }
    }

    let (parent, mut gun_transform) = q_gun_child.single_mut();
    // gun_transform.look_at(dir, Vec3::X);
}