use crate::{
    aabb::Aabb,
    hit::*,
    hittable_list::{Hitify, HittableList},
    math::{Interval, Ray},
};
use std::sync::Arc;

pub enum Bvh {
    Node(Arc<BvhNode>),
    Hittable(Arc<dyn Hit>),
}

struct EmptyPartition(Aabb);
impl EmptyPartition {
    pub fn new() -> Self {
        Self(Aabb::empty())
    }
}
impl Hit for EmptyPartition {
    fn bounding_box(&self) -> &Aabb {
        &self.0
    }
    fn hit(&self, _: &Ray, _: &Interval) -> Option<HitRecord> {
        None
    }
}

struct BvhNode {
    left: Bvh,
    right: Bvh,
    bbox: Aabb,
}

impl Bvh {
    pub fn from_objects(objects: Vec<Arc<dyn Hit>>) -> Self {
        Self::recursive_build(objects)
    }

    fn recursive_build(objects: Vec<Arc<dyn Hit>>) -> Bvh {
        if objects.is_empty() {
            return Bvh::Hittable(EmptyPartition::new().hitify());
        } else if objects.len() == 1 {
            return Bvh::Hittable(objects[0].clone());
        }
        let bbox = objects
            .iter()
            .fold(Aabb::empty(), |bbox, o| bbox.union(o.bounding_box()));

        let bbox_centroid = objects.iter().fold(Aabb::empty(), |bbox, o| {
            bbox.union_pt(&o.bounding_box().center())
        });
        let partition_axis = bbox_centroid.longest();
        let midpoint = bbox_centroid[partition_axis].center();
        // coplaner objects cannot be partitioned
        if bbox_centroid[partition_axis].size() == 0. {
            return Bvh::Hittable(HittableList::new(objects).hitify());
        }
        let (left, right) =
            objects
                .into_iter()
                .fold((vec![], vec![]), |(mut left, mut right), o| {
                    if o.bounding_box()[partition_axis].center() < midpoint {
                        left.push(o);
                    } else {
                        right.push(o)
                    }
                    (left, right)
                });

        Bvh::Node(Arc::new(BvhNode {
            bbox,
            left: Self::recursive_build(left),
            right: Self::recursive_build(right),
        }))
    }
}

impl Hit for BvhNode {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &crate::math::Ray, ray_t: &crate::math::Interval) -> Option<HitRecord> {
        self.bbox.hit(r, ray_t)?;
        let lhit = self.left.hit(r, ray_t);

        let ray_t = if let Some(h) = &lhit {
            &Interval::new(ray_t.min, h.t)
        } else {
            ray_t
        };
        let rhit = self.right.hit(r, ray_t);
        match (lhit, rhit) {
            (Some(l), Some(r)) => {
                if l.t < r.t {
                    Some(l)
                } else {
                    Some(r)
                }
            }
            (l @ Some(_), None) => l,
            (None, r @ Some(_)) => r,
            _ => None,
        }
    }
}

impl Hit for Bvh {
    fn bounding_box(&self) -> &Aabb {
        match self {
            Self::Node(n) => n.bounding_box(),
            Self::Hittable(h) => h.bounding_box(),
        }
    }

    fn hit(&self, r: &crate::math::Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Node(n) => n.hit(r, ray_t),
            Self::Hittable(h) => h.hit(r, ray_t),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        material::{Lambertian, Materialify},
        math::{Float, Interval, Ray},
        mesh::Mesh,
        ray, v3,
    };

    // bvh is falsly detecting hits on quads. The false positive is not consistent and some light still gets through
    // test with 2 planes
    // back: large
    // front small
    #[test]
    fn obscured_hit() {
        let material = Lambertian::new(v3!(1, 1, 1)).materialify();
        let objects: Vec<Arc<dyn Hit>> = vec![
            // back quad
            Mesh::quad(
                v3!(0, 0, 10),
                v3!(1000, 0, 0),
                v3!(0, 1000, 0),
                material.clone(),
            )
            .hitify(),
            Mesh::quad(
                v3!(10, 10, 0),
                v3!(10, 0, 0),
                v3!(0, 10, 0),
                material.clone(),
            )
            .hitify(),
        ];
        let look_from = v3!(15, 15, -10);
        let ray = Ray {
            direction: v3!(0, 0, 1),
            origin: look_from,
        };

        assert!(
            objects[1]
                .hit(&ray, &Interval::new(Float::MIN, Float::MAX))
                .is_some()
        );

        assert!(
            objects[0]
                .hit(&ray, &Interval::new(Float::MIN, Float::MAX))
                .is_some()
        );
        assert!(
            objects[0]
                .hit(&ray, &Interval::new(Float::MIN, 0.0))
                .is_none()
        );

        let bvh = Bvh::from_objects(objects.clone());
        let hit = bvh.hit(&ray, &Interval::new(Float::MIN, Float::MAX));
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().p, v3!(15, 15, 0));

        let r_miss = ray!(v3!(-1, -1, 0), v3!(0, 0, 1));
        assert!(objects[1].hit(&r_miss, &Interval::full()).is_none());
        assert!(objects[0].hit(&r_miss, &Interval::full()).is_none());
        assert!(bvh.hit(&r_miss, &Interval::full()).is_none());

        let r_hit_back = ray!(v3!(1, 1, 0), v3!(0, 0, 1));

        assert!(objects[1].hit(&r_hit_back, &Interval::full()).is_none());
        assert!(objects[0].hit(&r_hit_back, &Interval::full()).is_some());
        assert!(bvh.hit(&r_hit_back, &Interval::full()).is_some());
        assert_eq!(
            bvh.hit(&r_hit_back, &Interval::full()).unwrap().p,
            v3!(1, 1, 10)
        );
    }
}
