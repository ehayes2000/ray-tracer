pub use super::{Material, Scatter};
use crate::hittable::HitRecord;
use crate::math::Ray;
use crate::math::{Color, Vec3};
use std::sync::Arc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn obj(albedo: Color) -> Arc<dyn Material> {
        let lambertian = Self::new(albedo);
        Arc::new(lambertian)
    }
}

impl Into<Arc<dyn Material>> for Lambertian {
    fn into(self) -> Arc<dyn Material> {
        Arc::new(self)
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
