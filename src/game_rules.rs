use bevy::{
    ecs::schedule::StateError,
    prelude::{Color, Commands, Query, ResMut, State, Transform, With},
    text::{HorizontalAlign, Text, Text2dBundle, TextAlignment, TextStyle, VerticalAlign},
};

use crate::{enemy::Enemy, rust_game_plugin::AppState};

pub fn check_win_conditions(
    q_enemy: Query<(), With<Enemy>>,
    mut res_app_state: ResMut<State<AppState>>,
) {
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

pub fn display_win_screen(mut commands: Commands) {
    println!("You win!!");
    let mut winning_text = Text2dBundle::default();
    winning_text.text = Text::from_section(
        "You Win!!".to_string(),
        TextStyle {
            color: Color::WHITE,
            font_size: 48.0,
            ..Default::default()
        },
    )
    .with_alignment(TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    });
    winning_text.transform = Transform::from_xyz(50.0, 50.0, 0.0);

    commands.spawn_bundle(winning_text);
}
