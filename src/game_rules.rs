use bevy::{prelude::{Query, With, ResMut, State}, ecs::schedule::StateError};

use crate::{enemy::Enemy, rust_game_plugin::AppState};

pub fn check_win_conditions(q_enemy: Query<(), With<Enemy>>, mut res_app_state: ResMut<State<AppState>>) {
    if q_enemy.is_empty() {
        println!("You've Won!");
        if let Err(state_err) = res_app_state.set(AppState::Win) {
            match state_err {
                StateError::AlreadyInState => todo!(),
                StateError::StateAlreadyQueued => todo!(),
                StateError::StackEmpty => todo!(),
            }
        };
    }
}

pub fn display_win_screen() {
    println!("You win!!")
}