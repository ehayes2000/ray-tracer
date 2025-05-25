use super::ray::Ray;
use super::vec3::{Point, Vec3, dot};

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
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hit {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: HitRecord) -> Option<HitRecord>;
}
