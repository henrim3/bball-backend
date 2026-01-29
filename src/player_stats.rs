#[derive(Debug, Default, Clone)]
pub struct PlayerStats {
    pub games_played: u32,
    pub points: u32,
    pub assists: u32,
    pub rebounds: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            points: 0,
            assists: 0,
            rebounds: 0,
        }
    }
}
