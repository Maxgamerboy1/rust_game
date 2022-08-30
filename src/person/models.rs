use bevy::prelude::*;

#[derive(Bundle)]
pub struct PersonBundle {
    #[bundle]
    pub display: SpriteBundle,
    pub movement: MovementLock,
}

#[derive(Component)]
pub struct Person;

/**
Order of bools: L,T,R,B
*/
#[derive(Component)]
pub struct MovementLock(pub bool, pub bool, pub bool, pub bool);

#[derive(Component)]
struct Display(pub SpriteBundle);