use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::interval::Interval;
use crate::math::vec3::{Point3, Vec3};
use crate::ray::Ray;

use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat: Some(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = &self.center - &r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (&p - &self.center) / self.radius;
        let mut record = HitRecord::new(p, outward_normal.clone(), root, true);
        record.set_face_normal(r, outward_normal);
        record.mat = self.mat.clone();
        Some(record)
    }
}
