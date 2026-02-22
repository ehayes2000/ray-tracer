use crate::{
    aabb::Aabb,
    hit::{Hit, HitRecord},
    material::Material,
    math::{Float, Interval, Point, Ray, Vec3, cross, dot},
};
use std::sync::Arc;
// counter-clockwise winding front
#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub bbox: Aabb,
    pub material: Arc<dyn Material>,
}

pub struct TriHit {
    pub t: Float,
    pub front_face: bool,
    pub normal: Vec3,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point, material: Arc<dyn Material>) -> Self {
        let bbox = Aabb::empty().union_pt(&a).union_pt(&b).union_pt(&c).pad();
        Self {
            a,
            b,
            c,
            bbox,
            material,
        }
    }
    pub fn moller_trumbore_intersection(&self, r: &Ray) -> Option<TriHit> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = cross(r.direction, e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -crate::math::EPSILON && det < crate::math::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = r.origin - self.a;
        let u = inv_det * dot(&s, &ray_cross_e2);
        if !(0.0..1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = cross(s, e1);
        let v = inv_det * dot(&r.direction, &s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let outward_normal = cross(e1, e2).normalize();
        let front_face = dot(&r.direction, &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let t = inv_det * dot(&e2, &s_cross_e1);
        if t > crate::math::EPSILON {
            Some(TriHit {
                front_face,
                t,
                normal,
            })
        } else {
            None
        }
    }

    pub fn tri_hit(
        &self,
        r: &Ray,
        t: &Interval,
        material: &Arc<dyn Material>,
    ) -> Option<HitRecord> {
        self.moller_trumbore_intersection(r).and_then(|tri_hit| {
            if t.contains(tri_hit.t) {
                let intersection_point = r.origin + tri_hit.t * r.direction;
                Some(HitRecord {
                    p: intersection_point,
                    t: tri_hit.t,
                    normal: tri_hit.normal,
                    front_face: tri_hit.front_face,
                    material: material.clone(),
                })
            } else {
                None
            }
        })
    }

    pub fn update_bbox(&mut self) {
        self.bbox = Aabb::empty()
            .union_pt(&self.a)
            .union_pt(&self.b)
            .union_pt(&self.c)
            .pad();
    }
}

impl Hit for Triangle {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.moller_trumbore_intersection(r).and_then(|tri_hit| {
            if ray_t.contains(tri_hit.t) {
                let intersection_point = r.origin + tri_hit.t * r.direction;
                Some(HitRecord {
                    p: intersection_point,
                    t: tri_hit.t,
                    normal: tri_hit.normal,
                    front_face: tri_hit.front_face,
                    material: self.material.clone(),
                })
            } else {
                None
            }
        })
    }
}
