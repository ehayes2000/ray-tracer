use anyhow::{Result, anyhow};
use obj::{Obj, load_obj};
use std::path::Path;
use std::rc::Rc;
use std::{f32, fs::File, io::BufReader};

use crate::hittable::{Hit, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use crate::math::{Ray, Vec3, cross, dot};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

pub struct Mesh {
    indices: Vec<[usize; 3]>,
    vertices: Vec<Vertex>,
    material: Rc<dyn Material>,
}

impl Mesh {
    pub fn try_from_file(path: &str, material: Rc<dyn Material>) -> Result<Self> {
        let buf = BufReader::new(File::open(path)?);
        let obj: Obj = load_obj(buf)?;

        let n_indices = obj.indices.len();
        if n_indices % 3 != 0 {
            return Err(anyhow!("expected triangulated object"));
        }
        let (indices, ..) = obj.indices.into_iter().fold(
            (Vec::new(), [0, 0, 0], 0),
            |(mut collection, mut index, i), e| {
                index[i] = e as usize;
                if i == 2 {
                    collection.push(index);
                    (collection, [0, 0, 0], 0)
                } else {
                    (collection, index, i + 1)
                }
            },
        );

        assert_eq!(indices.len(), n_indices / 3, "unexpected n indices");

        let vertices = obj
            .vertices
            .into_iter()
            .map(|v| Vertex {
                normal: Vec3(v.normal[0], v.normal[1], v.normal[2]),
                position: Vec3(v.position[0], v.position[1], v.position[2]),
            })
            .collect();

        Ok(Self {
            vertices,
            indices,
            material,
        })
    }

    pub fn into_hittable(self) -> Box<dyn Hit> {
        Box::new(self)
    }
}

impl Mesh {
    fn moller_trumbore_intersection(&self, i: &[usize; 3], r: &Ray) -> Option<HitRecord> {
        let e1 = self.vertices[i[1]].position - self.vertices[i[0]].position;
        let e2 = self.vertices[i[2]].position - self.vertices[i[0]].position;

        let ray_cross_e2 = cross(r.direction, e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = r.origin - self.vertices[i[0]].position;
        let u = inv_det * dot(&s, &ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = cross(s, e1);
        let v = inv_det * dot(&r.direction, &s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let normal = cross(e1, e2).normalize();
        let front_face = dot(&r.direction, &normal) < 0.0;
        let t = inv_det * dot(&e2, &s_cross_e1);
        if t > f32::EPSILON {
            let intersection_point = r.origin + r.direction * t;
            Some(HitRecord {
                front_face,
                material: self.material.clone(),
                normal: cross(e1, e2).normalize(),
                p: intersection_point,
                t: t,
            })
        } else {
            None
        }
    }
}

impl Hit for Mesh {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut best_hit = None::<HitRecord>;
        for i in self.indices.iter() {
            if let Some(hit) = self.moller_trumbore_intersection(i, r) {
                if ray_t.surrounds(hit.t)
                    && hit.t < best_hit.as_ref().map(|h| h.t).unwrap_or(f32::MAX)
                {
                    best_hit = Some(hit);
                }
            }
        }
        best_hit
    }
}

#[macro_export]
macro_rules! mesh {
    ($path:expr, $material:expr) => {{
        let base = std::path::Path::new(file!()).parent().unwrap();
        let full_path = base.join($path);
        crate::mesh::Mesh::try_from_file(full_path.to_str().unwrap(), $material)
            .map(|mesh| mesh.into_hittable())
    }};
}
