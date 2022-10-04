pub mod models;

use crate::gun::models::{Gun, GunBundle, RotationLock};
use bevy::{input::keyboard::KeyboardInput, prelude::*, sprite::Anchor};
use models::*;

pub fn setup_people(mut commands: Commands) {
    let sb = SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(50., 50., 1.)),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(Person)
        .insert_bundle(PersonBundle {
            display: sb,
            movement: MovementLock(false, false, false, false),
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(GunBundle {
                    display: SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            anchor: Anchor::TopCenter,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 2.)
                            .with_scale(Vec3::new(0.5, 1., 1.)),
                        ..Default::default()
                    },
                    rotation_lock: RotationLock::default(),
                })
                .insert(Gun);
        });
}

pub fn handle_keyboard(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut query: Query<&mut MovementLock, With<Person>>,
) {
    if let Ok(mut movement) = query.get_single_mut() {
        for key in keyboard_event.iter() {
            match key.key_code {
                Some(code) => match code {
                    KeyCode::A => movement.0 = key.state.is_pressed(),
                    KeyCode::W => movement.1 = key.state.is_pressed(),
                    KeyCode::D => movement.2 = key.state.is_pressed(),
                    KeyCode::S => movement.3 = key.state.is_pressed(),
                    _ => {}
                },
                None => {}
            }
        }
    }
}

const BASE_SPEED: f32 = 0.98;

pub fn move_person(mut query: Query<(&mut Transform, &MovementLock), With<Person>>) {
    if let Ok((mut transform, movement_lock)) = query.get_single_mut() {
        let mut x_delta = 0.;
        let mut y_delta = 0.;

        // Right-handed, Y-Up coord system
        if movement_lock.0 {
            x_delta -= BASE_SPEED;
        }
        if movement_lock.2 {
            x_delta += BASE_SPEED;
        }

        if movement_lock.1 {
            y_delta += BASE_SPEED;
        }
        if movement_lock.3 {
            y_delta -= BASE_SPEED;
        }

        // BUG: (debug-only) Causing stagger when mouse moves
        transform.translation.x += x_delta;
        transform.translation.y += y_delta;
    }
}

pub fn teardown(p_query: Query<Entity, With<Person>>, mut commands: Commands) {
    println!("Exited!");
    if let Ok(person) = p_query.get_single() {
        commands.entity(person).despawn_recursive();
    }
}
