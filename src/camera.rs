use crate::vec3::unit_vector;
use std::io::stdout;

use super::hittable::Hit;
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::{Color, Point, Vec3};
use super::write_color;
pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio).floor() as i32).max(1);
        let center = Point::zero();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_height,
            image_width,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }
}

impl Camera {
    pub fn render<T>(&self, world: T)
    where
        T: Hit,
    {
        let mut stdout = stdout().lock();
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let direction = pixel_center - self.center;
                let r = Ray {
                    origin: self.center,
                    direction,
                };
                let pixel_color = self.ray_color(&r, &world);
                write_color(&mut stdout, &pixel_color).expect("io error");
            }
        }
    }

    pub fn ray_color<T>(&self, r: &Ray, world: &T) -> Color
    where
        T: Hit,
    {
        if let Some(hit) = world.hit(r, &Interval::new(0.0, f64::MAX)) {
            0.5 * (hit.normal + Color::one())
        } else {
            let unit_direction = unit_vector(&r.direction);
            let a = 0.5 * (unit_direction.1 + 1.0);
            (1.0 - a) * Color::one() + (a * Vec3(0.5, 0.7, 1.0))
        }
    }
}
