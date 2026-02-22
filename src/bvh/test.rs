use super::*;
use crate::{
    aabb::Aabb,
    hit::Hit,
    material::{Lambertian, Materialify},
    math::{Float, Interval, Ray},
    mesh::Mesh,
    mesh_obj, ray, v3,
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

// bvh is falsly detecting hits on quads. The false positive is not consistent and some light still gets through
// test with 2 planes
// back: large
// front small
// #[test]
// fn obscured_hit() {
//     let material = Lambertian::new(v3!(1, 1, 1)).materialify();
//     let objects: Vec<Arc<dyn Hit>> = vec![
//         // back quad
//         Mesh::quad(
//             v3!(0, 0, 10),
//             v3!(1000, 0, 0),
//             v3!(0, 1000, 0),
//             material.clone(),
//         )
//         .hitify(),
//         Mesh::quad(
//             v3!(10, 10, 0),
//             v3!(10, 0, 0),
//             v3!(0, 10, 0),
//             material.clone(),
//         )
//         .hitify(),
//     ];
//     let look_from = v3!(15, 15, -10);
//     let ray = Ray {
//         direction: v3!(0, 0, 1),
//         origin: look_from,
//     };

//     assert!(
//         objects[1]
//             .hit(&ray, &Interval::new(Float::MIN, Float::MAX))
//             .is_some()
//     );

//     assert!(
//         objects[0]
//             .hit(&ray, &Interval::new(Float::MIN, Float::MAX))
//             .is_some()
//     );
//     assert!(
//         objects[0]
//             .hit(&ray, &Interval::new(Float::MIN, 0.0))
//             .is_none()
//     );

//     let bvh = Bvh::from_objects(objects.clone());
//     let hit = bvh.hit(&ray, &Interval::new(Float::MIN, Float::MAX));
//     assert!(hit.is_some());
//     assert_eq!(hit.unwrap().p, v3!(15, 15, 0));

//     let r_miss = ray!(v3!(-1, -1, 0), v3!(0, 0, 1));
//     assert!(objects[1].hit(&r_miss, &Interval::full()).is_none());
//     assert!(objects[0].hit(&r_miss, &Interval::full()).is_none());
//     assert!(bvh.hit(&r_miss, &Interval::full()).is_none());

//     let r_hit_back = ray!(v3!(1, 1, 0), v3!(0, 0, 1));

//     assert!(objects[1].hit(&r_hit_back, &Interval::full()).is_none());
//     assert!(objects[0].hit(&r_hit_back, &Interval::full()).is_some());
//     assert!(bvh.hit(&r_hit_back, &Interval::full()).is_some());
//     assert_eq!(
//         bvh.hit(&r_hit_back, &Interval::full()).unwrap().p,
//         v3!(1, 1, 10)
//     );
// }
