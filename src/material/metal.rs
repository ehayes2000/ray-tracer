use super::Material;
use super::Scatter;
use crate::Float;
use crate::math::Ray;
use crate::math::{Color, Vec3, dot, unit_vector};
use std::sync::Arc;

pub struct Metal {
    albedo: Color,
    roughness: Float,
}

impl Metal {
    pub fn new(albedo: Color, roughness: Float) -> Self {
        Self { albedo, roughness }
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
