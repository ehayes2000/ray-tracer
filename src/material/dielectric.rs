use super::Material;
use super::Scatter;
use crate::math::random;
use crate::vec3::{dot, unit_vector};
use crate::{HitRecord, Ray, Vec3};
use std::rc::Rc;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn obj(refraction_index: f64) -> Rc<dyn Material> {
        Rc::new(Self::new(refraction_index))
    }

    fn reflectance(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let unit_direction = unit_vector(&r_in.direction);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let direction = if ri * sin_theta > 1.0 || self.reflectance(cos_theta) > random() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        Some(Scatter {
            color_attenuation: attenuation,
            ray: Ray {
                direction: direction,
                origin: rec.p,
            },
        })
    }
}
