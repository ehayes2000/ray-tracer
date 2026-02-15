use crate::{
    aabb::Aabb,
    hittable::{Hit, Hitify, HittableList},
    math::Interval,
};
use std::sync::Arc;

#[derive(Clone)]
enum NodeOrHittable {
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
    fn hit(
        &self,
        _: &crate::math::Ray,
        _: &crate::math::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        None
    }
}

pub struct BvhNode {
    left: NodeOrHittable,
    right: NodeOrHittable,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_objects(objects: Vec<Arc<dyn Hit>>) -> Self {
        if let NodeOrHittable::Node(node) = Self::recursive_build(objects) {
            Arc::into_inner(node).expect("concrete type")
        } else {
            panic!("expected tree");
        }
    }

    fn recursive_build(objects: Vec<Arc<dyn Hit>>) -> NodeOrHittable {
        if objects.is_empty() {
            return NodeOrHittable::Hittable(EmptyPartition::new().hittable());
        } else if objects.len() == 1 {
            return NodeOrHittable::Hittable(objects[0].clone());
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
            return NodeOrHittable::Hittable(HittableList::new(objects, bbox).hittable());
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

        NodeOrHittable::Node(Arc::new(Self {
            bbox,
            left: Self::recursive_build(left),
            right: Self::recursive_build(right),
        }))
    }

    pub fn log_bboxes(&self) {
        match &self.left {
            NodeOrHittable::Hittable(h) => {
                println!("Obj {:?}", h.bounding_box())
            }
            NodeOrHittable::Node(n) => n.log_bboxes(),
        }
        match &self.right {
            NodeOrHittable::Hittable(h) => {
                println!("Obj {:?}", h.bounding_box())
            }
            NodeOrHittable::Node(n) => n.log_bboxes(),
        }
    }
}

impl Hit for BvhNode {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(
        &self,
        r: &crate::math::Ray,
        ray_t: &crate::math::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        if self.bbox.hit(r, ray_t).is_none() {
            return None;
        }
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

impl Hit for NodeOrHittable {
    fn bounding_box(&self) -> &Aabb {
        match self {
            Self::Node(n) => n.bounding_box(),
            Self::Hittable(h) => h.bounding_box(),
        }
    }

    fn hit(
        &self,
        r: &crate::math::Ray,
        ray_t: &crate::math::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        match self {
            Self::Node(n) => n.hit(r, ray_t),
            Self::Hittable(h) => h.hit(r, ray_t),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Float;
    use crate::material::Lambertian;
    use crate::material::Materialify;
    use crate::math::Interval;
    use crate::math::Ray;
    use crate::mesh::Mesh;
    use crate::{ray, v3};

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
            .hittable(),
            Mesh::quad(
                v3!(10, 10, 0),
                v3!(10, 0, 0),
                v3!(0, 10, 0),
                material.clone(),
            )
            .hittable(),
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

        let bvh = BvhNode::from_objects(objects.clone());
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
