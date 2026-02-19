use crate::{
    hit::HitRecord,
    material::Material,
    math::{Float, Interval, Point, Ray, Vec3, cross, dot},
};
use std::sync::Arc;
// counter-clockwise winding front
#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

pub(crate) struct TriHit {
    pub t: Float,
    pub front_face: bool,
    pub normal: Vec3,
}

impl Triangle {
    fn moller_trumbore_intersection(&self, r: &Ray) -> Option<TriHit> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        let t = Triangle {
            a: Vec3(0.0, 0.48371198773384094, 0.21013599634170532),
            b: Vec3(0.0, 0.454694002866745, 0.2285809963941574),
            c: Vec3(0.012532000429928303, 0.4558370113372803, 0.2269749939441681),
        };
        let r = Ray {
            origin: Vec3(0., 0.45843350887298584, 1.8337340354919434),
            direction: Vec3(0., 0., -1.0),
        };

        assert!(t.moller_trumbore_intersection(&r).is_some());
    }
}
