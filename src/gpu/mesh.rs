use crate::math::Vec3;
use obj::load_obj;
use std::{fs::File, io::BufReader};

use super::types::Triangle;
use anyhow::Result;
use obj::Obj;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub obj: Obj,
}

impl Mesh {
    pub fn from_file(path: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let obj: Obj = load_obj(reader)?;
        Ok(Self { obj })
    }

    pub fn into_triangles(self, material: u32) -> Vec<Triangle> {
        self.obj
            .indices
            .into_iter()
            .fold((Vec::new(), [0, 0, 0], 0), |(mut c, mut p, i), e| {
                p[i] = e;
                if i == 2 {
                    c.push(p);
                    (c, [0, 0, 0], 0)
                } else {
                    (c, p, i + 1)
                }
            })
            .0
            .into_iter()
            .map(|[a, b, c]| Triangle {
                a: Vec3::from(self.obj.vertices[a as usize].position),
                b: Vec3::from(self.obj.vertices[b as usize].position),
                c: Vec3::from(self.obj.vertices[c as usize].position),
                material,
            })
            .collect()
    }

    pub fn translate(mut self, direction: Vec3) -> Self {
        self.obj.vertices.iter_mut().for_each(|v| {
            v.position[0] += direction.0;
            v.position[1] += direction.1;
            v.position[2] += direction.2;
        });
        self
    }
}
