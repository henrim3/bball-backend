use std::collections::HashMap;

use crate::player::Player;
use crate::player_stats::PlayerStats;
use crate::team::Team;

#[derive(Debug)]
pub struct League {
    pub teams: HashMap<u64, Team>,
    pub players: HashMap<u64, Player>,
    pub player_stats: HashMap<u64, PlayerStats>,
}
