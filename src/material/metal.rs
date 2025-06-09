use super::Material;
use super::Scatter;
use crate::{Color, Ray};
use std::rc::Rc;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_rc(albedo: Color) -> Rc<dyn Material> {
        Rc::new(Self::new(albedo))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &crate::hittable::HitRecord) -> Option<Scatter> {
        let reflected = ray_in.direction.reflect(&hit.normal);
        let ray = Ray {
            direction: reflected,
            origin: hit.p,
        };
        Some(Scatter {
            color_attenuation: self.albedo,
            ray,
        })
    }
}
