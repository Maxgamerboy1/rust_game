use bevy::prelude::*;

#[derive(Component)]
pub struct Gun;

/// angle: -1.0 :: 1.0, anti-clockwise :: clockwise
#[derive(Component)]
pub struct RotationLock(pub f32);

#[derive(Bundle)]
pub struct GunBundle {
    #[bundle]
    pub display: SpriteBundle,
    pub rotation_lock: RotationLock,
}
