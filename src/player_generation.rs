use rand;
use std::fs::read_to_string;

use crate::player::{Player, PlayerPosition};

fn load_first_names() -> Vec<String> {
    read_to_string("src/data/first_names.txt")
        .expect("Failed to open last names file")
        .lines()
        .map(str::to_owned)
        .collect()
}

fn load_last_names() -> Vec<String> {
    read_to_string("src/data/last_names.txt")
        .expect("Failed to open last names file")
        .lines()
        .map(str::to_owned)
        .collect()
}

pub fn generate_player(id: u64) -> Player {
    let first_names = load_first_names();
    let last_names = load_last_names();

    let first_name = &first_names[rand::random_range(0..first_names.len())];
    let middle_name = &first_names[rand::random_range(0..first_names.len())];
    let last_name = &last_names[rand::random_range(0..last_names.len())];

    Player {
        id,
        team_id: None,
        first_name: first_name.to_string(),
        middle_name: middle_name.to_string(),
        last_name: last_name.to_string(),
        position: PlayerPosition::PG,
        country: String::from("US"),
        city: String::from("New York"),
        state: Some(String::from("NY")),
        height_inches: 72,
        wingspan_inches: 72,
    }
}
