use std::sync::Arc;

use crate::{aabb::AABB, material, ray, vec3};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub material: Option<Arc<dyn material::Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, t: f64, ray: &ray::Ray, outward_normal: &vec3::Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            material: None,
            t,
            front_face,
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: vec3::Point3::origin(),
            normal: vec3::Vec3::origin(),
            material: None,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn copy_from(&mut self, other: &HitRecord) {
        self.p = other.p;
        self.normal = other.normal;
        match &other.material {
            Some(material) => self.material = Option::Some(Arc::clone(material)),
            None => self.material = Option::None,
        }
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
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, _ray: &ray::Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
