pub mod bullet;
pub mod models;

use bullet::models::*;
use models::*;

use bevy::{input::keyboard::KeyboardInput, prelude::*, sprite::collide_aabb::collide};

use crate::wall::models::Wall;

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
    mut mouse_button: EventReader<KeyboardInput>,
    q_gun: Query<&GlobalTransform, With<Gun>>,
) {
    let gun_transform = q_gun.single();
    for item in mouse_button.iter() {
        if item.state.is_pressed() {
            if let Some(key_code) = item.key_code {
                if key_code == KeyCode::Space {
                    let mut transform: Transform = gun_transform
                        .compute_transform()
                        .with_scale(Vec3::new(12.0, 20.0, 1.0));

                    transform.translation =
                        bullet::local_transform_by_offset(&transform, 0.0, 35.0);

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
        }
    }
}

const ROTATION_SPEED: f32 = 0.05;
pub fn set_aim_lock(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut q_gun_child: Query<&mut RotationLock, With<Gun>>,
) {
    let mut rotation_lock = q_gun_child.single_mut();
    for input in keyboard_event.iter() {
        match input.key_code {
            Some(x) if x == KeyCode::J => {
                rotation_lock.angle = if input.state.is_pressed() {
                    ROTATION_SPEED
                } else {
                    0.0
                };
            }
            Some(x) if x == KeyCode::L => {
                rotation_lock.angle = if input.state.is_pressed() {
                    -ROTATION_SPEED
                } else {
                    0.0
                };
            }
            _ => {}
        }
    }
}

pub fn handle_aim_lock(mut q_gun_child: Query<(&RotationLock, &mut Transform), With<Gun>>) {
    let (rotation_lock, mut gun_transform) = q_gun_child.single_mut();
    if rotation_lock.angle != 0.0 {
        gun_transform.rotate_local(Quat::from_rotation_z(rotation_lock.angle));
    }
}
