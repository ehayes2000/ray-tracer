use super::Mesh;
use crate::{tri, v3};

use crate::material::Material;
use crate::math::{Point, Vec3};
use std::sync::Arc;

impl Mesh {
    ///  a --- b // xz plane
    ///  |     |
    ///  |     | ^
    ///  d --- c |
    ///          z --> x
    pub fn volume(ba: Point, tc_v: Vec3, material: Arc<dyn Material>) -> Self {
        let bb = ba + v3!(tc_v.0, 0, 0);
        let bc = ba + v3!(tc_v.0, 0, tc_v.2);
        let bd = ba + v3!(0, 0, tc_v.2);
        let tc = ba + tc_v;
        let ta = tc - v3!(tc_v.0, 0, tc_v.2);
        let tb = tc - v3!(0, 0, tc_v.2);
        let td = tc - v3!(tc_v.0, 0, 0);

        let tris = vec![
            // bottom (-y outward)
            tri!(ba, bb, bd),
            tri!(bb, bc, bd),
            // top (+y outward)
            tri!(tb, ta, td),
            tri!(tb, td, tc),
            // side_ab (-z outward)
            tri!(bb, ba, tb),
            tri!(ta, tb, ba),
            // side_bc (+x outward)
            tri!(bb, tc, bc),
            tri!(tb, tc, bb),
            // side_cd (+z outward)
            tri!(bd, bc, td),
            tri!(tc, td, bc),
            // side_ad (-x outward)
            tri!(bd, td, ba),
            tri!(td, ta, ba),
        ];

        let bbox = Self::bounding_box(&mut tris.iter());
        Self {
            bbox,
            tris,
            material,
        }
    }
}
