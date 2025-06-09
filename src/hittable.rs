use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{Point, Vec3, dot};
use std::rc::Rc;

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn with_normal(
        p: Vec3,
        r: &Ray,
        u_out_norm: Vec3,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = dot(&r.direction, &u_out_norm) < 0.0;
        let normal = if front_face { u_out_norm } else { -u_out_norm };
        Self {
            front_face,
            p,
            normal,
            t,
            material,
        }
    }
}
