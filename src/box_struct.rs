use std::sync::Arc;

use crate::{
    aabb::AABB,
    aarect::{XYRect, XZRect, YZRect},
    hittable::Hittable,
    hittable_list::HittableList,
    material::MaterialPtr,
    vec3::Point3,
};

pub struct Box {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Box {
    pub fn new(min: &Point3, max: &Point3, material: &MaterialPtr) -> Box {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material,
        )));
        sides.add(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material,
        )));

        sides.add(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material,
        )));
        sides.add(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material,
        )));

        sides.add(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material,
        )));
        sides.add(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material,
        )));

        Box {
            min: *min,
            max: *max,
            sides,
        }
    }
}

impl Hittable for Box {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(self.min, self.max))
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }
}
