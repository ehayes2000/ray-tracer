use ray_tracer::{
    ball,
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    math::{random, random_f64},
    sphere::Sphere,
    v3,
    vec3::Vec3,
};

fn main() {
    let mut world = HittableList::new();
    let ground_m = Lambertian::obj(v3!(0.5, 0.5, 0.5));
    world.add(ball!(v3!(0, -1000, 0), 1000., ground_m));
    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;
            let mat = random();
            let center = v3!(a + 0.9 * random(), 0.2, b + 0.9 * random());
            if (center - v3!(4, 0.2, 0)).len() > 0.9 {
                let material = if mat < 0.8 {
                    let color = Vec3::unit_random() * Vec3::unit_random();
                    Lambertian::obj(color)
                } else if mat < 0.95 {
                    let color = Vec3::random_mm(0.5, 1.0);
                    let fuzz = random_f64(0., 0.5);
                    Metal::obj(color, fuzz)
                } else {
                    Dielectric::obj(1.5)
                };
                world.add(ball!(center, 0.2, material))
            }
        }
    }
    world.add(ball!(v3!(0, 1, 0), 1.0, Dielectric::obj(1.5)));
    world.add(ball!(
        v3!(-4, 1, 0),
        1.0,
        Lambertian::obj(v3!(0.4, 0.2, 0.1))
    ));
    world.add(ball!(
        v3!(4, 1, 0),
        1.0,
        Metal::obj(v3!(0.7, 0.6, 0.5), 0.0)
    ));
    let render_params = RenderParameters {
        aspect_ratio: 16. / 9.,
        image_width: 1200.,
        samples_per_pixel: 500.,
        max_bounces: 50.,
    };
    let camera_params = CameraParameters {
        vfov: 20.,
        look_from: v3!(13, 2, 3),
        look_at: v3!(0, 0, 0),
        defocus_angle: 0.6,
        focus_distance: 10.,
        focal_length: 1.0,
    };
    let camera = Camera::new(camera_params, render_params);
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("random_spheres.ppm")
        .expect("random_spheres.ppm");
    camera.render(&mut output_file, &world);
}
