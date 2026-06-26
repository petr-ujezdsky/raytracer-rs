use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

/// List of hittable objects.
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new<T: Hittable + 'static>(object: T) -> HittableList {
        HittableList { objects: vec![Box::new(object)] }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) { self.objects.clear();    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut search_range_t = ray_t;
        let mut result = None;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, search_range_t) {
                search_range_t = Interval::new(ray_t.min, rec.t);
                result = Some(rec);
            }
        }

        result
    }
}