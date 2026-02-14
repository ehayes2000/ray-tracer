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
            // bottom
            tri!(bb, ba, bd),
            tri!(bb, bd, bc),
            // top
            tri!(tb, ta, td),
            tri!(tb, td, tc),
            // side_ab
            tri!(ba, bb, tb),
            tri!(ta, tb, ba),
            // side_bc
            tri!(bb, bc, tc),
            tri!(tb, tc, bb),
            // side_cd
            tri!(bc, bd, td),
            tri!(tc, td, bc),
            // side_ad
            tri!(bd, ba, ta),
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
