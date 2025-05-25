use super::hittable::{Hit, HitRecord};
use super::ray::Ray;
use super::vec3::Vec3;

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
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut any_hit = None::<HitRecord>;
        let mut closest_so_far = ray_tmax;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = hit.t;
                any_hit = Some(hit);
            }
        }
        any_hit
    }
}
