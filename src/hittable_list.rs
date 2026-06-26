use crate::hittable::{HitRecord, Hittable};
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
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut result = None;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}