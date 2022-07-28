// MARK: module definitions
mod rust_game_plugin;
mod main_scene;
mod gun;
mod new_person;
mod wall;
mod enemy;
mod game_rules;

use bevy::{prelude::App, DefaultPlugins};
use rust_game_plugin::RustGamePlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(RustGamePlugin)
    .run();
}
