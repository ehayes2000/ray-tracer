use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable_list::HittableList,
    material::{DiffuseLight, Lambertian, Materialify},
    mesh::Mesh,
    v3,
};
use std::sync::Arc;

fn main() {
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05)).materialify();
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73)).materialify();
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(v3!(15, 15, 15));

    let world = HittableList::empty()
        // right wall
        .push(Mesh::quad(
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            green,
        ))
        // light
        .push(Mesh::quad(
            v3!(343, 554, 332),
            v3!(-130, 0, 0),
            v3!(0, 0, -105),
            light,
        ))
        // back wall
        .push(Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        ))
        // left wall
        .push(Mesh::quad(
            v3!(0, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            red.clone(),
        ))
        // roof
        .push(Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        ))
        // floor
        .push(Mesh::quad(
            v3!(0, 0, 0),
            v3!(555, 0, 0),
            v3!(0, 0, 555),
            white.clone(),
        ))
        .push(
            Mesh::volume(v3!(130, 0, 60), v3!(165, 165, 165), white.clone())
                .rotate(v3!(0, -0.3, 0)),
        )
        .push(
            Mesh::volume(v3!(265, 0, 295), v3!(165, 330, 165), white.clone())
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
            image_width: 600.,
            aspect_ratio: 1.0,
            max_bounces: 25.,
            samples_per_pixel: 14. * 2.,
            background_color: v3!(0, 0, 0),
        },
    );

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("cornell.ppm")
        .expect("cornell.ppm");

    let writer = BufWriter::new(file);
    camera.render_multi(14, writer, Arc::new(world.into_bvh()));
}
