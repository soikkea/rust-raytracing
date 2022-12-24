use crate::{ray, vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, t: f64, ray: &ray::Ray, outward_normal: &vec3::Vec3) -> HitRecord {
        let front_face = vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: vec3::Point3::origin(),
            normal: vec3::Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn copy_from(&mut self, other: &HitRecord) {
        self.p = other.p;
        self.normal = other.normal;
        self.t = other.t;
        self.front_face = other.front_face;
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> &vec3::Vec3 {
        &self.normal
    }

    pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, _ray: &ray::Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
}
