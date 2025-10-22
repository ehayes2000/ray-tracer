mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::Color;
use crate::HitRecord;
use crate::Ray;

pub struct Scatter {
    pub color_attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scatter>;
}
