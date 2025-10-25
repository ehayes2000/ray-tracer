use ray_tracer::{
    ball,
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::HittableList,
    material::Lambertian,
    sphere::Sphere,
    v3,
    vec3::Vec3,
};

fn main() {
    let r = f64::cos(std::f64::consts::PI / 4.0);
    let mat_l = Lambertian::obj(v3!(0, 0, 1));
    let mat_r = Lambertian::obj(v3!(1, 0, 0));
    let mut world = HittableList::new();

    world.add(ball!(v3!(-r, 0, -1), r, mat_l));
    world.add(ball!(v3!(r, 0, -1), r, mat_r));

    let rparams = RenderParameters::default();
    let cparams = CameraParameters {
        look_from: v3!(0, 0, 0),
        look_at: v3!(0, 0, -1),
        focus_distance: 10.,
        ..Default::default()
    };

    let cam = Camera::new(cparams, rparams);
    let mut f = Vec::<u8>::with_capacity(1024 * 1024);
    cam.render(&mut f, &world);
    std::fs::write("calibration.ppm", f).expect("write to file");
}
