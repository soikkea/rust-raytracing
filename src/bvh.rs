use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittablePtr},
    hittable_list::HittableList,
    ray::Ray,
};

pub struct BVHNode {
    pub left: HittablePtr,
    pub right: HittablePtr,
    pub bounding_box: Aabb,
}

impl BVHNode {
    pub fn new(source_objects: &[HittablePtr], time0: f64, time1: f64) -> BVHNode {
        let mut my_objects = Vec::new();

        for object in source_objects {
            my_objects.push(Arc::clone(object));
        }

        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..=2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let objects_size = my_objects.len();

        let (left, right) = match objects_size {
            1 => (Arc::clone(&my_objects[0]), Arc::clone(&my_objects[0])),
            2 => {
                // if my_objects[0] < my_objects[1], else...
                if comparator(&my_objects[0], &my_objects[1]).is_lt() {
                    (Arc::clone(&my_objects[0]), Arc::clone(&my_objects[1]))
                } else {
                    (Arc::clone(&my_objects[1]), Arc::clone(&my_objects[0]))
                }
            }
            _ => {
                my_objects.sort_by(comparator);

                let mid = objects_size / 2;
                let left_tmp: HittablePtr =
                    Arc::new(BVHNode::new(&my_objects[0..mid], time0, time1));
                let right_tmp: HittablePtr =
                    Arc::new(BVHNode::new(&my_objects[mid..], time0, time1));

                (left_tmp, right_tmp)
            }
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            eprintln!("No bounding box in BVHNode constructor.");
        }

        let bounding_box = box_left
            .unwrap_or_else(Aabb::empty)
            .surrounding_box(&box_right.unwrap_or_else(Aabb::empty));

        BVHNode {
            left,
            right,
            bounding_box,
        }
    }

    pub fn from_hittable_list(list: &HittableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::new(&list.objects, time0, time1)
    }
}

fn box_compare(a: &HittablePtr, b: &HittablePtr, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in BVHNode constructor.");
    }

    box_a
        .map_or_else(|| 0.0, |b| b.min().e[axis])
        .total_cmp(&box_b.map_or_else(|| 0.0, |b| b.min().e[axis]))
}

fn box_x_compare(a: &HittablePtr, b: &HittablePtr) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &HittablePtr, b: &HittablePtr) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &HittablePtr, b: &HittablePtr) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let right_t_max = if hit_left { rec.t } else { t_max };
        let hit_right = self.right.hit(ray, t_min, right_t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
