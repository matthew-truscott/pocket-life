use indicatif::ProgressBar;
use std::{fs::File, io::prelude::*, io::LineWriter};

use crate::color::Color;
use crate::hittable::Hittable;
use crate::math::interval::Interval;
use crate::math::random::random;
use crate::math::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,

    // Private
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        // Camera
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let center: Point3 = Point3::new(0.0, 0.0, 0.0);

        // Viewport
        let viewport_u = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height as f64, 0.0);

        // Delta Vectors
        let pixel_delta_u: Vec3 = &viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = &viewport_v / image_height as f64;

        // Upper Left Pixel
        let viewport_upper_left: Point3 =
            &center - Point3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc: Point3 = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

        // Sampling
        let pixel_samples_scale: f64 = 1.0 / (samples_per_pixel as f64);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }

    pub fn default() -> Self {
        Self::new(16.0 / 9.0, 400, 10)
    }

    pub fn render(&self, world: &dyn Hittable) -> Result<(), std::io::Error> {
        // File
        let file = File::create("image.ppm")?;
        let mut file = LineWriter::new(file);

        // Progress
        let bar = ProgressBar::new(self.image_height as u64);

        // Render
        write!(
            file,
            "P3\n{0} {1}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                write!(file, "{}", pixel_color * self.pixel_samples_scale)?;
            }
        }

        bar.finish();

        Ok(())
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        let interval: Interval = Interval::with_bounds(0.0, f64::INFINITY);
        if let Some(hit) = world.hit(r, &interval) {
            return (&hit.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction: Vec3 = r.direction.unit();
        let a: f64 = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = &self.pixel00_loc
            + (&self.pixel_delta_u * ((i as f64) + offset.x()))
            + (&self.pixel_delta_v * ((j as f64) + offset.y()));

        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random() - 0.5, random() - 0.5, 0.0)
    }
}
