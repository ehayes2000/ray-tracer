use super::Material;
use crate::math::Color;
use std::rc::Rc;

pub struct DiffuseLight {
    pub color: Color,
}

impl DiffuseLight {
    pub fn obj(color: Color) -> Rc<dyn Material> {
        Rc::new(Self { color })
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
