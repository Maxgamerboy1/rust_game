// MARK: top-level module definitions
mod enemy;
mod game_rules;
mod gun;
mod person;
mod rust_game_plugin;
mod scenes;
mod wall;

// use std::env;

use bevy::{prelude::App, DefaultPlugins};
use rust_game_plugin::RustGamePlugin;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RustGamePlugin)
        .run();
}
