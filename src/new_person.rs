use bevy::ecs::bundle::Bundle;
use bevy::math::{Vec3};
use bevy::prelude::*;
use bevy::prelude::{EventReader, Query, With};
use bevy::sprite::{Sprite, SpriteBundle, Anchor};
use bevy::{ecs::component::Component, input::keyboard::KeyboardInput};

use crate::gun::{Gun, GunBundle};

#[derive(Bundle)]
pub struct PersonBundle {
    #[bundle]
    display: SpriteBundle,
    movement: MovementDelta,
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct MovementDelta(pub f32, pub f32);

#[derive(Component)]
struct Display(pub SpriteBundle);

const BASE_SPEED: f32 = 2.0;

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
            movement: MovementDelta(0.0, 0.0),
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
                        transform: Transform::from_xyz(0., 0., 2.).with_scale(Vec3::new(0.5, 1., 1.)),
                        ..Default::default()
                    },
                })
                .insert(Gun);
        });
}

impl Person {

    pub fn handle_keyboard(
        mut keyboard_event: EventReader<KeyboardInput>,
        mut query: Query<&mut MovementDelta, With<Person>>,
    ) {
        let mut movement = query.single_mut();
        for key in keyboard_event.iter() {
            match key.key_code {
                Some(code) => {
                    // println!("KeyCode: {:?}, isPressed: {}", code, key.state.is_pressed());
                    if key.state.is_pressed() {
                        if code == KeyCode::A {
                            movement.0 = -BASE_SPEED;
                        }
                        if code == KeyCode::D {
                            movement.0 = BASE_SPEED;
                        }
                        if code == KeyCode::W {
                            movement.1 = BASE_SPEED;
                        }
                        if code == KeyCode::S {
                            movement.1 = -BASE_SPEED;
                        }
                    } else {
                        // BUG: quickly changing same-axis direction causes pause.
                        if code == KeyCode::A {
                            movement.0 = 0.0;
                        }
                        if code == KeyCode::D {
                            movement.0 = 0.0;
                        }
                        if code == KeyCode::W {
                            movement.1 = 0.0;
                        }
                        if code == KeyCode::S {
                            movement.1 = 0.0;
                        }
                    }
                }
                None => {}
            }
        }
    }

    pub fn draw(mut query: Query<(&mut Transform, &MovementDelta), With<Person>>) {
        let (mut transform, trans) = query.single_mut();
        transform.translation.x += trans.0;
        transform.translation.y += trans.1;
    }
}
