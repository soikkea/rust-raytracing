use crate::{ray, vec3};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, _ray: &ray::Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
}
