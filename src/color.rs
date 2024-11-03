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

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intensity = Interval::with_bounds(0.000, 0.999);
        let r = linear_to_gamma(self.r());
        let g = linear_to_gamma(self.g());
        let b = linear_to_gamma(self.b());
        let rb = (256.0 * intensity.clamp(r)) as i32;
        let gb = (256.0 * intensity.clamp(g)) as i32;
        let bb = (256.0 * intensity.clamp(b)) as i32;
        write!(f, "{0} {1} {2}\n", rb, gb, bb)
    }
}
