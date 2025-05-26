use super::interval::Interval;
use super::ray::Ray;
use super::vec3::{Point, Vec3, dot};

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn zero() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn with_normal(p: Vec3, r: &Ray, u_out_norm: Vec3, t: f64) -> Self {
        let front_face = dot(&r.direction, &u_out_norm) < 0.0;
        let normal = if front_face { u_out_norm } else { -u_out_norm };
        Self {
            front_face,
            p,
            normal,
            t,
        }
    }
}
