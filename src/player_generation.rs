use rand;
use std::fs::read_to_string;

use crate::player::{Player, PlayerPosition};

const MIN_HEIGHT: u8 = 64;
const MAX_HEIGHT: u8 = 91;

const MIN_WINGSPAN_DIFF: i8 = -1;
const MAX_WINGSPAN_DIFF: i8 = 11;

const MIN_WEIGHT: u16 = 135;
const MAX_WEIGHT: u16 = 310;

pub struct PlayerGenerator {
    first_names: Vec<String>,
    last_names: Vec<String>,
}

impl PlayerGenerator {
    pub fn new() -> Self {
        Self {
            first_names: Self::load_first_names(),
            last_names: Self::load_last_names(),
        }
    }

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

    pub fn generate_player(&mut self, id: u64) -> Player {
        // Name
        let first_name = &self.first_names[rand::random_range(0..self.first_names.len())];
        let middle_name = &self.first_names[rand::random_range(0..self.first_names.len())];
        let last_name = &self.last_names[rand::random_range(0..self.last_names.len())];

        // Physicals
        let height_inches = rand::random_range(MIN_HEIGHT..=MAX_HEIGHT);
        let wingspan_diff = rand::random_range(MIN_WINGSPAN_DIFF..=MAX_WINGSPAN_DIFF);
        let wingspan_inches = height_inches.saturating_add_signed(wingspan_diff);
        let weight_lbs = rand::random_range(MIN_WEIGHT..=MAX_WEIGHT);

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
            height_inches,
            wingspan_inches,
            weight_lbs,
        }
    }
}
