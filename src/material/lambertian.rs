pub use super::{Material, Scatter};
use crate::{
    hit::HitRecord,
    math::{Color, Ray, Vec3},
};
use std::sync::Arc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl From<Lambertian> for Arc<dyn Material> {
    fn from(value: Lambertian) -> Self {
        Arc::new(value)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let mut direction = hit.normal + Vec3::unit_random();
        if direction.near_zero() {
            direction = hit.normal;
        }
        let ray = Ray {
            direction,
            origin: hit.p,
        };
        Some(Scatter {
            color_attenuation: self.albedo,
            ray,
        })
    }
}
