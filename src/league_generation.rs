use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::{
    city::load_cities,
    league::League,
    player::{Player, PlayerPosition},
    player_generation::PlayerGenerator,
    player_stats::PlayerStats,
    team::Team,
    team_generation::TeamGenerator,
};

pub struct LeagueGenerator {
    player_generator: PlayerGenerator,
    team_generator: TeamGenerator,
}

impl LeagueGenerator {
    pub fn new() -> Self {
        let cities = load_cities();
        Self {
            player_generator: PlayerGenerator::new(cities.clone()),
            team_generator: TeamGenerator::new(cities),
        }
    }

    fn generate_team_players(&mut self) -> Vec<Player> {
        let mut players: Vec<Player> = vec![];
        for position in PlayerPosition::iter() {
            for _ in 0..3 {
                players.push(self.player_generator.generate_player_by_position(position));
            }
        }
        players
    }

    pub fn generate_league(&mut self, n_teams: i32) -> League {
        let mut teams: HashMap<u64, Team> = HashMap::new();
        let mut players: HashMap<u64, Player> = HashMap::new();
        let mut player_stats: HashMap<u64, PlayerStats> = HashMap::new();

        for _ in 0..n_teams {
            let mut team = self.team_generator.generate_team();

            let team_players = self.generate_team_players();
            team.player_ids = team_players.iter().map(|p| p.id).collect();

            for player in team_players {
                player_stats.insert(
                    player.id,
                    PlayerStats {
                        games_played: 0,
                        points: 0,
                        assists: 0,
                        rebounds: 0,
                    },
                );
                players.insert(player.id, player);
            }

            teams.insert(team.id, team);
        }

        League {
            teams: teams,
            players: players,
            player_stats: player_stats,
        }
    }
}
