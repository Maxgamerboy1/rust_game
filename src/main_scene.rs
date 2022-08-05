use bevy::prelude::{Commands, Component, Camera2dBundle, UiCameraConfig};

#[derive(Component)]
pub struct MainScene;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(UiCameraConfig {
            show_ui: true
        })
        .insert(MainScene);
}
