use crate::math::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + (&self.direction * t)
    }
}
