use crate::math::vec3::{Point3, Vec3};

pub struct Ray<'a, 'b> {
    pub origin: &'a Point3,
    pub direction: &'b Vec3,
}

impl<'a, 'b> Ray<'a, 'b> {
    pub fn new(origin: &'a Point3, direction: &'b Vec3) -> Self {
        Self { origin, direction }
    }
    fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}
