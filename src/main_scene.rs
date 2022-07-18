use bevy::prelude::{Commands, OrthographicCameraBundle, Component};

#[derive(Component)]
pub struct MainScene;

pub fn setup(mut commands: Commands) {
  commands.spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d())
    .insert(MainScene);
}