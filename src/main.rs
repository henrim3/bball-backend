use crate::city::{City, load_cities};
use crate::player_generation::PlayerGenerator;
use crate::team_generation::TeamGenerator;

mod city;
mod league;
mod player;
mod player_generation;
mod player_stats;
mod team;
mod team_generation;

fn main() {
    let cities = load_cities();
    let mut pg = PlayerGenerator::new(cities.clone());
    let mut tg = TeamGenerator::new(cities);
    let team = tg.generate_team();
    println!("{:?}", team);
}
