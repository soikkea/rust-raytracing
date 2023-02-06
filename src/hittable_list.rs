use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittablePtr},
};

pub struct HittableList {
    pub objects: Vec<HittablePtr>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: (Vec::new()),
        }
    }

    pub fn add(&mut self, object: HittablePtr) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, _ray: &crate::ray::Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = _t_max;

        for object in &self.objects {
            if object.hit(_ray, _t_min, closest_so_far, &mut temp_rec) {
                closest_so_far = temp_rec.t();
                hit_anything = true;
                _rec.copy_from(&temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output: Option<Aabb> = None;

        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                None => {
                    return None;
                }
                Some(temp_box) => match output {
                    Some(output_box) => {
                        output = Some(output_box.surrounding_box(&temp_box));
                    }
                    None => {
                        output = Some(temp_box);
                    }
                },
            }
        }

        output
    }
}
