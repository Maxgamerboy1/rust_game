use bevy::{
    math::Vec3,
    prelude::{Bundle, Color, Commands, Component, Query, Transform, With, Without, Changed},
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite, SpriteBundle,
    },
};

use crate::new_person::Person;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sp: SpriteBundle,
}

pub fn setup_walls(mut commands: Commands) {
    commands.spawn().insert(Wall).insert_bundle(WallBundle {
        sp: SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                ..Default::default()
            },
            transform: Transform::from_xyz(-50., -80., 0.).with_scale(Vec3::new(30., 60., 0.)),
            ..Default::default()
        },
    });

    commands.spawn().insert(Wall).insert_bundle(WallBundle {
        sp: SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                ..Default::default()
            },
            transform: Transform::from_xyz(30., 50., 0.).with_scale(Vec3::new(34., 12., 0.)),
            ..Default::default()
        },
    });
}

pub fn check_wall_collision(
    mut q_person: Query<&mut Transform, (With<Person>, Changed<Transform>)>,
    q_wall: Query<&Transform, (With<Wall>, Without<Person>)>,
) {
    let mut person_transform = q_person.single_mut();
    for wall_transform in q_wall.iter() {
        if let Some(collision) = collide(
            person_transform.translation,
            person_transform.scale.truncate(),
            wall_transform.translation,
            wall_transform.scale.truncate(),
        ) {
            let mut x_delta = 0.;
            let mut y_delta = 0.;

            match collision {
                Collision::Left => x_delta -= 2.,
                Collision::Right => x_delta += 2.,
                Collision::Top => y_delta += 2.,
                Collision::Bottom => y_delta -= 2.,
                _ =>{}
            }

            // println!("x_delta: {}  y_delta: {}", x_delta, y_delta);

            person_transform.translation.x += x_delta;
            person_transform.translation.y += y_delta;
        }
    }
}
