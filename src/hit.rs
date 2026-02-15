use crate::{
    aabb::Aabb,
    material::Material,
    math::{Float, Interval, Point, Ray, Vec3, dot},
};
use std::{fmt::Debug, sync::Arc};

pub trait Hit: Send + Sync + 'static {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
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
