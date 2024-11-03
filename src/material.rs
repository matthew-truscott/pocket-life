use crate::color::Color;
use crate::hittable::HitRecord;
use crate::math::vec3::Vec3;
use crate::ray::Ray;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = &rec.normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some(ScatterRecord {
            attenuation,
            ray: scattered,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&r_in.direction.unit(), &rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected);
        let attenuation = self.albedo.clone();
        Some(ScatterRecord {
            attenuation,
            ray: scattered,
        })
    }
}
