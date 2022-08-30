use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
/// lifespan_boundary (s), lifespan (s)
pub struct BulletLifespan(pub f32, pub f32);

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub display: SpriteBundle,
}