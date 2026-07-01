use std::sync::Arc;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::random::Random;
use crate::ray::Ray;

/// Bounding volume hierarchy node
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);

        let t_max = match &hit_left {
            Some(rec) => rec.t,
            None => ray_t.max,
        };

        let hit_right = self.right.hit(r, Interval::new(ray_t.min, t_max));

        // if hit_right is some, it is closer than hit_left -> use it
        // else return hit_left (some / none)
        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl BvhNode {
    pub fn new(objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize, rng: &mut Random) -> BvhNode {
        // Build the bounding box of the span of source objects.
        let mut bbox = Aabb::empty();
        for object_index in start..end {
            bbox = Aabb::from_aabbs(&bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => { Self::box_x_compare }
            1 => { Self::box_y_compare }
            2 => { Self::box_z_compare }
            _ => { panic!("invalid axis value: {axis}") }
        };

        // let comparator = match axis {
        //     0 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| a.bounding_box().min.x.partial_cmp(&b.bounding_box().min.x).unwrap(),
        //     1 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| a.bounding_box().min.y.partial_cmp(&b.bounding_box().min.y).unwrap(),
        //     _ => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| a.bounding_box().min.z.partial_cmp(&b.bounding_box().min.z).unwrap(),
        // };

        let object_span = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            // only 1 item remaining -> set it to both left and right
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            // exactly 2 items remaining
            left = objects[start].clone();
            right = objects[start+1].clone();
        } else {
            let mut sorted_objects = objects[start..end].to_vec();
            sorted_objects.sort_by(comparator);

            let mid = object_span/2;
            left = Arc::new(Self::new(&sorted_objects, 0, mid, rng));
            right = Arc::new(Self::new(&sorted_objects, mid, object_span, rng));
        }

        BvhNode { left, right, bbox }
    }

    pub fn from_list(list: &HittableList, rng: &mut Random) -> BvhNode {
        Self::new(&list.objects, 0, list.objects.len(), rng)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> std::cmp::Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}
