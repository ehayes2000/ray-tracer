use super::Material;
use super::Scatter;
use crate::math::Ray;
use crate::math::{Color, Vec3, dot, unit_vector};
use std::rc::Rc;

pub struct Metal {
    albedo: Color,
    roughness: f32,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Self { albedo, roughness }
    }

    pub fn obj(albedo: Color, roughness: f32) -> Rc<dyn Material> {
        Rc::new(Self::new(albedo, roughness))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &crate::hittable::HitRecord) -> Option<Scatter> {
        let reflected = ray_in.direction.reflect(&hit.normal);
        let reflected = unit_vector(&reflected) + (self.roughness * Vec3::unit_random());
        let ray = Ray {
            direction: reflected,
            origin: hit.p,
        };
        if dot(&ray.direction, &hit.normal) > 0.0 {
            Some(Scatter {
                color_attenuation: self.albedo,
                ray,
            })
        } else {
            None
        }
    }
}
