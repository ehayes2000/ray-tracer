use std::io::BufWriter;

use ray_tracer::{
    bvh::BvhBuilder,
    camera::{Camera, CameraParameters, RenderParameters},
    material::Lambertian,
    mesh::Mesh,
    v3,
};

fn main() {
    // Materials
    let left_red = Lambertian::new(v3!(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(v3!(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(v3!(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(v3!(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(v3!(0.2, 0.8, 0.8));

    // Quads
    let world = BvhBuilder::new()
        .mesh(Mesh::quad(
            v3!(-3.0, -2.0, 5.0),
            v3!(0.0, 0.0, -4.0),
            v3!(0.0, 4.0, 0.0),
            left_red,
        ))
        .mesh(Mesh::quad(
            v3!(-2.0, -2.0, 0.0),
            v3!(4.0, 0.0, 0.0),
            v3!(0.0, 4.0, 0.0),
            back_green,
        ))
        .mesh(Mesh::quad(
            v3!(3.0, -2.0, 1.0),
            v3!(0.0, 0.0, 4.0),
            v3!(0.0, 4.0, 0.0),
            right_blue,
        ))
        .mesh(Mesh::quad(
            v3!(-2.0, 3.0, 1.0),
            v3!(4.0, 0.0, 0.0),
            v3!(0.0, 0.0, 4.0),
            upper_orange,
        ))
        .mesh(Mesh::quad(
            v3!(-2.0, -3.0, 5.0),
            v3!(4.0, 0.0, 0.0),
            v3!(0.0, 0.0, -4.0),
            lower_teal,
        ));

    let cparams = CameraParameters {
        vfov: 80.0,
        look_from: v3!(0.0, 0.0, 9.0),
        look_at: v3!(0.0, 0.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    };

    let rparams = RenderParameters {
        image_width: 400.0,
        aspect_ratio: 1.0,
        samples_per_pixel: 20.0,
        max_bounces: 5.0,
        background_color: v3!(1, 1, 1),
    };

    let camera = Camera::new(cparams, rparams);

    let output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("box.ppm")
        .expect("box.ppm");

    let mut writer = BufWriter::new(output_file);
    camera.render(&mut writer, world.build());
}
