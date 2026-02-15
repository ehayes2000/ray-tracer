use super::Material;
use crate::math::Color;

pub struct DiffuseLight {
    pub color: Color,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for DiffuseLight {
    fn emit(&self, _: crate::math::Point) -> Color {
        self.color
    }

    fn scatter(
        &self,
        _: &crate::math::Ray,
        _: &crate::hittable::HitRecord,
    ) -> Option<super::Scatter> {
        None
    }
}
