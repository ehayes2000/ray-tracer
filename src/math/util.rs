use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random() -> f32 {
    rand::rng().random()
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..=max)
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    random() * (max - min) + min
}
