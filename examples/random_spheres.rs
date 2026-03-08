use ray_tracer::{
    ball,
    bvh::BvhBuilder,
    camera::{Camera, CameraParameters, RenderParameters},
    material::{Dielectric, Lambertian, Metal},
    math::{Float, Vec3, random, random_float},
    sphere::Sphere,
    v3,
};
use std::sync::Arc;

fn main() {
    let ground_m = Lambertian::new(v3!(0.5, 0.5, 0.5));
    let mut world = BvhBuilder::new().sphere(ball!(v3!(0, -1000, 0), 1000., ground_m));
    for a in -11..11 {
        let a = a as Float;
        for b in -11..11 {
            let b = b as Float;
            let mat = random();
            let center = v3!(a + 0.9 * random(), 0.2, b + 0.9 * random());
            if (center - v3!(4, 0.2, 0)).len() > 0.9 {
                if mat < 0.8 {
                    let color = Vec3::unit_random() * Vec3::unit_random();
                    world = world.sphere(ball!(center, 0.2, Lambertian::new(color)));
                } else if mat < 0.95 {
                    let color = Vec3::random_mm(0.5, 1.0);
                    let fuzz = random_float(0., 0.5);
                    world = world.sphere(ball!(center, 0.2, Metal::new(color, fuzz)));
                } else {
                    world = world.sphere(ball!(center, 0.2, Dielectric::new(1.5)));
                };
            }
        }
    }
    let world = world
        .sphere(ball!(v3!(0, 1, 0), 1.0, Dielectric::new(1.5)))
        .sphere(ball!(
            v3!(-4, 1, 0),
            1.0,
            Lambertian::new(v3!(0.4, 0.2, 0.1))
        ))
        .sphere(ball!(
            v3!(4, 1, 0),
            1.0,
            Metal::new(v3!(0.7, 0.6, 0.5), 0.0)
        ));
    let render_params = RenderParameters {
        aspect_ratio: 16. / 9.,
        image_width: 1200.,
        samples_per_pixel: 14.,
        max_bounces: 15.,
        background_color: v3!(0, 0, 0),
    };
    let camera_params = CameraParameters {
        vfov: 20.,
        look_from: v3!(13, 2, 3),
        look_at: v3!(0, 0, 0),
        defocus_angle: 0.6,
        focus_distance: 10.,
        // focal_length: 1.0,
    };
    let camera = Camera::new(camera_params, render_params);
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("random_spheres.ppm")
        .expect("random_spheres.ppm");

    camera.render_multi(14, &mut output_file, Arc::new(world.build()));
}
