use crate::league_generation::LeagueGenerator;

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
    for player in league.players.values() {
        println!("{:?}", player.position);
    }
}
