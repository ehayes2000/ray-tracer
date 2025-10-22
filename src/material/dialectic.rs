use super::Material;
use super::Scatter;
use crate::vec3::unit_vector;
use crate::{HitRecord, Ray, Vec3};
use std::rc::Rc;

pub struct Dialectic {
    refraction_index: f64,
}

impl Dialectic {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn obj(refraction_index: f64) -> Rc<dyn Material> {
        Rc::new(Self::new(refraction_index))
    }
}

impl Material for Dialectic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(&r_in.direction);
        let refracted = unit_direction.refract(&rec.normal, ri);
        let scattered = Ray {
            direction: refracted,
            origin: rec.p,
        };

        Some(Scatter {
            color_attenuation: attenuation,
            ray: scattered,
        })
    }
}
