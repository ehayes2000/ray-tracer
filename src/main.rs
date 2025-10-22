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

use camera::Camera;
use color::write_color;
use hittable::HitRecord;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    let image_width = 400;
    let matt = material::Lambertian::obj(Vec3(0., 0.4, 0.6));
    let matt_gray = material::Lambertian::obj(Vec3(0.4, 0.5, 0.4));
    // let mirror = material::Metal::new_rc(Vec3(0.5, 0.5, 0.5));
    let glass = material::Dialectic::obj(1.5);
    let mut world = HittableList::new();
    world.add(Sphere::obj(Vec3(0., 0., -1.), 0.5, glass.clone()));
    world.add(Sphere::obj(Vec3(-1.0, 0., -1.), 0.5, matt.clone()));
    world.add(Sphere::obj(Vec3(1.0, 0., -1.), 0.5, glass.clone()));
    world.add(Sphere::obj(Vec3(0., -100.5, -1.), 100., matt_gray.clone()));
    let camera = Camera::new(ASPECT_RATIO, image_width);
    camera.render(&world);
}
