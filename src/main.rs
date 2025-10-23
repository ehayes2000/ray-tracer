#[allow(unused, dead_code)]
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod math;
mod ray;
mod sphere;
mod vec3;

use std::f64;

use camera::Camera;
use color::write_color;
use hittable::HitRecord;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

const ASPECT_RATIO: f64 = 16. / 9.;

fn main() {
    three_sphere();
}

fn two_sphere() {
    let r = f64::cos(f64::consts::PI / 4.0);
    let mat_l = material::Lambertian::obj(v3!(0, 0, 1));
    let mat_r = material::Lambertian::obj(v3!(1, 0, 0));
    let mut world = HittableList::new();
    world.add(ball!(v3!(-r, 0, -1), r, mat_l));
    world.add(ball!(v3!(r, 0, -1), r, mat_r));
    let mut cam = Camera::new(ASPECT_RATIO, 400);
    cam.look_from = v3!(-2, 2, 1);
    cam.look_at = v3!(0, 0, -1);
    cam.vup = v3!(0, 1, 0);

    cam.render(&world);
}

fn three_sphere() {
    let image_width = 400;
    let ground = material::Lambertian::obj(Vec3(0.8, 0.8, 0.0));
    let left = material::Dielectric::obj(1.5);
    let bubble = material::Dielectric::obj(1.0 / 1.5);
    let center = material::Lambertian::obj(Vec3(0.1, 0.2, 0.5));
    let right = material::Metal::obj(Vec3(0.8, 0.6, 0.2), 1.0);

    // let matt = material::Lambertian::obj(Vec3(0., 0.4, 0.6));
    // let matt_gray = material::Lambertian::obj(Vec3(0.4, 0.5, 0.4));
    // // let mirror = material::Metal::new_rc(Vec3(0.5, 0.5, 0.5));
    // let glass = material::Dialectic::obj(1.5);
    let mut world = HittableList::new();

    world.add(Sphere::obj(Vec3(0., 0., -1.2), 0.5, center));
    world.add(Sphere::obj(Vec3(1.0, 0., -1.), 0.5, right));
    world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.5, left));
    world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.4, bubble));
    world.add(Sphere::obj(Vec3(0., -100.5, -1.), 100., ground));

    let mut cam = Camera::new(ASPECT_RATIO, 400);
    cam.look_from = v3!(-2, 2, 1);
    cam.look_at = v3!(0, 0, -1);
    cam.vup = v3!(0, 1, 0);
    cam.vfov = 20.;

    cam.render(&world);
}
