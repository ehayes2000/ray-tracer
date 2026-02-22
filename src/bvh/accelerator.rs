use super::types::{HittableObject, Pair};
use crate::{
    aabb::Aabb,
    hit::*,
    math::{Interval, Ray},
};
pub enum Bvh {
    Node(Box<BvhNode>),
    Leaf(Box<dyn Hit>),
}

impl std::fmt::Debug for Bvh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(n) => write!(f, "Node({:?})", n),
            Self::Leaf(_) => write!(f, "Leaf"),
        }
    }
}

#[derive(Debug)]
pub struct BvhNode {
    pub left: Option<Bvh>,
    pub right: Option<Bvh>,
    pub bbox: Aabb,
}

impl Bvh {
    pub(crate) fn from_objects(objects: Vec<HittableObject>) -> Self {
        Self::recursive_build(objects).expect("at least one object")
    }

    fn recursive_build(objects: Vec<HittableObject>) -> Option<Bvh> {
        if objects.is_empty() {
            return None;
        } else if objects.len() == 1 {
            return Some(Bvh::Leaf(Box::new(objects[0].clone())));
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
            if objects.len() > 2 {
                eprintln!("n objects {} -> {:?}", objects.len(), bbox_centroid);
                eprintln!("{:#?}", objects);
                panic!("more than 2 coplaner objects");
            }
            return Some(Bvh::Leaf(Box::new(Pair::new(objects))));
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

        Some(Bvh::Node(Box::new(BvhNode {
            bbox,
            left: Self::recursive_build(left),
            right: Self::recursive_build(right),
        })))
    }
}

impl Hit for BvhNode {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.bbox.hit(r, ray_t)?;
        let lhit = self.left.as_ref().and_then(|l| l.hit(r, ray_t));

        let ray_t = if let Some(h) = &lhit {
            &Interval::new(ray_t.min, h.t)
        } else {
            ray_t
        };
        let rhit = self.right.as_ref().and_then(|n| n.hit(r, ray_t));
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
            Self::Leaf(h) => h.bounding_box(),
        }
    }

    fn hit(&self, r: &crate::math::Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Node(n) => n.hit(r, ray_t),
            Self::Leaf(h) => h.hit(r, ray_t),
        }
    }
}
