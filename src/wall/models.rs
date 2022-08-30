use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    pub sp: SpriteBundle,
}