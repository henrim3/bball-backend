use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct City {
    pub city: String,
    pub state_id: String,
}

pub fn load_cities() -> Vec<City> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("src/data/us_cities/us_cities.csv")
        .expect("Failed to read US cities file");

    let mut result = Vec::new();

    for record in reader.deserialize::<City>() {
        let record = record.expect("Failed to read city/state record");
        result.push(record);
    }

    result
}
