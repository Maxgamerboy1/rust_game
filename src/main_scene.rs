use bevy::prelude::{Commands, Component, OrthographicCameraBundle, UiCameraBundle};

#[derive(Component)]
pub struct MainScene;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainScene);
}
