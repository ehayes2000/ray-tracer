use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random() -> f64 {
    rand::rng().random()
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    random() * (max - min) + min
}
