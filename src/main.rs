mod league;
mod player;
mod player_generation;
mod player_stats;
mod team;

use crate::player_generation::generate_player;

fn main() {
    let player = generate_player(1);
    println!("{:?}", player);
}
