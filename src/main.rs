mod camera;
mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;

use env_logger;
use log;

fn main() -> std::io::Result<()> {
    // Logging
    env_logger::init();

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        math::vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        math::vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = camera::Camera::default();
    camera.render(&world)?;

    log::info!("Done!");

    Ok(())
}
