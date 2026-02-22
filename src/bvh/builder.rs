use super::{Bvh, types::HittableObject};
use crate::{
    aabb::Aabb,
    hit::{Hit, HitRecord},
    math::{Interval, Ray},
    mesh::Mesh,
    sphere::Sphere,
};

#[derive(Clone, Default)]
pub struct BvhBuilder {
    pub objects: Vec<HittableObject>,
}

#[derive(Debug, Default)]
pub struct BvhDebugger {
    pub objects: Vec<HittableObject>,
    pub bbox: Aabb,
}

impl BvhBuilder {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn mesh(mut self, mesh: Mesh) -> Self {
        let tris = mesh.tris.into_iter().map(HittableObject::Tri);
        self.objects.extend(tris);
        self
    }

    pub fn sphere(mut self, sphere: Sphere) -> Self {
        self.objects.push(HittableObject::Sphere(sphere));
        self
    }

    pub fn build(self) -> Bvh {
        Bvh::from_objects(self.objects)
    }

    pub fn debug(self) -> BvhDebugger {
        eprintln!("using O(n) intersection search");
        let bbox = self
            .objects
            .iter()
            .fold(Aabb::empty(), |acc, o| acc.union(o.bounding_box()));

        BvhDebugger {
            objects: self.objects,
            bbox,
        }
    }
}

impl Hit for BvhDebugger {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut any_hit = None::<HitRecord>;
        let mut closest_so_far = ray_t.max;
        let mut bbox = None;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                any_hit = Some(hit);
                bbox = Some(object.bounding_box());
            }
        }
        println!("hit {:?}", bbox);
        any_hit
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
