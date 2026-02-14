use super::Material;
use super::Scatter;
use crate::Float;
use crate::hittable::HitRecord;
use crate::math::Ray;
use crate::math::random;
use crate::math::{Vec3, dot, unit_vector};
use std::sync::Arc;

pub struct Dielectric {
    refraction_index: Float,
}

impl Dielectric {
    pub fn new(refraction_index: Float) -> Self {
        Self { refraction_index }
    }

    pub fn obj(refraction_index: Float) -> Arc<dyn Material> {
        Arc::new(Self::new(refraction_index))
    }

    fn reflectance(&self, cosine: Float) -> Float {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * Float::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let unit_direction = unit_vector(&r_in.direction);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let cos_theta = Float::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = Float::sqrt(1.0 - cos_theta * cos_theta);
        let direction = if ri * sin_theta > 1.0 || self.reflectance(cos_theta) > random() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        Some(Scatter {
            color_attenuation: Vec3(1.0, 1.0, 1.0),
            ray: Ray {
                direction,
                origin: rec.p,
            },
        })
    }
}
