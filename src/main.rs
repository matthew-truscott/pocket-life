mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;

use color::Color;
use env_logger;
use hittable::Hittable;
use indicatif::ProgressBar;
use log;
use math::vec3::{Point3, Vec3};
use ray::Ray;
use std::{fs::File, io::prelude::*, io::LineWriter};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit) = world.hit(r, 0.0, f64::INFINITY) {
        return (&hit.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction: Vec3 = r.direction.unit();
    let a: f64 = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() -> std::io::Result<()> {
    // Logging
    env_logger::init();

    // File
    let file = File::create("image.ppm")?;
    let mut file = LineWriter::new(file);

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Viewport
    let viewport_u = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f64, 0.0);

    // Delta Vectors
    let pixel_delta_u: Vec3 = &viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = &viewport_v / image_height as f64;

    // Upper Left Pixel
    let viewport_upper_left: Point3 = &camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);
    let pixel00_loc: Point3 = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    // Progress
    let bar = ProgressBar::new(image_height as u64);

    // Render
    write!(file, "P3\n{0} {1}\n255\n", image_width, image_height)?;

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_center: Point3 =
                &pixel00_loc + (&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction: Vec3 = &pixel_center - &camera_center;
            let ray: Ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color: Color = ray_color(&ray, &world);

            write!(file, "{0}", pixel_color)?;
        }
    }

    bar.finish();
    log::info!("Done!");

    Ok(())
}
