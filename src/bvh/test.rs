use super::*;
use crate::{
    aabb::Aabb,
    hit::Hit,
    material::Lambertian,
    math::{Interval, Ray},
    mesh::Mesh,
    mesh_obj, v3,
};

#[test]
fn quad_hit() {
    let material = Lambertian::new(v3!(1, 1, 1));
    let quad = Mesh::quad(v3!(0, 0, 0), v3!(1, 0, 0), v3!(0, 1, 0), material);
    let bvh = BvhBuilder::new().mesh(quad);
    let dbg = bvh.clone().debug();
    let bvh = bvh.build();
    let r = Ray {
        direction: v3!(0, 0, 1),
        origin: v3!(0.25, 0.25, -1),
    };
    let i = Interval::full();
    let hb = bvh.hit(&r, &i).unwrap();
    let hd = dbg.hit(&r, &i).unwrap();
    assert_eq!(hb.p, hd.p);
    let r = Ray {
        direction: v3!(0, 0, 1),
        origin: v3!(0.75, 0.75, -1),
    };

    let hb = bvh.hit(&r, &i).unwrap();
    let hd = dbg.hit(&r, &i).unwrap();
    assert_eq!(hb.p, hd.p);
}

#[test]
fn mesh_hit() {
    let material = Lambertian::new(v3!(0, 0, 0));
    let mesh = mesh_obj!("models/chess_knight.obj", material).expect("load mesh");
    let builder = BvhBuilder::new().mesh(mesh);
    let dbg = builder.clone().debug();
    let bvh = builder.build();
    let r = Ray {
        origin: v3!(0.0, 0.45843350887298584, 1.8337340354919434),
        direction: v3!(0.0, 0.0, -1.0),
    };
    let i = Interval::full();
    let hb = bvh.hit(&r, &i);
    let hd = dbg.hit(&r, &i);
    hd.unwrap();
    hb.unwrap();
}

#[test]
fn nested_boxes() {
    let material = Lambertian::new(v3!(0, 0, 0));
    let mesh = mesh_obj!("models/chess_knight.obj", material).expect("load mesh");
    let bvh = BvhBuilder::new().mesh(mesh).build();
    fn contains_children(b: &Bvh, parent: Option<Aabb>) -> bool {
        match (b, parent) {
            (Bvh::Leaf(l), Some(parent)) => parent.contains(l.bounding_box()),
            (Bvh::Leaf(_), None) => true,
            (Bvh::Node(n), None) => {
                n.left
                    .as_ref()
                    .map(|l| contains_children(l, Some(n.bounding_box().clone())))
                    .unwrap_or(true)
                    && n.right
                        .as_ref()
                        .map(|r| contains_children(r, Some(n.bounding_box().clone())))
                        .unwrap_or(true)
            }
            (Bvh::Node(n), Some(parent)) => {
                if !parent.contains(n.bounding_box()) {
                    return false;
                }
                n.left
                    .as_ref()
                    .map(|l| contains_children(l, Some(n.bounding_box().clone())))
                    .unwrap_or(true)
                    && n.right
                        .as_ref()
                        .map(|r| contains_children(r, Some(n.bounding_box().clone())))
                        .unwrap_or(true)
            }
        }
    }
    assert!(contains_children(&bvh, None));
}
