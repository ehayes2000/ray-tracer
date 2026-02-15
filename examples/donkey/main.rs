use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    math::Vec3,
    mesh,
    sphere::Sphere,
    v3,
};

fn main() {
    let ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let metal = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0);

    let world = HittableList::empty()
        .push(mesh!("dk.obj", metal).expect("load mesh"))
        .push(Sphere::new(Vec3(0., -10000., -1.), 10000., ground));

    let rparams = RenderParameters::default();
    let cparams = CameraParameters {
        focus_distance: 3.4,
        vfov: 20.,
        look_at: v3!(0, 12, -1),
        look_from: v3!(1, 12, 100),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let cam = Camera::new(cparams, rparams);
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("donkey.ppm")
        .expect("donkey.ppm");
    cam.render(&mut output_file, world);
}
