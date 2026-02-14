use crate::{aabb::Aabb, hittable::Hit};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
enum NodeOrHittable {
    Node(Arc<BvhNode>),
    Hittable(Arc<dyn Hit>),
}

pub struct BvhNode {
    left: NodeOrHittable,
    right: NodeOrHittable,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_objects(objects: Vec<Arc<dyn Hit>>) -> Self {
        if objects.is_empty() {
            panic!("expected non-empty objects")
        }
        let end = objects.len();
        Self::from_range_of_objects(objects, 0, end)
    }

    fn from_range_of_objects(mut objects: Vec<Arc<dyn Hit>>, start: usize, end: usize) -> Self {
        let big_box = objects[start..end]
            .iter()
            .fold(Aabb::empty(), |acc, o| acc.union(o.bounding_box()));

        let axis = if big_box.x.size() > big_box.y.size() {
            if big_box.x.size() > big_box.z.size() {
                0
            } else {
                2
            }
        } else {
            if big_box.y.size() > big_box.z.size() {
                1
            } else {
                2
            }
        };

        let span = end - start;
        if span == 1 {
            let node = NodeOrHittable::Hittable(objects[start].clone());
            let left = node.clone();
            let right = node;
            let bbox = left.bounding_box().to_owned();
            Self { bbox, right, left }
        } else if span == 2 {
            let left = NodeOrHittable::Hittable(objects[start].clone());
            let right = NodeOrHittable::Hittable(objects[start + 1].clone());
            let bbox = Aabb::empty()
                .union(left.bounding_box())
                .union(right.bounding_box());
            Self { bbox, left, right }
        } else {
            (&mut objects[start..end]).sort_by(|a, b| Self::box_compare(a, b, axis));
            let mid = start + span / 2;
            let left = NodeOrHittable::Node(Arc::new(Self::from_range_of_objects(
                objects.clone(),
                start,
                mid,
            )));
            let right = NodeOrHittable::Node(Arc::new(Self::from_range_of_objects(
                objects.clone(),
                mid,
                end,
            )));
            let bbox = Aabb::empty()
                .union(left.bounding_box())
                .union(right.bounding_box());
            Self { bbox, left, right }
        }
    }

    fn box_compare(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>, axis: usize) -> Ordering {
        let a_ax_interval = a.bounding_box().axis(axis);
        let b_ax_interval = b.bounding_box().axis(axis);
        a_ax_interval.min.total_cmp(&b_ax_interval.min)
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
        ray_t: &crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        if let Some(hit_t) = self.bbox.hit(r, ray_t) {
            let lhit = self.left.hit(r, &hit_t);
            let rhit = self.right.hit(r, &hit_t);
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
                (None, None) => {
                    // unreachable!("left or right must return hit");
                    None
                }
            }
        } else {
            None
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
        ray_t: &crate::interval::Interval,
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
    use crate::interval::Interval;
    use crate::material::Lambertian;
    use crate::math::Ray;
    use crate::mesh::Mesh;
    use crate::{ray, v3};

    // bvh is falsly detecting hits on quads. The false positive is not consistent and some light still gets through
    // test with 2 planes
    // back: large
    // front small
    #[test]
    fn obscured_hit() {
        let material = Lambertian::obj(v3!(1, 1, 1));
        let objects = vec![
            // back quad
            Mesh::quad(
                v3!(0, 0, 10),
                v3!(1000, 0, 0),
                v3!(0, 1000, 0),
                material.clone(),
            )
            .obj(),
            Mesh::quad(
                v3!(10, 10, 0),
                v3!(10, 0, 0),
                v3!(0, 10, 0),
                material.clone(),
            )
            .obj(),
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
