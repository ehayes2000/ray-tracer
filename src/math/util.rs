use super::{Float, Vec3};
use crate::v3;
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

pub fn lerp(v1: Vec3, v2: Vec3, f: Float) -> Vec3 {
    v3!(
        v1.0 + (v2.0 - v1.0) * f,
        v1.1 + (v2.1 - v1.1) * f,
        v1.2 + (v2.2 - v1.2) * f
    )
}
