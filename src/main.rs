mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod ray;
mod sphere;

use std::rc::Rc;

use env_logger;
use log;

fn main() -> std::io::Result<()> {
    // Logging
    env_logger::init();

    // World
    let mut world = hittable_list::HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Metal::new(color::Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(sphere::Sphere::new(
        math::vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(sphere::Sphere::new(
        math::vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(sphere::Sphere::new(
        math::vec3::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(sphere::Sphere::new(
        math::vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    let camera = camera::Camera::default();
    camera.render(&world)?;

    log::info!("Done!");

    Ok(())
}
