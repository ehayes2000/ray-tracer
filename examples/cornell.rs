use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    bvh::BvhBuilder,
    camera::{Camera, CameraParameters, RenderParameters},
    material::{DiffuseLight, Lambertian, Metal},
    mesh::Mesh,
    v3,
};

use std::sync::Arc;

fn main() {
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05));
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73));
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(v3!(15, 15, 15));
    let mirror = Metal::new(v3!(1, 1, 1), 0.1);

    let world = BvhBuilder::new()
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
            red.clone(),
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
            white.clone(),
        ))
        .mesh(
            Mesh::volume(v3!(130, 0, 60), v3!(165, 165, 165), mirror.clone())
                .rotate(v3!(0, -0.3, 0)),
        )
        .mesh(
            Mesh::volume(v3!(265, 0, 295), v3!(165, 330, 165), mirror.clone())
                .rotate(v3!(0, 0.3, 0)),
        );

    let camera = Camera::new(
        CameraParameters {
            vfov: 40.,
            look_from: v3!(278, 278, -800),
            look_at: v3!(278, 278, 0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
        },
        RenderParameters {
            image_width: 1080.,
            aspect_ratio: 1.0,
            max_bounces: 25.,
            samples_per_pixel: 14. * 50.,
            background_color: v3!(0, 0, 0),
        },
    );

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("cornell.ppm")
        .expect("cornell.ppm");

    let writer = BufWriter::new(file);
    camera.render_multi(14, writer, Arc::new(world.build()));
}
