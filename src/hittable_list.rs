use crate::hittable;

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
        _rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = hittable::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = _t_max;

        for object in &self.objects {
            if object.hit(_ray, _t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                _rec.clone_from(&temp_rec);
            }
        }

        hit_anything
    }
}
