use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::HittableList,
    material::{DiffuseLight, Lambertian},
    mesh::Mesh,
    v3,
};

fn main() {
    let red = Lambertian::obj(v3!(0.65, 0.05, 0.05));
    let white = Lambertian::obj(v3!(0.73, 0.73, 0.73));
    let green = Lambertian::obj(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::obj(v3!(15, 15, 15));

    let mut world = HittableList::new();
    world.add(
        Mesh::quad(
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            green.clone(),
        )
        .obj(),
    );
    world.add(Mesh::quad(v3!(343, 554, 332), v3!(-130, 0, 0), v3!(0, 0, -105), light).obj());
    world.add(
        Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        )
        .obj(),
    );
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), red).obj());
    world.add(
        Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        )
        .obj(),
    );
    world.add(Mesh::quad(v3!(0, 0, 0), v3!(555, 0, 0), v3!(0, 0, 555), white.clone()).obj());

    let camera = Camera::new(
        CameraParameters {
            vfov: 40.0,
            look_from: v3!(278, 278, -800),
            look_at: v3!(278, 278, 0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
        },
        RenderParameters {
            image_width: 720.,
            aspect_ratio: 1.0,
            max_bounces: 10.,
            samples_per_pixel: 100.0,
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
    camera.render(writer, world.into_bvh());
}
