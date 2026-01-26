#[derive(Debug, Clone)]
pub struct Team {
    pub id: u64,

    // Basics
    pub name: String,
    pub abbreviated_name: String,

    // Location
    pub country: String,
    pub city: String,

    pub player_ids: Vec<u64>,
}
