mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{
    hit::HitRecord,
    math::{Color, Point, Ray},
};
use std::fmt::Debug;

use std::sync::Arc;

pub struct Scatter {
    pub color_attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Debug + Send + Sync + 'static {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scatter>;
    fn emit(&self, _: Point) -> Color {
        Color::zero()
    }
}

pub trait Materialify {
    fn materialify(self) -> Arc<dyn Material>;
}

impl<T: Material> Materialify for T {
    fn materialify(self) -> Arc<dyn Material> {
        Arc::new(self)
    }
}

impl Materialify for Arc<dyn Material> {
    fn materialify(self) -> Arc<dyn Material> {
        self
    }
}
