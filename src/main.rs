use crate::player_generation::PlayerGenerator;

mod league;
mod player;
mod player_generation;
mod player_stats;
mod team;

fn main() {
    let mut pg = PlayerGenerator::new();
    let player = pg.generate_player();
    println!("{:?}", player);
}
