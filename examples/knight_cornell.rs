use std::io::BufWriter;

use ray_tracer::{
    bvh::BvhBuilder,
    camera::{Camera, CameraParameters, RenderParameters},
    material::{Dielectric, DiffuseLight, Lambertian},
    mesh::Mesh,
    mesh_obj, v3,
};
use std::sync::Arc;

fn main() {
    // Materials
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05));
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73));
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(v3!(15, 15, 15));
    let glass = Dielectric::new(1.7);

    let model = mesh_obj!("models/chess_knight.obj", glass)
        .expect("knight")
        .scale(300.)
        .translate(v3!(200, 150.0 - 12.929, 200))
        .rotate(v3!(0, 3.1415 * 0.25, 0));

    let bvh = BvhBuilder::new()
        // right wall
        .mesh(Mesh::quad(
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            green,
        ))
        // light
        .mesh(Mesh::quad(
            v3!(343, 554, 332),
            v3!(-130, 0, 0),
            v3!(0, 0, -105),
            light,
        ))
        // back wall
        .mesh(Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        ))
        // left wall
        .mesh(Mesh::quad(
            v3!(0, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            red,
        ))
        // roof
        .mesh(Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        ))
        // floor
        .mesh(Mesh::quad(
            v3!(0, 0, 0),
            v3!(555, 0, 0),
            v3!(0, 0, 555),
            white,
        ))
        .mesh(model)
        .build();

    let camera = Camera::new(
        CameraParameters {
            vfov: 40.,
            look_from: v3!(278, 278, -800),
            look_at: v3!(278, 278, 0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
        },
        RenderParameters {
            image_width: 1920.,
            aspect_ratio: 1.0,
            max_bounces: 15.,
            samples_per_pixel: 14. * 1000.,
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
    camera.render_multi(14, &mut writer, Arc::new(bvh));
}
