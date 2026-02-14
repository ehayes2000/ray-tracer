use std::default::Default;
use std::io::Write;
use std::sync::Arc;

use crate::Float;
use crate::color::to_8bit;
use crate::hittable::Hit;
use crate::math::Interval;
use crate::math::Ray;
use crate::math::degrees_to_radians;
use crate::math::random;
use crate::math::{Color, Point, Vec3};
use crate::math::{cross, unit_vector};
use crate::v3;

#[derive(Debug, Clone)]
pub struct CameraParameters {
    pub look_at: Point,
    pub look_from: Point,
    pub vfov: Float,
    // pub focal_length: Float,
    pub focus_distance: Float,
    pub defocus_angle: Float,
}

impl Default for CameraParameters {
    fn default() -> Self {
        Self {
            look_at: Vec3::zero(),
            look_from: v3!(1, 1, 0),
            vfov: 90.,
            defocus_angle: 0.0,
            // focal_length: 1.0,
            focus_distance: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderParameters {
    pub image_width: Float,
    pub aspect_ratio: Float,
    pub samples_per_pixel: Float,
    pub max_bounces: Float,
    pub background_color: Color,
}

impl Default for RenderParameters {
    fn default() -> Self {
        Self {
            image_width: 400.,
            aspect_ratio: 16.0 / 9.0,
            max_bounces: 20.,
            samples_per_pixel: 100.,
            background_color: v3!(0, 0, 0),
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
    pub pixel_samples_scale: Float,
    pub image_height: Float,
}

impl Camera {
    pub fn new(c_params: CameraParameters, r_params: RenderParameters) -> Self {
        let image_height = ((r_params.image_width / r_params.aspect_ratio).floor()).max(1.);
        let theta = degrees_to_radians(c_params.vfov);
        let h = f64::tan(theta as f64 / 2.0) as Float;
        let viewport_height = 2.0 * h * c_params.focus_distance;
        let viewport_width = viewport_height * r_params.image_width / image_height;

        // parametarize in future?
        let vup = v3!(0, 1, 0);

        let w = unit_vector(&(c_params.look_from - c_params.look_at));
        let u = unit_vector(&cross(vup, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        // world coordinates per pixel (width)
        let pixel_delta_u = viewport_u / r_params.image_width;
        // world coordinates per pixel (height)
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = c_params.look_from
            - (c_params.focus_distance * w)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let defocus_radius =
            c_params.focus_distance * Float::tan(degrees_to_radians(c_params.defocus_angle / 2.));

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

struct Img(pub Vec<Vec<Color>>);

impl std::ops::Add for Img {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for j in 0..self.0.len() {
            for i in 0..self.0[0].len() {
                self.0[j][i] += rhs.0[j][i];
            }
        }
        self
    }
}

impl std::ops::Div<Float> for Img {
    type Output = Self;
    fn div(mut self, rhs: Float) -> Self::Output {
        for j in 0..self.0.len() {
            for i in 0..self.0[0].len() {
                self.0[j][i] /= rhs;
            }
        }
        self
    }
}

impl Camera {
    pub fn render_multi(
        &self,
        n_threads: u16,
        f: impl Write,
        world: Arc<dyn Hit + Send + Sync + 'static>,
    ) {
        let samples_per_worker = self.r_params.samples_per_pixel as i32 / n_threads as i32;
        let mut buf = None;
        for result in (0..n_threads)
            .map(|i| {
                let mut this = self.clone();
                this.r_params.samples_per_pixel = samples_per_worker as _;
                let thread_world = world.clone();
                std::thread::Builder::new()
                    .name(format!("worker-{}", i))
                    .spawn(move || this.render_to_buf(thread_world))
            })
            .collect::<Vec<_>>()
        {
            if let Ok(handle) = result {
                if let Ok(part_buf) = handle.join() {
                    if let Some(img) = buf {
                        buf = Some(img + part_buf);
                    } else {
                        buf = Some(part_buf);
                    }
                }
            }
        }
        let buf = buf.unwrap() / n_threads as Float;
        self.write_to_f(f, buf);
    }

    fn render_to_buf(&self, world: impl Hit) -> Img {
        let mut buf = Vec::with_capacity(self.image_height as usize);
        for j in 0..self.image_height as usize {
            eprint!("\r       ");
            eprint!("\r{}%", ((j as Float / self.image_height) * 100.0).ceil());
            let mut row = Vec::with_capacity(self.r_params.image_width as usize);
            for i in 0..self.r_params.image_width as usize {
                let mut color = Vec3::zero();
                for _ in 0..self.r_params.samples_per_pixel as i64 {
                    let r = self.get_ray(i as Float, j as Float);
                    color += self.ray_color(&r, &world, self.r_params.max_bounces as u32);
                }
                row.push(color / self.r_params.samples_per_pixel);
            }
            buf.push(row);
        }
        Img(buf)
    }

    pub fn render(&self, f: impl Write, world: impl Hit) {
        let buf = self.render_to_buf(world);
        self.write_to_f(f, buf);
    }

    fn write_to_f(&self, mut f: impl Write, buf: Img) {
        write!(
            f,
            "P3\n{} {}\n255\n",
            self.r_params.image_width, self.image_height
        )
        .expect("write failed");
        for j in 0..self.image_height as usize {
            for i in 0..self.r_params.image_width as usize {
                let (r, g, b) = to_8bit(&buf.0[j][i]);
                writeln!(f, "{} {} {}", r, g, b).expect("write");
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
        if let Some(hit) = world.hit(r, &Interval::new(0.001, Float::MAX)) {
            // something reflected ray
            if let Some(scatter) = hit.material.scatter(r, &hit) {
                scatter.color_attenuation
                    * self.ray_color(&scatter.ray, world, remaining_bounces - 1)
                    + hit.material.emit(hit.p)
            } else {
                Color::zero() + hit.material.emit(hit.p)
            }
        } else {
            self.r_params.background_color
        }
    }

    fn get_ray(&self, i: Float, j: Float) -> Ray {
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
