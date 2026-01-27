#[derive(Debug, Clone)]
pub enum PlayerPosition {
    PG,
    SG,
    SF,
    PF,
    C,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub team_id: Option<u64>,

    // Basics
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub position: PlayerPosition,

    // Origin
    pub country: String,
    pub city: String,
    pub state: Option<String>,

    // Measurements
    // TODO: move into separate class, add standing reach, vertical, etc.
    pub height_inches: u8,
    pub wingspan_inches: u8,
    pub weight_lbs: u16,
}
