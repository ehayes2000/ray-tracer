mod quad;
mod triangle;
mod volume;
pub use triangle::*;

use anyhow::{Result, anyhow};
use obj::{Obj, load_obj};
use std::{fs::File, io::BufReader, sync::Arc};

use crate::{
    aabb::Aabb,
    material::Material,
    math::{Float, Vec3},
};

#[macro_export]
macro_rules! tri {
    ($a:expr, $b:expr, $c:expr, $m:expr) => {{
        let normal = $crate::mesh::Triangle::compute_normal(&$a, &$b, &$c);
        let a = crate::mesh::Vertex {
            position: $a,
            normal,
        };
        let b = crate::mesh::Vertex {
            position: $b,
            normal,
        };

        let c = crate::mesh::Vertex {
            position: $c,
            normal,
        };
        $crate::mesh::Triangle::new(a, b, c, $m.clone())
    }};
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub material: Arc<dyn Material>,
    pub bbox: Aabb,
}

impl Mesh {
    pub fn try_from_file(path: &str, material: impl Material) -> Result<Self> {
        let buf = BufReader::new(File::open(path)?);
        let obj: Obj = load_obj(buf)?;
        let material = Arc::new(material);

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
            tris.push(Triangle::new(
                Vertex {
                    normal: obj.vertices[i[0]].normal.into(),
                    position: obj.vertices[i[0]].position.into(),
                },
                Vertex {
                    normal: obj.vertices[i[1]].normal.into(),
                    position: obj.vertices[i[1]].position.into(),
                },
                Vertex {
                    normal: obj.vertices[i[2]].normal.into(),
                    position: obj.vertices[i[2]].position.into(),
                },
                material.clone(),
            ));
            tris
        });

        let bbox = tris.iter().fold(Aabb::empty(), |bbox, tri| {
            bbox.union_pt(&tri.a.position)
                .union_pt(&tri.b.position)
                .union_pt(&tri.c.position)
        });

        Ok(Self {
            tris,
            material,
            bbox,
        })
    }

    pub fn rotate(mut self, rotation: Vec3) -> Self {
        let center = self.bbox.center();
        self.tris.iter_mut().for_each(|t| {
            t.a.position = Self::rotate_about_center(t.a.position, center, rotation);
            t.b.position = Self::rotate_about_center(t.b.position, center, rotation);
            t.c.position = Self::rotate_about_center(t.c.position, center, rotation);

            t.a.normal = Self::rotate_point(t.a.normal, rotation);
            t.b.normal = Self::rotate_point(t.b.normal, rotation);
            t.c.normal = Self::rotate_point(t.c.normal, rotation);
            t.update_bbox();
        });

        let bbox = self
            .tris
            .iter()
            .fold(Aabb::empty(), |acc, tri| acc.union(&tri.bbox));

        Self {
            bbox,
            material: self.material,
            tris: self.tris,
        }
    }

    fn rotate_point(p: Vec3, rotation: Vec3) -> Vec3 {
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

        Vec3(x3, y3, z3)
    }

    fn rotate_about_center(point: Vec3, center: Vec3, rotation: Vec3) -> Vec3 {
        let p = point - center;
        Self::rotate_point(p, rotation) + center
    }

    pub fn translate(self, v: Vec3) -> Self {
        let tris = self
            .tris
            .into_iter()
            .map(|mut tri| {
                tri.a.position += v;
                tri.b.position += v;
                tri.c.position += v;
                tri.update_bbox();
                tri
            })
            .collect::<Vec<_>>();

        let bbox = tris
            .iter()
            .fold(Aabb::empty(), |acc, tri| acc.union(&tri.bbox));

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
                tri.a.position = (tri.a.position - c) * f + c;
                tri.b.position = (tri.b.position - c) * f + c;
                tri.c.position = (tri.c.position - c) * f + c;
                tri.update_bbox();
                tri
            })
            .collect::<Vec<_>>();
        let bbox = tris
            .iter()
            .fold(Aabb::empty(), |acc, tri| acc.union(&tri.bbox));

        Self {
            bbox,
            tris,
            material: self.material,
        }
    }
}

#[macro_export]
macro_rules! mesh_obj {
    ($path:expr, $material:expr) => {{
        let full_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join($path);
        $crate::mesh::Mesh::try_from_file(full_path.to_str().unwrap(), $material)
    }};
}
