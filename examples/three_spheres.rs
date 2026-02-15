use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    math::Vec3,
    sphere::Sphere,
    v3,
};

fn main() {
    let ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let left = Dielectric::new(1.5);
    let bubble = Dielectric::new(1.0 / 1.5);
    let center = Lambertian::new(Vec3(0.1, 0.2, 0.5));
    let right = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0);

    let world = HittableList::empty()
        .push(Sphere::new(Vec3(0., 0., -1.2), 0.5, center))
        .push(Sphere::new(Vec3(1.0, 0., -1.), 0.5, right))
        .push(Sphere::new(Vec3(-1.0, 0., -1.), 0.5, left))
        .push(Sphere::new(Vec3(-1.0, 0., -1.), 0.4, bubble))
        .push(Sphere::new(Vec3(0., -100.5, -1.), 100., ground));
    let bvh = world.into_bvh();

    let rparams = RenderParameters::default();
    let cparams = CameraParameters {
        focus_distance: 3.4,
        vfov: 20.,
        look_at: v3!(0, 0, -1),
        look_from: v3!(-2, 2, 1),
        defocus_angle: 10.0,
        ..Default::default()
    };
    let cam = Camera::new(cparams, rparams);
    let output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("three_spheres.ppm")
        .expect("three_spheres.ppm");
    let mut writer = std::io::BufWriter::new(output_file);
    cam.render_multi(14, &mut writer, std::sync::Arc::new(bvh));
}
