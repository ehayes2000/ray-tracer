use crate::{
    aabb::Aabb,
    hit::*,
    math::{Interval, Ray},
};
use std::{fmt::Debug, sync::Arc};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hit>>,
    bbox: Aabb,
}

impl Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HittableList")
            .field("bbox", &self.bbox)
            .field("objects", &format!("[{} objects]", self.objects.len()))
            .finish()
    }
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hit>>) -> Self {
        let bbox = objects
            .iter()
            .fold(Aabb::empty(), |acc, o| acc.union(o.bounding_box()));
        Self { bbox, objects }
    }
    pub fn empty() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::empty(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push(mut self, object: Box<dyn Hit>) -> Self {
        self.bbox = self.bbox.union(object.bounding_box());
        self.objects.push(object);
        self
    }
}

impl Hit for HittableList {
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub trait Hitify {
    fn hitify(self) -> Arc<dyn Hit>;
}

impl<T: Hit> Hitify for T {
    fn hitify(self) -> Arc<dyn Hit> {
        Arc::new(self)
    }
}

impl Hitify for Arc<dyn Hit> {
    fn hitify(self) -> Arc<dyn Hit> {
        self
    }
}
