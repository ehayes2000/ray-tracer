use crate::{
    aabb::Aabb,
    bvh::BvhNode,
    hit::*,
    math::{Interval, Ray},
};
use std::{fmt::Debug, sync::Arc};

impl Hit for Arc<dyn Hit + Send + Sync + 'static> {
    fn bounding_box(&self) -> &Aabb {
        (**self).bounding_box()
    }
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        (**self).hit(r, ray_t)
    }
}
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hit>>,
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
    pub fn new(objects: Vec<Arc<dyn Hit>>) -> Self {
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

    pub fn push<T: Hitify>(mut self, object: T) -> Self {
        let object = object.hitify();
        self.bbox = self.bbox.union(object.bounding_box());
        self.objects.push(object);
        self
    }

    pub fn into_bvh(self) -> BvhNode {
        BvhNode::from_objects(self.objects)
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
