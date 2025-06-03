use crate::vec3::unit_vector;
use std::io::stdout;

use super::hittable::Hit;
use super::interval::Interval;
use super::math::random;
use super::ray::Ray;
use super::vec3::{Color, Point, Vec3};
use super::write_color;

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_BOUNCES: i32 = 10;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_bounces: i32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio).floor() as i32).max(1);
        let center = Point::zero();
        let focal_length = FOCAL_LENGTH;
        let viewport_height = VIEWPORT_HEIGHT;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let samples_per_pixel = SAMPLES_PER_PIXEL;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            image_height,
            image_width,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            pixel_samples_scale,
            max_bounces: MAX_BOUNCES,
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
            eprint!("\r       ");
            eprint!(
                "\r{}%",
                ((j as f32 / self.image_height as f32) * 100.0).ceil()
            );
            for i in 0..self.image_width {
                let mut color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color += self.ray_color(&r, &world, self.max_bounces as u32);
                }
                write_color(&mut stdout, &(color * self.pixel_samples_scale)).expect("io error");
            }
        }
        eprintln!();
    }

    pub fn ray_color<T>(&self, r: &Ray, world: &T, remaining_bounces: u32) -> Color
    where
        T: Hit,
    {
        if remaining_bounces == 0 {
            return Vec3::zero();
        }
        if let Some(hit) = world.hit(r, &Interval::new(0.001, f64::MAX)) {
            let direction = hit.normal + Vec3::unit_random();
            let ray = Ray {
                direction,
                origin: hit.p,
            };
            0.5 * self.ray_color(&ray, world, remaining_bounces - 1)
        } else {
            let unit_direction = unit_vector(&r.direction);
            let a = 0.5 * (unit_direction.1 + 1.0);
            (1.0 - a) * Color::one() + (a * Vec3(0.5, 0.7, 1.0))
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.0) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray {
            direction: ray_direction,
            origin: ray_origin,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3(random() - 0.5, random() - 0.5, 0.0)
    }
}
