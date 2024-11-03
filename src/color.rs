use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use std::fmt::Display;

// attribute aliasing for Color
pub trait Rgb {
    fn r(&self) -> f64;
    fn g(&self) -> f64;
    fn b(&self) -> f64;
}

pub type Color = Vec3;

impl Rgb for Color {
    fn r(&self) -> f64 {
        self.e[0]
    }
    fn g(&self) -> f64 {
        self.e[1]
    }
    fn b(&self) -> f64 {
        self.e[2]
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intensity = Interval::with_bounds(0.000, 0.999);
        let ir = (256.0 * intensity.clamp(self.r())) as i32;
        let ig = (256.0 * intensity.clamp(self.g())) as i32;
        let ib = (256.0 * intensity.clamp(self.b())) as i32;
        write!(f, "{0} {1} {2}\n", ir, ig, ib)
    }
}
