use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    v3,
    vec3::Vec3,
};

fn main() {
    let ground = Lambertian::obj(Vec3(0.8, 0.8, 0.0));
    let left = Dielectric::obj(1.5);
    let bubble = Dielectric::obj(1.0 / 1.5);
    let center = Lambertian::obj(Vec3(0.1, 0.2, 0.5));
    let right = Metal::obj(Vec3(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::new();

    world.add(Sphere::obj(Vec3(0., 0., -1.2), 0.5, center));
    world.add(Sphere::obj(Vec3(1.0, 0., -1.), 0.5, right));
    world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.5, left));
    world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.4, bubble));
    world.add(Sphere::obj(Vec3(0., -100.5, -1.), 100., ground));

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
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("three_spheres.ppm")
        .expect("three_spheres.ppm");
    cam.render(&mut output_file, &world);
}
