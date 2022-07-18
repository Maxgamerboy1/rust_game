use bevy::input::keyboard::KeyboardInput;
use bevy::{prelude::Plugin};
use crate::gun::Gun;
use crate::new_person::Person;
use crate::{main_scene, wall, new_person};

pub struct RustGamePlugin;

impl Plugin for RustGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<KeyboardInput>();
        app.add_startup_system(main_scene::setup);
        app.add_startup_system(new_person::setup_people);
        app.add_startup_system(wall::setup_walls);
        app.add_system(Gun::point_to_mouse);
        app.add_system(Person::handle_keyboard);
        app.add_system(Person::draw);
        app.add_system(wall::check_wall_collision);
    }
}