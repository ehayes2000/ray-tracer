mod quad;
mod volume;

use anyhow::{Result, anyhow};
use obj::{Obj, load_obj};
use std::{fs::File, io::BufReader, sync::Arc};

use crate::{
    aabb::Aabb,
    hit::*,
    material::{Material, Materialify},
    math::{Float, Interval, Point, Ray, Vec3, cross, dot},
};

// counter-clockwise winding front
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[macro_export]
macro_rules! tri {
    ($a:expr, $b:expr, $c:expr) => {
        $crate::mesh::Triangle {
            a: $a,
            b: $b,
            c: $c,
        }
    };
}

pub struct Vertex {
    pub position: Vec3,
}

pub struct Mesh {
    tris: Vec<Triangle>,
    material: Arc<dyn Material>,
    bbox: Aabb,
}

impl Mesh {
    pub fn try_from_file(path: &str, material: impl Materialify) -> Result<Self> {
        let material = material.materialify();
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
                    obj.vertices[i[0]].position[0] as _,
                    obj.vertices[i[0]].position[1] as _,
                    obj.vertices[i[0]].position[2] as _,
                ),

                b: Vec3(
                    obj.vertices[i[1]].position[0] as _,
                    obj.vertices[i[1]].position[1] as _,
                    obj.vertices[i[1]].position[2] as _,
                ),

                c: Vec3(
                    obj.vertices[i[2]].position[0] as _,
                    obj.vertices[i[2]].position[1] as _,
                    obj.vertices[i[2]].position[2] as _,
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

    pub fn into_hittable(self) -> Box<dyn Hit> {
        Box::new(self)
    }

    fn bounding_box(tris: &mut dyn Iterator<Item = &Triangle>) -> Aabb {
        tris.fold(Aabb::empty(), |bounds, tri| {
            bounds.union_pt(&tri.a).union_pt(&tri.b).union_pt(&tri.c)
        })
    }

    pub fn rotate(mut self, rotation: Vec3) -> Self {
        let center = self.bbox.center();
        self.tris.iter_mut().for_each(|t| {
            t.a = Self::rotate_point(t.a, center, rotation);
            t.b = Self::rotate_point(t.b, center, rotation);
            t.c = Self::rotate_point(t.c, center, rotation);
        });
        let bbox = Self::bounding_box(&mut self.tris.iter());
        Self {
            bbox,
            material: self.material,
            tris: self.tris,
        }
    }

    fn rotate_point(point: Vec3, center: Vec3, rotation: Vec3) -> Vec3 {
        let p = point - center;

        let (sx, cx) = rotation.0.sin_cos();
        let (sy, cy) = rotation.1.sin_cos();
        let (sz, cz) = rotation.2.sin_cos();

        let y1 = p.1 * cx - p.2 * sx;
        let z1 = p.1 * sx + p.2 * cx;
        let x1 = p.0;

        let x2 = x1 * cy + z1 * sy;
        let z2 = -x1 * sy + z1 * cy;
        let y2 = y1;

        let x3 = x2 * cz - y2 * sz;
        let y3 = x2 * sz + y2 * cz;
        let z3 = z2;

        Vec3(x3, y3, z3) + center
    }

    pub fn translate(self, v: Vec3) -> Self {
        let tris = self
            .tris
            .into_iter()
            .map(|mut tri| {
                tri.a += v;
                tri.b += v;
                tri.c += v;
                tri
            })
            .collect::<Vec<_>>();
        let bbox = tris.iter().fold(Aabb::empty(), |acc, tri| {
            acc.union_pt(&tri.a).union_pt(&tri.b).union_pt(&tri.c)
        });

        Self {
            tris,
            bbox,
            material: self.material,
        }
    }

    pub fn scale(self, f: Float) -> Self {
        let c = self.bbox.center();
        let tris = self
            .tris
            .into_iter()
            .map(|mut tri| {
                tri.a = (tri.a - c) * f + c;
                tri.b = (tri.b - c) * f + c;
                tri.c = (tri.c - c) * f + c;
                tri
            })
            .collect::<Vec<_>>();
        let bbox = tris.iter().fold(Aabb::empty(), |acc, tri| {
            acc.union_pt(&tri.a).union_pt(&tri.b).union_pt(&tri.c)
        });

        Self {
            bbox,
            tris,
            material: self.material,
        }
    }
}

impl Mesh {
    fn moller_trumbore_intersection(&self, i: usize, r: &Ray) -> Option<HitRecord> {
        let tri = &self.tris[i];
        let e1 = tri.b - tri.a;
        let e2 = tri.c - tri.a;

        let ray_cross_e2 = cross(r.direction, e2);
        let det = dot(&e1, &ray_cross_e2);

        if det > -crate::math::EPSILON && det < crate::math::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = r.origin - tri.a;
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
            if let Some(hit) = self.moller_trumbore_intersection(i, r)
                && ray_t.surrounds(hit.t)
                && hit.t < best_hit.as_ref().map(|h| h.t).unwrap_or(Float::MAX)
            {
                best_hit = Some(hit);
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
        $crate::mesh::Mesh::try_from_file(full_path.to_str().unwrap(), $material)
    }};
}
