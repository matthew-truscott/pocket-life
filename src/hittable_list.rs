use crate::hittable::{HitRecord, Hittable};
use crate::math::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            let interval = Interval::with_bounds(ray_t.min, closest_so_far);
            if let Some(record) = object.hit(r, &interval) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
