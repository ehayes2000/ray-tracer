pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod math;
pub mod ray;
pub mod sphere;
pub mod vec3;

// use camera::{Camera, CameraParameters, RenderParameters};
// use color::write_color;
// use hittable::HitRecord;
// use hittable_list::HittableList;
// use math::{random, random_f64};
// use ray::Ray;
// use sphere::Sphere;
// use vec3::{Color, Vec3};

// fn main() {
//     // random_spheres();
//     // three_sphere();
//     two_sphere();
// }

// fn two_sphere() {
//     let r = f64::cos(f64::consts::PI / 4.0);
//     let mat_l = material::Lambertian::obj(v3!(0, 0, 1));
//     let mat_r = material::Lambertian::obj(v3!(1, 0, 0));
//     let mut world = HittableList::new();

//     world.add(ball!(v3!(-r, 0, -1), r, mat_l));
//     world.add(ball!(v3!(r, 0, -1), r, mat_r));

//     let rparams = RenderParameters::default();
//     let cparams = CameraParameters {
//         look_from: v3!(0, 0, 0),
//         look_at: v3!(0, 0, -1),
//         focus_distance: 10.,
//         ..Default::default()
//     };

//     let cam = Camera::new(cparams, rparams);
//     let f = std::io::stdout();
//     cam.render(f, &world);
// }

// fn three_sphere() {
//     let ground = material::Lambertian::obj(Vec3(0.8, 0.8, 0.0));
//     let left = material::Dielectric::obj(1.5);
//     let bubble = material::Dielectric::obj(1.0 / 1.5);
//     let center = material::Lambertian::obj(Vec3(0.1, 0.2, 0.5));
//     let right = material::Metal::obj(Vec3(0.8, 0.6, 0.2), 1.0);

//     let mut world = HittableList::new();

//     world.add(Sphere::obj(Vec3(0., 0., -1.2), 0.5, center));
//     world.add(Sphere::obj(Vec3(1.0, 0., -1.), 0.5, right));
//     world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.5, left));
//     world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.4, bubble));
//     world.add(Sphere::obj(Vec3(0., -100.5, -1.), 100., ground));

//     let rparams = RenderParameters::default();
//     let cparams = CameraParameters {
//         focus_distance: 3.4,
//         vfov: 20.,
//         look_at: v3!(0, 0, -1),
//         look_from: v3!(-2, 2, 1),
//         defocus_angle: 10.0,
//         ..Default::default()
//     };
//     let cam = Camera::new(cparams, rparams);
//     let f = std::io::stdout();
//     cam.render(f, &world);
// }

// // fn random_spheres() {
// //     // --- World ---
// //     let mut world = HittableList::new();

// //     // Ground
// //     let ground_mat = material::Lambertian::obj(Vec3(0.5, 0.5, 0.5));
// //     world.add(Sphere::obj(v3!(0.0, -1000.0, 0.0), 1000.0, ground_mat));

// //     // Random small spheres
// //     for a in -11..11 {
// //         for b in -11..11 {
// //             let choose_mat = random();
// //             let center = Vec3(
// //                 a as f64 as f64 + 0.9 * random(),
// //                 0.2,
// //                 b as f64 as f64 + 0.9 * random(),
// //             );

// //             if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
// //                 if choose_mat < 0.8 {
// //                     // diffuse
// //                     let albedo = Vec3::unit_random() * Vec3::random_on_disk();
// //                     let mat = material::Lambertian::obj(albedo);
// //                     world.add(Sphere::obj(center, 0.2, mat));
// //                 } else if choose_mat < 0.95 {
// //                     // metal
// //                     let albedo = v3!(
// //                         random_f64(0.5, 1.0),
// //                         random_f64(0.5, 1.0),
// //                         random_f64(0.5, 1.0)
// //                     );
// //                     let fuzz = random_f64(0.0, 0.5);
// //                     let mat = material::Metal::obj(albedo, fuzz);
// //                     world.add(Sphere::obj(center, 0.2, mat));
// //                 } else {
// //                     // glass
// //                     let mat = material::Dielectric::obj(1.5);
// //                     world.add(Sphere::obj(center, 0.2, mat));
// //                 }
// //             }
// //         }
// //     }

// //     // Three large spheres
// //     let material1 = material::Dielectric::obj(1.5);
// //     world.add(Sphere::obj(v3!(0.0, 1.0, 0.0), 1.0, material1));

// //     let material2 = material::Lambertian::obj(Vec3(0.4, 0.2, 0.1));
// //     world.add(Sphere::obj(v3!(-4.0, 1.0, 0.0), 1.0, material2));

// //     let material3 = material::Metal::obj(Vec3(0.7, 0.6, 0.5), 0.0);
// //     world.add(Sphere::obj(v3!(4.0, 1.0, 0.0), 1.0, material3));

// //     // --- Camera (matches the C++ settings) ---
// //     let mut cam = Camera::new(16.0 / 9.0, 1200);
// //     cam.samples_per_pixel = 500;
// //     cam.max_bounces = 50;

// //     cam.vfov = 20.0;
// //     cam.look_from = v3!(13.0, 2.0, 3.0);
// //     cam.look_at = v3!(0.0, 0.0, 0.0);
// //     cam.vup = v3!(0.0, 1.0, 0.0);

// //     cam.defocus_angle = 0.6;
// //     cam.focus_dist = 10.0;

// //     cam.render(&world);
// // }
