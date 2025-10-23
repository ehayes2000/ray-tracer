use crate::math::degrees_to_radians;
use crate::vec3::{cross, unit_vector};
use std::io::stdout;

use super::hittable::Hit;
use super::interval::Interval;
use super::math::random;
use super::ray::Ray;
use super::vec3::{Color, Point, Vec3};
use super::write_color;
use crate::v3;

const FOV: f64 = 90.0;
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_BOUNCES: i32 = 50;

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
    aspect_ratio: f64,
    pub vfov: f64,
    pub vup: Vec3,
    pub look_from: Vec3,
    pub look_at: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel00_loc: Point::zero(),
            samples_per_pixel: 0,
            pixel_samples_scale: 0.0,
            max_bounces: 0,
            vfov: FOV,
            vup: Vec3::zero(),
            look_at: Vec3::zero(),
            look_from: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio).floor() as i32).max(1);
        self.center = self.look_from;
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * FOCAL_LENGTH;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        let focal_length = FOCAL_LENGTH;

        self.w = unit_vector(&(self.look_from - self.look_at));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel00_loc = pixel00_loc;
        self.samples_per_pixel = SAMPLES_PER_PIXEL;
        self.pixel_samples_scale = 1.0 / SAMPLES_PER_PIXEL as f64;
        self.max_bounces = MAX_BOUNCES;
        self.vfov = self.vfov;
    }
}

impl Camera {
    pub fn render<T>(&mut self, world: T)
    where
        T: Hit,
    {
        self.initialize();
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

        // ray hit something
        if let Some(hit) = world.hit(r, &Interval::new(0.001, f64::MAX)) {
            // something reflected ray
            if let Some(scatter) = hit.material.scatter(r, &hit) {
                scatter.color_attenuation
                    * self.ray_color(&scatter.ray, world, remaining_bounces - 1)
            } else {
                Color::zero()
            }
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
