use std::{collections::HashSet, fs::read_to_string};

use rand::seq::IndexedRandom;

use crate::{city::City, player::Player, player_generation::PlayerGenerator, team::Team};

pub struct TeamGenerator {
    id_counter: u64,
    cities: Vec<City>,
    team_names: Vec<String>,
    chosen_cities: HashSet<City>,
    chosen_team_names: HashSet<String>,
}

impl TeamGenerator {
    pub fn new(cities: Vec<City>) -> Self {
        Self {
            id_counter: 0,
            cities: cities,
            team_names: Self::load_team_names(),
            chosen_cities: HashSet::new(),
            chosen_team_names: HashSet::new(),
        }
    }

    fn load_team_names() -> Vec<String> {
        read_to_string("src/data/team_names.txt")
            .expect("Failed to open team names file")
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn random_team_name(&mut self) -> String {
        let mut rng = rand::rng();
        loop {
            let team_name = self
                .team_names
                .choose(&mut rng)
                .expect("Failed to choose team name")
                .clone();
            if !self.chosen_team_names.contains(&team_name) {
                self.chosen_team_names.insert(team_name.clone());
                return team_name;
            }
        }
    }

    fn random_city(&mut self) -> City {
        let mut rng = rand::rng();
        loop {
            let city = self
                .cities
                .choose(&mut rng)
                .expect("Failed to choose city")
                .clone();
            if !self.chosen_cities.contains(&city) {
                self.chosen_cities.insert(city.clone());
                return city;
            }
        }
    }

    pub fn generate_team(&mut self) -> Team {
        let id = self.id_counter;
        self.id_counter += 1;

        let team_name = self.random_team_name();
        let city = self.random_city();
        let abbreviated_name: String = city.city.chars().take(4).collect();
        let abbreviated_name = abbreviated_name.to_uppercase();

        Team {
            id: id,
            name: team_name,
            abbreviated_name: abbreviated_name,
            country: String::from("US"),
            city: city.city,
            state: Some(city.state_id),
            player_ids: vec![],
        }
    }
}
