use super::{Mesh, Triangle};

use crate::aabb::Aabb;
use crate::material::Material;
use crate::math::{Point, Vec3};
use std::sync::Arc;

impl Mesh {
    pub fn quad(corner: Point, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::material::Lambertian;
    use crate::v3;
    #[test]
    fn bounding_box() {
        let mesh = Mesh::quad(
            v3!(0, 0, 0),
            v3!(1, 0, 0),
            v3!(0, 1, 0),
            Lambertian::new(v3!(1, 1, 1)).into(),
        );
        println!("bbox {:?}", mesh.bbox);
        assert_eq!(mesh.bbox.center(), v3!(0.5, 0.5, 0.0));
    }
}
