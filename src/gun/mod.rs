pub mod bullet;
pub mod models;

use bullet::models::*;
use models::*;

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
    sprite::collide_aabb::collide,
};

use crate::{person::models::Person, wall::models::Wall};

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
            _ => {}
        }
    }
}

pub fn handle_aim(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut q_gun_child: Query<(&Parent, &mut Transform), With<Gun>>,
    q_parent: Query<&Transform, (With<Person>, Without<Gun>)>,
) {
    let (parent, mut gun_transform) = q_gun_child.single_mut();
    if let Ok(person_transform) = q_parent.get(parent.get()) {
        for key in keyboard_event.iter() {
            let mut dir: Option<Vec3> = None;
            match key.key_code {
                Some(key_code) => match key_code {
                    KeyCode::Down => dir = Some(person_transform.up()),
                    KeyCode::Left => dir = Some(person_transform.right()),
                    KeyCode::Right => {
                        dir = Some(person_transform.right() * Vec3::new(-1.0, -1.0, 1.0))
                    }
                    KeyCode::Up => dir = Some(person_transform.up() * Vec3::new(-1.0, -1.0, 1.0)),
                    _ => {}
                },
                None => {}
            }
            if let Some(set_dir) = dir {
                gun_transform.look_at(Vec3::Z, set_dir);
            }
        }
    }
}
