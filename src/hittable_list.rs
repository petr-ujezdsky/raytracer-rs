use std::sync::Arc;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

/// List of hittable objects.
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new<T: Hittable + 'static>(object: T) -> HittableList {
        HittableList { objects: vec![Arc::new(object)], bbox: Aabb::empty() }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        let bbox = object.bounding_box();
        self.bbox = Aabb::from_aabbs(&self.bbox, bbox);

        self.objects.push(Arc::new(object));
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

    fn bounding_box(&self) -> &Aabb { &self.bbox }
}