use crate::{
    aabb::Aabb,
    hit::{Hit, HitRecord},
    math::{Interval, Ray},
    mesh::Triangle,
    sphere::Sphere,
};

#[derive(Clone, Debug)]
pub enum HittableObject {
    Tri(Triangle),
    Sphere(Sphere),
}

#[derive(Clone, Debug)]
pub struct Pair {
    bbox: Aabb,
    pair: Vec<HittableObject>,
}

impl Pair {
    pub fn new(objects: Vec<HittableObject>) -> Self {
        if objects.len() != 2 {
            panic!("expected exactly 2 objects")
        }
        let bbox = Aabb::empty()
            .union(objects[0].bounding_box())
            .union(objects[1].bounding_box());
        Self {
            pair: objects,
            bbox,
        }
    }
}

impl Hit for Pair {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
    fn hit(&self, r: &Ray, t: &Interval) -> Option<HitRecord> {
        let mut any_hit = None::<HitRecord>;
        let mut closest_so_far = t.max;
        for object in &self.pair {
            if let Some(hit) = object.hit(r, &Interval::new(t.min, closest_so_far)) {
                closest_so_far = hit.t;
                any_hit = Some(hit);
            }
        }
        any_hit
    }
}

impl Hit for HittableObject {
    fn bounding_box(&self) -> &Aabb {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Tri(t) => t.bounding_box(),
        }
    }
    fn hit(&self, r: &Ray, t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(r, t),
            Self::Tri(tri) => tri.hit(r, t),
        }
    }
}
