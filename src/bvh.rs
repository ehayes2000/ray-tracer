use crate::{aabb::Aabb, hittable::Hit};
use std::{cmp::Ordering, rc::Rc};

#[derive(Clone)]
enum NodeOrHittable {
    Node(Rc<BvhNode>),
    Hittable(Rc<dyn Hit>),
}

pub struct BvhNode {
    left: NodeOrHittable,
    right: NodeOrHittable,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_objects(objects: Vec<Rc<dyn Hit>>) -> Self {
        if objects.is_empty() {
            panic!("expected non-empty objects")
        }
        let end = objects.len();
        Self::from_range_of_objects(objects, 0, end)
    }

    fn from_range_of_objects(mut objects: Vec<Rc<dyn Hit>>, start: usize, end: usize) -> Self {
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
            let left = NodeOrHittable::Node(Rc::new(Self::from_range_of_objects(
                objects.clone(),
                start,
                mid,
            )));
            let right = NodeOrHittable::Node(Rc::new(Self::from_range_of_objects(
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

    fn box_compare(a: &Rc<dyn Hit>, b: &Rc<dyn Hit>, axis: usize) -> Ordering {
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
