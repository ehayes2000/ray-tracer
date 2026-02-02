use anyhow::{Result, anyhow};
use obj::{Obj, load_obj};
use std::rc::Rc;
use std::{fs::File, io::BufReader};

use crate::aabb::Aabb;
use crate::hittable::{Hit, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use crate::math::{Point, Ray, Vec3, cross, dot};

// triangles are clockwise forward
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

pub struct Vertex {
    pub position: Vec3,
}

pub struct Mesh {
    tris: Vec<Triangle>,
    material: Rc<dyn Material>,
    bbox: Aabb,
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

        let tris = indices.iter().fold(Vec::new(), |mut tris, i| {
            tris.push(Triangle {
                a: Vec3(
                    obj.vertices[i[0]].position[0],
                    obj.vertices[i[0]].position[1],
                    obj.vertices[i[0]].position[2],
                ),

                b: Vec3(
                    obj.vertices[i[1]].position[0],
                    obj.vertices[i[1]].position[1],
                    obj.vertices[i[1]].position[2],
                ),

                c: Vec3(
                    obj.vertices[i[2]].position[0],
                    obj.vertices[i[2]].position[1],
                    obj.vertices[i[2]].position[2],
                ),
            });
            tris
        });

        let bbox = tris.iter().fold(Aabb::empty(), |bbox, tri| {
            bbox.union_pt(&tri.a).union_pt(&tri.b).union_pt(&tri.c)
        });

        Ok(Self {
            tris,
            material,
            bbox,
        })
    }

    pub fn quad(corner: Point, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let a = corner;
        let b = a + u;
        let c = a + v;
        let d = b + v;
        let bbox = Aabb::empty()
            .union_pt(&a)
            .union_pt(&b)
            .union_pt(&c)
            .union_pt(&d)
            .pad();
        let tris = vec![Triangle { a, b, c }, Triangle { a: b, b: d, c: c }];
        Self {
            tris,
            material,
            bbox,
        }
    }

    pub fn into_hittable(self) -> Box<dyn Hit> {
        Box::new(self)
    }

    pub fn obj(self) -> Rc<dyn Hit> {
        Rc::new(self)
    }
}

impl Mesh {
    fn moller_trumbore_intersection(&self, i: usize, r: &Ray) -> Option<HitRecord> {
        let tri = &self.tris[i];
        let e1 = tri.b - tri.a;
        let e2 = tri.c - tri.a;

        let ray_cross_e2 = cross(r.direction, e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = r.origin - tri.a;
        let u = inv_det * dot(&s, &ray_cross_e2);
        if u < 0.0 || u > 1.0 {
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
        if t > f32::EPSILON {
            let intersection_point = r.origin + r.direction * t;
            Some(HitRecord {
                front_face,
                material: self.material.clone(),
                normal,
                p: intersection_point,
                t,
            })
        } else {
            None
        }
    }
}

impl Hit for Mesh {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut best_hit = None::<HitRecord>;
        for i in 0..self.tris.len() {
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
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
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
