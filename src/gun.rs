use bevy::{
    hierarchy::Parent,
    math::{Quat, Vec2, Vec3, Vec3Swizzles},
    prelude::{Bundle, Camera, Component, GlobalTransform, Query, Res, Transform, With},
    sprite::SpriteBundle,
    window::Windows,
};
use cached::proc_macro::once;

use crate::{main_scene::MainScene, new_person::Person};

#[derive(Component)]
pub struct Gun;

#[derive(Bundle)]
pub struct GunBundle {
    #[bundle]
    pub display: SpriteBundle,
}

impl Gun {
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
                let rotate_to_pointer =
                    Quat::from_rotation_arc(Vec3::Y, to_pointer_norm.extend(0.));

                // println!("gunpos: {}", pos.translation);

                // BUG: moving cursor causes parent to slow down when moving
                pos.rotation = rotate_to_pointer;
            }
        }
    }
}

#[once]
fn get_inverse_projection_matrix(
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> bevy::math::Mat4 {
    camera_transform.compute_matrix() * camera.projection_matrix.inverse()
}

#[once]
fn get_window_size(window: &bevy::window::Window) -> Vec2 {
    Vec2::new(window.width() as f32, window.height() as f32)
}
