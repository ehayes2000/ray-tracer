mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod math;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    let image_width = 400;

    let mut world = HittableList::new();
    world.add(Sphere::new_boxed(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new_boxed(Vec3(0., -100.5, -1.), 100.));
    let camera = Camera::new(ASPECT_RATIO, image_width);
    camera.render(&world);
}
