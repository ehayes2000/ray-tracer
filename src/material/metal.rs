use super::Material;
use super::Scatter;
use crate::vec3::unit_vector;
use crate::vec3::{Vec3, dot};
use crate::{Color, Ray};
use std::rc::Rc;

pub struct Metal {
    albedo: Color,
    roughness: f64,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self {
        Self { albedo, roughness }
    }

    pub fn obj(albedo: Color, roughness: f64) -> Rc<dyn Material> {
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
