use bevy::input::keyboard::KeyboardInput;
use bevy::{prelude::Plugin};
use crate::{gun, enemy, game_rules};
use crate::{main_scene, wall, new_person};

pub struct RustGamePlugin;

impl Plugin for RustGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<KeyboardInput>();
        app.add_startup_system(main_scene::setup);
        app.add_startup_system(new_person::setup_people);
        app.add_startup_system(wall::setup_walls);
        app.add_startup_system(enemy::setup);
        app.add_system(gun::point_to_mouse);
        app.add_system(gun::shoot);
        app.add_system(gun::move_bullet);
        app.add_system(gun::check_bullet_hit_wall);
        app.add_system(gun::check_bullet_hit_enemy);
        app.add_system(gun::check_bullet_lifespan);
        app.add_system(new_person::handle_keyboard);
        app.add_system(new_person::move_person);
        app.add_system(wall::check_wall_collision);
        app.add_system(enemy::move_enemy);
        app.add_system(game_rules::check_win_conditions);
    }
}