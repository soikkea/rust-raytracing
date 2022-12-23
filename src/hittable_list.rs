use crate::hittable::{self, HitRecord};

pub struct HittableList {
    pub objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: (Vec::new()),
        }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &self,
        _ray: &crate::ray::Ray,
        _t_min: f64,
        _t_max: f64,
    ) -> Option<hittable::HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = _t_max;

        for object in &self.objects {
            let object_hit_result = object.hit(_ray, _t_min, closest_so_far);
            if object_hit_result.is_some() {
                closest_so_far = object_hit_result.unwrap().t();
                result = object_hit_result;
            }
        }

        result
    }
}
