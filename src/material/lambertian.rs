pub use super::{Material, Scatter};
use crate::{Color, HitRecord, Ray, vec3::Vec3};
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_rc(albedo: Color) -> Rc<dyn Material> {
        let lambertian = Self::new(albedo);
        Rc::new(lambertian)
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
