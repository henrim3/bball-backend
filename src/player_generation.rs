use csv::{self, ReaderBuilder};
use rand::{self, seq::IndexedRandom};
use rand_distr::{Distribution, Normal, num_traits::Pow};
use serde::Deserialize;
use std::fs::read_to_string;

use crate::player::{Player, PlayerPosition};

const MIN_HEIGHT: f32 = 64.0;
const MAX_HEIGHT: f32 = 91.0;

const MIN_WINGSPAN_DIFF: f32 = -1.0;
const MAX_WINGSPAN_DIFF: f32 = 11.0;

pub struct PlayerGenerator {
    id_counter: u64,
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
            id_counter: 0,
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

    fn random_bmi() -> f32 {
        let mean_bmi = 24.88;
        let std_dev = 2.0;
        let normal = Normal::new(mean_bmi, std_dev).expect("Invalid BMI distribution");
        let mut rng = rand::rng();
        let bmi: f32 = normal.sample(&mut rng);
        bmi.clamp(21.33, 32.0)
    }

    fn random_weight(height_inches: f32) -> f32 {
        let bmi = Self::random_bmi();
        (bmi * (height_inches).pow(2)) / 703 as f32
    }

    fn random_height(position: PlayerPosition) -> f32 {
        let mean_height_inches = position.mean_height_inches();
        let std_dev = 2.0;
        let normal =
            Normal::new(mean_height_inches as f32, std_dev).expect("Invalid height distribution");
        let mut rng = rand::rng();
        let height: f32 = normal.sample(&mut rng);
        height.clamp(MIN_HEIGHT, MAX_HEIGHT)
    }

    fn random_position() -> PlayerPosition {
        let mut rng = rand::rng();
        [
            PlayerPosition::PG,
            PlayerPosition::SG,
            PlayerPosition::SF,
            PlayerPosition::PF,
            PlayerPosition::C,
        ]
        .choose(&mut rng)
        .copied()
        .expect("Invalid random position")
    }

    pub fn generate_player(&mut self) -> Player {
        let id = self.id_counter;
        self.id_counter += 1;

        // Name
        let first_name = &self.first_names[rand::random_range(0..self.first_names.len())];
        let middle_name = &self.first_names[rand::random_range(0..self.first_names.len())];
        let last_name = &self.last_names[rand::random_range(0..self.last_names.len())];

        let position = Self::random_position();

        // Physicals
        let height_inches = Self::random_height(position);
        let wingspan_diff = rand::random_range(MIN_WINGSPAN_DIFF..=MAX_WINGSPAN_DIFF); // TODO: use distribution
        let wingspan_inches = height_inches + wingspan_diff;
        let weight_lbs = Self::random_weight(height_inches);

        let city_state = &self.city_states[rand::random_range(0..self.city_states.len())];

        Player {
            id,
            team_id: None,
            first_name: first_name.to_string(),
            middle_name: middle_name.to_string(),
            last_name: last_name.to_string(),
            position: position,
            country: String::from("US"),
            city: city_state.city.to_string(),
            state: Some(city_state.state_id.to_string()),
            height_inches,
            wingspan_inches,
            weight_lbs,
        }
    }
}
