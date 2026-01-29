use crate::city::{City, load_cities};
use crate::league_generation::LeagueGenerator;
use crate::player_generation::PlayerGenerator;
use crate::team_generation::TeamGenerator;

mod city;
mod league;
mod league_generation;
mod player;
mod player_generation;
mod player_stats;
mod team;
mod team_generation;

fn main() {
    let mut league_generator = LeagueGenerator::new();
    let league = league_generator.generate_league(1);
    println!("{:?}", league);
}
