#[cfg(test)]
mod tests;

use super::random;
use std::fmt;
use std::ops;

pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn random() -> Self {
        Vec3 {
            e: [random::random(), random::random(), random::random()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3 {
            e: [
                random::random_range(min, max),
                random::random_range(min, max),
                random::random_range(min, max),
            ],
        }
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let p_sq = p.length_squared();
            if 1e-160 < p_sq && p_sq <= 1.0 {
                return p / p_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        v - &(n * 2.0 * Vec3::dot(v, n))
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn add(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }

    pub fn sub(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }

    pub fn mul(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }

    pub fn div(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    #[allow(dead_code)]
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    #[allow(dead_code)]
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.e[0])
            .field("y", &self.e[1])
            .field("z", &self.e[2])
            .finish()
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }

    fn ne(&self, other: &Self) -> bool {
        self.e != other.e
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::add(&self, &rhs)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::add(self, &rhs)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::add(&self, rhs)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::add(self, rhs)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::sub(&self, &rhs)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::sub(self, &rhs)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::sub(&self, rhs)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::sub(self, rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::mul(&self, &rhs)
    }
}

impl ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::mul(self, &rhs)
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::mul(&self, rhs)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::mul(self, rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        let r = 1.0f64 / rhs;
        Vec3 {
            e: [self.e[0] * r, self.e[1] * r, self.e[2] * r],
        }
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        let r = 1.0f64 / rhs;
        Vec3 {
            e: [self.e[0] * r, self.e[1] * r, self.e[2] * r],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 {
            e: [self.e[0], self.e[1], self.e[2]],
        }
    }
}
