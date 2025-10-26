use std::default::Default;
use std::io::Write;

use crate::color::write_color;
use crate::hittable::Hit;
use crate::interval::Interval;
use crate::math::degrees_to_radians;
use crate::math::random;
use crate::ray::Ray;
use crate::v3;
use crate::vec3::{Color, Point, Vec3};
use crate::vec3::{cross, unit_vector};

#[derive(Debug, Clone)]
pub struct CameraParameters {
    pub look_at: Point,
    pub look_from: Point,
    pub vfov: f64,
    pub focal_length: f64,
    pub focus_distance: f64,
    pub defocus_angle: f64,
}

impl Default for CameraParameters {
    fn default() -> Self {
        Self {
            look_at: Vec3::zero(),
            look_from: v3!(1, 1, 0),
            vfov: 90.,
            defocus_angle: 0.0,
            focal_length: 1.0,
            focus_distance: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderParameters {
    pub image_width: f64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: f64,
    pub max_bounces: f64,
}

impl Default for RenderParameters {
    fn default() -> Self {
        Self {
            image_width: 400.,
            aspect_ratio: 16.0 / 9.0,
            max_bounces: 50.,
            samples_per_pixel: 100.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub r_params: RenderParameters,
    pub c_params: CameraParameters,
    pub pixel_00_loc: Point,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_samples_scale: f64,
    pub image_height: f64,
}

impl Camera {
    pub fn new(c_params: CameraParameters, r_params: RenderParameters) -> Self {
        let image_height = ((r_params.image_width / r_params.aspect_ratio).floor()).max(1.);
        let theta = degrees_to_radians(c_params.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * c_params.focus_distance;
        let viewport_width = viewport_height * r_params.image_width / image_height;

        // parametarize in future?
        let vup = v3!(0, 1, 0);

        let w = unit_vector(&(c_params.look_from - c_params.look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_delta_u = viewport_u / r_params.image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = c_params.look_from
            - (c_params.focus_distance * w)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let defocus_radius =
            c_params.focus_distance * f64::tan(degrees_to_radians(c_params.defocus_angle / 2.));

        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        let pixel_samples_scale = 1.0 / r_params.samples_per_pixel;

        Self {
            c_params,
            r_params,
            pixel_00_loc,
            defocus_disk_u,
            defocus_disk_v,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            image_height,
        }
    }
}

impl Camera {
    pub fn render(&self, mut f: impl Write, world: impl Hit) {
        write!(
            f,
            "P3\n{} {}\n255\n",
            self.r_params.image_width, self.image_height
        )
        .expect("write failed");
        for j in 0..self.image_height as i64 {
            eprint!("\r       ");
            eprint!("\r{}%", ((j as f64 / self.image_height) * 100.0).ceil());
            for i in 0..self.r_params.image_width as i64 {
                let mut color = Vec3::zero();
                for _ in 0..self.r_params.samples_per_pixel as i64 {
                    let r = self.get_ray(i as f64, j as f64);
                    color += self.ray_color(&r, &world, self.r_params.max_bounces as u32);
                }
                write_color(&mut f, &(color * self.pixel_samples_scale)).expect("io error");
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

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i + offset.0) * self.pixel_delta_u)
            + ((j + offset.0) * self.pixel_delta_v);

        let ray_origin = if self.c_params.defocus_angle <= 0. {
            self.c_params.look_from
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        Ray {
            direction: ray_direction,
            origin: ray_origin,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3(random() - 0.5, random() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_on_disk();
        self.c_params.look_from + (p.0 * self.defocus_disk_u) + (p.1 * self.defocus_disk_v)
    }
}
