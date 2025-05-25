mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use hittable::{Hit, HitRecord};
use hittable_list::HittableList;
use ray::{Point3, Ray};
use sphere::Sphere;
use std::io::stdout;
use vec3::{Color, Vec3, dot, unit_vector};

// fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> Option<f64> {
//     let oc = center - r.origin;
//     let a = r.direction.len_squared();
//     let h = dot(&r.direction, &oc);
//     let c = oc.len_squared() - (radius * radius);
//     let descriminent = h * h - a * c;
//     if descriminent < 0. {
//         None
//     } else {
//         Some(h - f64::sqrt(descriminent) / a)
//     }
// }

fn ray_color<T>(r: &Ray, world: T) -> Color
where
    T: Hit,
{
    let rec = HitRecord::zero();
    if let Some(t) = world.hit(r, 0., f64::MAX, rec) {
        return 0.5 * (t.normal + Color::one());
    }
    let unit_dir = unit_vector(&r.direction);
    let a = 0.5 * (unit_dir.y() + 1.);
    (1. - a) * Vec3(1., 1., 1.) + (a * Vec3(0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO).floor() as i32;
    let image_height = image_height.max(1);

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::zero();

    let viewport_u = Vec3(viewport_width, 0., 0.);
    let viewport_v = Vec3(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    // eprintln!("viewport width {}", viewport_width);
    // eprintln!("viewport height {}", viewport_height);
    // eprintln!("width delta {}", pixel_delta_u);
    // eprintln!("height delta {}", pixel_delta_v);
    let viewport_upper_left =
        camera_center - Vec3(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);

    let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut stdout = stdout().lock();
    let mut world = HittableList::new();
    world.add(Sphere::new_boxed(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new_boxed(Vec3(0., -100.5, -1.), 100.));

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (pixel_delta_v * j as f64);

            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(&r, &world);
            // if pixel_color == Vec3(1., 0., 0.) {
            //     eprintln!("{}", pixel_center);
            // }

            write_color(&mut stdout, &pixel_color).expect("write color");
        }
    }
}
