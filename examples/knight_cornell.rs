use std::io::BufWriter;

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable_list::{Hitify, HittableList},
    material::{Dielectric, DiffuseLight, Lambertian, Materialify},
    mesh::Mesh,
    mesh_obj, v3,
};

fn main() {
    // Materials
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05));
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73)).materialify();
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(v3!(15, 15, 15));
    let glass = Dielectric::new(1.75);

    let mut objects = vec![
        // right wall
        Mesh::quad(v3!(555, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), green),
        // light
        Mesh::quad(v3!(343, 554, 332), v3!(-130, 0, 0), v3!(0, 0, -105), light),
        // back wall
        Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        ),
        // left wall
        Mesh::quad(v3!(0, 0, 0), v3!(0, 555, 0), v3!(0, 0, 555), red),
        // roof
        Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        ),
        // floor
        Mesh::quad(v3!(0, 0, 0), v3!(555, 0, 0), v3!(0, 0, 555), white),
    ];

    objects.push(
        mesh_obj!("../models/chess_knight.obj", glass)
            .expect("knight")
            .scale(250.)
            .translate(v3!(225, 125, 200))
            .rotate(v3!(0, 3.1415 * 0.25, 0)),
    );

    let world = HittableList::new(objects.into_iter().map(|o| o.hitify()).collect());

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
            max_bounces: 25.,
            samples_per_pixel: 14. * 200.,
            background_color: v3!(0, 0, 0),
        },
    );

    let output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("knight_cornell.ppm")
        .expect("knight_cornell.ppm");

    let mut writer = BufWriter::new(output_file);
    camera.render(&mut writer, world.into_bvh());
}
