use super::hittable::{Hit, HitRecord};
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::{Point, dot};
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new_boxed(center: Point, radius: f64) -> Box<dyn Hit> {
        Box::new(Self { center, radius })
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.len_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let descriminant = h * h - a * c;
        if descriminant <= 0. {
            return None;
        }
        let sqrtd = descriminant.sqrt();
        Some((h - sqrtd) / a)
            // if this root is in bounds use it
            .and_then(|root| {
                if ray_t.surrounds(root) {
                    Some(root)
                } else {
                    None
                }
            })
            // or else check the other root
            .or_else(|| {
                let root = (h + sqrtd) / a;
                if ray_t.surrounds(root) {
                    Some(root)
                } else {
                    None
                }
            })
            // using the bounded root return a HitRecord
            .map(|root| {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                HitRecord::with_normal(p, r, normal, root)
            })
    }
}
