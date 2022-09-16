use crate::{enemy, game_rules, gun};
use crate::{main_scene, person, wall};
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
    Loose,
}

fn setup_states(app: &mut App) {
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(wall::setup_walls)
            .with_system(person::setup_people)
            .with_system(main_scene::setup)
            .with_system(enemy::setup),
    );
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(gun::set_aim_lock)
            .with_system(gun::handle_aim_lock)
            .with_system(gun::shoot)
            .with_system(gun::bullet::move_bullet)
            .with_system(gun::check_bullet_hit_wall)
            .with_system(gun::bullet::check_bullet_hit_enemy)
            .with_system(gun::bullet::check_bullet_lifespan)
            .with_system(person::handle_keyboard)
            .with_system(person::move_person)
            .with_system(wall::check_wall_collision)
            .with_system(enemy::move_enemy)
            .with_system(enemy::check_player_collision)
            .with_system(game_rules::check_win_conditions)
            .with_system(game_rules::check_loose_conditions),
    );
    app.add_system_set(
        SystemSet::on_enter(AppState::Win).with_system(game_rules::display_win_screen),
    );
    app.add_system_set(
        SystemSet::on_update(AppState::Win).with_system(game_rules::win_button_system),
    );

    app.add_system_set(
        SystemSet::on_enter(AppState::Loose).with_system(game_rules::display_loose_screen),
    );
    app.add_system_set(
        SystemSet::on_update(AppState::Loose).with_system(game_rules::loose_button_system),
    );
}
