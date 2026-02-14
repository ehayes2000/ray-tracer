use crate::Float;
use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::material::Material;
use crate::math::Interval;
use crate::math::Ray;
use crate::math::{Point, Vec3, dot};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Hit: Send + Sync + 'static {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}

impl Hit for Arc<dyn Hit + Send + Sync + 'static> {
    fn bounding_box(&self) -> &Aabb {
        (**self).bounding_box()
    }
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        (**self).hit(r, ray_t)
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub t: Float,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HitRecord")
            .field("p", &self.p)
            .field("t", &self.t)
            .field("normal", &self.normal)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl HitRecord {
    pub fn with_normal(
        p: Vec3,
        r: &Ray,
        u_out_norm: Vec3,
        t: Float,
        material: Arc<dyn Material>,
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
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::empty(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hit>) {
        self.bbox = self.bbox.clone().union(object.bounding_box());
        self.objects.push(object)
    }

    pub fn into_bvh(self) -> BvhNode {
        BvhNode::from_objects(self.objects)
    }
}

impl Hit for HittableList
// where
//     T: std::ops::Deref<Target = HittableList>,
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
