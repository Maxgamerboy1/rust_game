use bevy::{ecs::schedule::StateError, prelude::*};

use crate::{enemy::models::Enemy, person::models::Person, rust_game_plugin::AppState};

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

pub fn check_loose_conditions(
    q_enemy: Query<(), With<Person>>,
    mut res_app_state: ResMut<State<AppState>>,
) {
    if q_enemy.is_empty() {
        println!("You've lost =(");
        if let Err(state_err) = res_app_state.set(AppState::Loose) {
            match state_err {
                StateError::AlreadyInState => todo!(),
                StateError::StateAlreadyQueued => todo!(),
                StateError::StackEmpty => todo!(),
            }
        };
    }
}

pub fn display_win_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        text: Text::from_section(
            "You Win!!".to_string(),
            TextStyle {
                color: Color::GOLD,
                font_size: 48.0,
                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
            },
        )
        .with_alignment(TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        }),
        ..Default::default()
    });
    commands.spawn_bundle(TextBundle {
        text: Text::from_section(
            "Play again?".to_string(),
            TextStyle {
                color: Color::ANTIQUE_WHITE,
                font_size: 40.0,
                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
            },
        )
        .with_alignment(TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        }),
        transform: Transform::from_xyz(0.0, -10.0, 0.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: bevy::ui::UiColor(Color::PURPLE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Play again?".to_string(),
                    TextStyle {
                        color: Color::ANTIQUE_WHITE,
                        font_size: 40.0,
                        font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
                transform: Transform::from_xyz(0.0, -10.0, 0.0),
                ..Default::default()
            });
        });
}

pub fn win_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut res_app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => match res_app_state.set(AppState::InGame) {
                Ok(_) => {}
                Err(_) => {}
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn loose_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut res_app_state: ResMut<State<AppState>>,
) {
}

pub fn display_loose_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: bevy::ui::UiColor(Color::PURPLE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "You lost, play again?".to_string(),
                    TextStyle {
                        color: Color::ANTIQUE_WHITE,
                        font_size: 40.0,
                        font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
                transform: Transform::from_xyz(0.0, -10.0, 0.0),
                ..Default::default()
            });
        });
}
