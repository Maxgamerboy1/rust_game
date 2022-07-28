use bevy::{
    core::Time,
    hierarchy::Parent,
    input::mouse::MouseButtonInput,
    math::{Quat, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        Bundle, Camera, Color, Commands, Component, Entity, EventReader, GlobalTransform,
        MouseButton, Query, Res, Transform, With,
    },
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
    window::Windows,
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
                        .clone()
                        .with_scale(Vec3::new(12.0, 20.0, 1.0))
                        .into();

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

pub fn point_to_mouse(
    windows: Res<Windows>,
    mut q_gun_child: Query<(&Parent, &mut Transform), With<Gun>>,
    q_parent: Query<&GlobalTransform, With<Person>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainScene>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(cursor_in_screen_pos) = window.cursor_position() {
        let (parent, mut pos) = q_gun_child.single_mut();
        if let Ok(parent_global_transform) = q_parent.get(parent.0) {
            let absolute = parent_global_transform.translation + pos.translation;

            let (camera, camera_transform) = q_camera.single();
            // get the size of the window
            let window_size = get_window_size(window);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (cursor_in_screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world = get_inverse_projection_matrix(camera_transform, camera);

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            let to_pointer = absolute.xy() - world_pos;
            let to_pointer_norm = to_pointer.normalize();
            let rotate_to_pointer = Quat::from_rotation_arc(Vec3::Y, to_pointer_norm.extend(0.));

            pos.rotation = rotate_to_pointer;
        }
    }
}

fn get_inverse_projection_matrix(
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> bevy::math::Mat4 {
    camera_transform.compute_matrix() * camera.projection_matrix.inverse()
}

fn get_window_size(window: &bevy::window::Window) -> Vec2 {
    Vec2::new(window.width() as f32, window.height() as f32)
}
