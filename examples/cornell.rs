use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::{Hitify, HittableList},
    material::{DiffuseLight, Lambertian, Materialify, Metal},
    mesh,
    mesh::Mesh,
    v3,
};
use std::sync::Arc;

fn main() {
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05)).materialify();
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73)).materialify();
    let blue = Lambertian::new(v3!(0.05, 0.05, 0.64)).materialify();
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15)).materialify();
    let light = DiffuseLight::new(v3!(15, 15, 15)).materialify();
    let mirror = Metal::new(v3!(0.9, 0.9, 0.9), 0.0).materialify();

    let mut world = HittableList::empty();

    // right wall
    world.add(Mesh::quad(v3!(555, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), green).hittable());
    // light
    world.add(Mesh::quad(v3!(343, 554, 332), v3!(-130, 0, 0), v3!(0, 0, -105), light).hittable());
    // back wall
    world.add(
        Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        )
        .hittable(),
    );
    // left wall
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), red.clone()).hittable());
    // roof
    world.add(
        Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        )
        .hittable(),
    );
    // floor
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(555, 0, 0), v3!(0, 0, 555), white.clone()).hittable());

    world.add(Mesh::volume(v3!(130, 0, 60), v3!(165, 165, 165), white.clone()).hittable());
    world.add(Mesh::volume(v3!(265, 0, 295), v3!(165, 330, 165), white.clone()).hittable());

    let dk = Mesh::try_from_file("models/dk-scaled.obj", mirror)
        .expect("dk")
        .scale(150.)
        .translate(v3!(150, 150, 150))
        .hittable();
    world.add(dk);

    let camera = Camera::new(
        CameraParameters {
            vfov: 40.,
            look_from: v3!(278, 278, -800),
            look_at: v3!(278, 278, 0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
        },
        RenderParameters {
            image_width: 600.,
            aspect_ratio: 1.0,
            max_bounces: 15.,
            samples_per_pixel: 14. * 4.,
            background_color: v3!(0, 0, 0),
        },
    );

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("cornell_bvh.ppm")
        .expect("cornell_bvh.ppm");

    let writer = BufWriter::new(file);
    camera.render_multi(14, writer, Arc::new(world.into_bvh()));
}
