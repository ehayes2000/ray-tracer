use super::Mesh;
use crate::{aabb::Aabb, tri, v3};

use crate::{
    material::Material,
    math::{Point, Vec3},
};
use std::sync::Arc;

impl Mesh {
    ///  a --- b // xz plane
    ///  |     |
    ///  |     | ^
    ///  d --- c |
    ///          z --> x
    pub fn volume(ba: Point, tc_v: Vec3, material: impl Material) -> Self {
        let material = Arc::new(material);
        let bb = ba + v3!(tc_v.0, 0, 0);
        let bc = ba + v3!(tc_v.0, 0, tc_v.2);
        let bd = ba + v3!(0, 0, tc_v.2);
        let tc = ba + tc_v;
        let ta = tc - v3!(tc_v.0, 0, tc_v.2);
        let tb = tc - v3!(0, 0, tc_v.2);
        let td = tc - v3!(tc_v.0, 0, 0);

        let tris = vec![
            // bottom (-y outward)
            tri!(ba, bb, bd, material),
            tri!(bb, bc, bd, material),
            // top (+y outward)
            tri!(tb, ta, td, material),
            tri!(tb, td, tc, material),
            // side_ab (-z outward)
            tri!(bb, ba, tb, material),
            tri!(ta, tb, ba, material),
            // side_bc (+x outward)
            tri!(bb, tc, bc, material),
            tri!(tb, tc, bb, material),
            // side_cd (+z outward)
            tri!(bd, bc, td, material),
            tri!(tc, td, bc, material),
            // side_ad (-x outward)
            tri!(bd, td, ba, material),
            tri!(td, ta, ba, material),
        ];

        let bbox = tris
            .iter()
            .fold(Aabb::empty(), |acc, tri| acc.union(&tri.bbox));

        Self {
            bbox,
            tris,
            material,
        }
    }
}
