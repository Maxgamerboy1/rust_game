use crate::{enemy, game_rules, gun};
use crate::{main_scene, new_person, wall};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::Plugin;
use bevy::prelude::{App, SystemSet};

pub struct RustGamePlugin;

impl Plugin for RustGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyboardInput>();
        app.add_state(AppState::InGame);
        setup_states(app);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Win,
}

fn setup_states(app: &mut App) {
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(wall::setup_walls)
            .with_system(new_person::setup_people)
            .with_system(main_scene::setup)
            .with_system(enemy::setup),
    );
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(gun::point_to_mouse)
            .with_system(gun::shoot)
            .with_system(gun::move_bullet)
            .with_system(gun::check_bullet_hit_wall)
            .with_system(gun::check_bullet_hit_enemy)
            .with_system(gun::check_bullet_lifespan)
            .with_system(new_person::handle_keyboard)
            .with_system(new_person::move_person)
            .with_system(wall::check_wall_collision)
            .with_system(enemy::move_enemy)
            .with_system(game_rules::check_win_conditions),
    );
    app.add_system_set(
        SystemSet::on_enter(AppState::Win).with_system(game_rules::display_win_screen),
    );
}
