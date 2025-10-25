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
    pub t: f64,
    pub normal: Vec3,
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

pub struct HittableList {
    pub objects: Vec<Box<dyn Hit>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object)
    }
}

impl<T> Hit for T
where
    T: std::ops::Deref<Target = HittableList>,
{
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut any_hit = None::<HitRecord>;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                any_hit = Some(hit);
            }
        }
        any_hit
    }
}
