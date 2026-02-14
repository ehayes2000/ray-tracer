use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::HittableList,
    material::{DiffuseLight, Lambertian, Metal},
    mesh::Mesh,
    v3,
};
use std::sync::Arc;

fn main() {
    let red = Lambertian::obj(v3!(0.65, 0.05, 0.05));
    let white = Lambertian::obj(v3!(0.73, 0.73, 0.73));
    let blue = Lambertian::obj(v3!(0.05, 0.05, 0.64));
    let green = Lambertian::obj(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::obj(v3!(15, 15, 15));
    let mirror = Metal::obj(v3!(0.9, 0.9, 0.9), 0.0);

    let mut world = HittableList::new();

    // right wall
    world.add(
        Mesh::quad(
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            mirror.clone(),
        )
        .obj(),
    );
    // light
    world.add(Mesh::quad(v3!(343, 554, 332), v3!(-130, 0, 0), v3!(0, 0, -105), light).obj());
    // back wall
    world.add(
        Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        )
        .obj(),
    );
    // left wall
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), mirror.clone()).obj());
    // roof
    world.add(
        Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        )
        .obj(),
    );
    // floor
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(555, 0, 0), v3!(0, 0, 555), white.clone()).obj());

    world.add(
        Mesh::quad(
            v3!(100, 0, 112.5),
            v3!(0, 195, 0),
            v3!(195, 0, 0),
            blue.clone(),
        )
        .obj(),
    );
    world.add(Mesh::volume(v3!(130, 0, 60), v3!(165, 165, 165), red.clone()).obj());

    world.add(Mesh::volume(v3!(265, 0, 295), v3!(165, 330, 165), green.clone()).obj());

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
            max_bounces: 50.,
            samples_per_pixel: 14. * 16.,
            background_color: v3!(0, 0, 0),
        },
    );

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("cornell.ppm")
        .expect("cornell.ppm");

    let writer = BufWriter::new(file);
    // world.into_bvh().log_bboxes();
    camera.render_multi(14, writer, Arc::new(world));
}
