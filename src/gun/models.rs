use bevy::prelude::*;

#[derive(Component)]
pub struct Gun;

#[derive(Bundle)]
pub struct GunBundle {
    #[bundle]
    pub display: SpriteBundle,
}
