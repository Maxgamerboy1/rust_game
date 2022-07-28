use bevy::prelude::{Query, With};

use crate::enemy::Enemy;

pub fn check_win_conditions(q_enemy: Query<(), With<Enemy>>) {
    if q_enemy.is_empty() {
        println!("You've Won!");
    }
}