use bevy::ecs::bundle::Bundle;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::prelude::{EventReader, Query, With};
use bevy::sprite::{Anchor, Sprite, SpriteBundle};
use bevy::{ecs::component::Component, input::keyboard::KeyboardInput};

use crate::gun::{Gun, GunBundle};

#[derive(Bundle)]
pub struct PersonBundle {
    #[bundle]
    display: SpriteBundle,
    movement: MovementLock,
}

#[derive(Component)]
pub struct Person;

/**
Order of bools: L,T,R,B
*/
#[derive(Component)]
pub struct MovementLock(pub bool, pub bool, pub bool, pub bool);

#[derive(Component)]
struct Display(pub SpriteBundle);

const BASE_SPEED: f32 = 0.94;

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
                })
                .insert(Gun);
        });
}

pub fn handle_keyboard(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut query: Query<&mut MovementLock, With<Person>>,
) {
    let mut movement = query.single_mut();
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

pub fn move_person(mut query: Query<(&mut Transform, &MovementLock), With<Person>>) {
    let (mut transform, movement_lock) = query.single_mut();
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
