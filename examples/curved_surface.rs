use ray_tracer::{
    aabb::Aabb,
    bvh::BvhBuilder,
    camera::{Camera, CameraParameters, RenderParameters},
    material::{DiffuseLight, Lambertian},
    mesh::{Mesh, Triangle, Vertex},
    sphere::Sphere,
    v3,
};
use std::sync::Arc;

fn main() {
    let material = Arc::new(Lambertian::new(v3!(1., 1., 1.)));

    let tris = vec![
        // b -- a
        //   \  |
        //      c
        Triangle::new(
            Vertex {
                position: v3!(0.5, 0.5, 0.5),
                normal: v3!(0.5, 0.5, 0.5).normalize(),
            },
            Vertex {
                position: v3!(-0.5, 0.5, 0.5),
                normal: v3!(-0.5, 0.5, 0.5).normalize(),
            },
            Vertex {
                position: v3!(0.5, -0.5, 0.5),
                normal: v3!(0.5, -0.5, 0.5).normalize(),
            },
            material.clone(),
        ),
        // a
        // |  \
        // b - c
        Triangle::new(
            Vertex {
                position: v3!(-0.5, 0.5, 0.5),
                normal: v3!(-0.5, 0.5, 0.5).normalize(),
            },
            Vertex {
                position: v3!(-0.5, -0.5, 0.5),
                normal: v3!(-0.5, -0.5, 0.5).normalize(),
            },
            Vertex {
                position: v3!(0.5, -0.5, 0.5),
                normal: v3!(0.5, -0.5, 0.5).normalize(),
            },
            material.clone(),
        ),
    ];
    let bbox = tris.iter().fold(Aabb::empty(), |acc, v| {
        acc.union_pt(&v.a.position)
            .union_pt(&v.b.position)
            .union_pt(&v.c.position)
    });
    let curved = Mesh {
        bbox,
        material,
        tris,
    };
    let quad = Mesh::quad(
        v3!(-0.1, -0.75, 0.5),
        v3!(0.2, 0, 0),
        v3!(0., 0.2, 0),
        Lambertian::new(v3!(1, 1, 1)),
    );

    let red = Sphere::new(
        v3!(-0.8, 0., 0.5),
        0.2,
        DiffuseLight::new(v3!(0.8, 0.15, 0.15)),
    );
    let green = Sphere::new(
        v3!(0.8, 0., 0.5),
        0.2,
        DiffuseLight::new(v3!(0.15, 0.8, 0.15)),
    );
    let blue = Sphere::new(
        v3!(0.0, 0.8, 0.5),
        0.2,
        DiffuseLight::new(v3!(0.15, 0.15, 0.8)),
    );

    let world = BvhBuilder::new()
        .mesh(curved)
        .mesh(quad)
        .sphere(red)
        .sphere(green)
        .sphere(blue)
        .build();

    let camera_params = CameraParameters {
        look_at: v3!(0, 0, 0),
        look_from: v3!(0, 0, 1.5),
        ..Default::default()
    };
    let render_params = RenderParameters {
        aspect_ratio: 1.,
        background_color: v3!(0., 0., 0.),
        image_width: 720.,
        samples_per_pixel: 50.,
        max_bounces: 5.,
    };
    let camera = Camera::new(camera_params, render_params);
    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("curved.ppm")
        .expect("file");

    camera.render_multi(14, f, Arc::new(world));
}
