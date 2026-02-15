use super::Float;
use rand::Rng;

#[allow(clippy::unnecessary_cast)]
pub fn degrees_to_radians(degrees: Float) -> Float {
    (degrees as f64 * std::f64::consts::PI / 180.0) as Float
}

pub fn random() -> Float {
    rand::rng().random()
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..=max)
}

pub fn random_float(min: Float, max: Float) -> Float {
    random() * (max - min) + min
}
