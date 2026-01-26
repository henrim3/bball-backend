use csv::{self, ReaderBuilder};
use rand;
use serde::Deserialize;
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
    city_states: Vec<CityState>,
}

#[derive(Debug, Deserialize)]
struct CityState {
    city: String,
    state_id: String,
}

impl PlayerGenerator {
    pub fn new() -> Self {
        Self {
            first_names: Self::load_first_names(),
            last_names: Self::load_last_names(),
            city_states: Self::load_cities(),
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

    fn load_cities() -> Vec<CityState> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path("src/data/us_cities/us_cities.csv")
            .expect("Failed to read US cities file");

        let mut result = Vec::new();

        for record in reader.deserialize::<CityState>() {
            let record = record.expect("Failed to read city/state record");
            result.push(record);
        }

        result
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

        let city_state = &self.city_states[rand::random_range(0..self.city_states.len())];

        Player {
            id,
            team_id: None,
            first_name: first_name.to_string(),
            middle_name: middle_name.to_string(),
            last_name: last_name.to_string(),
            position: PlayerPosition::PG,
            country: String::from("US"),
            city: city_state.city.to_string(),
            state: Some(city_state.state_id.to_string()),
            height_inches,
            wingspan_inches,
            weight_lbs,
        }
    }
}
