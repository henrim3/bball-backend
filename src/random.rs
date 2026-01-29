use rand::Rng;

pub fn random_u8() -> u8 {
    rand::rng().random()
}
