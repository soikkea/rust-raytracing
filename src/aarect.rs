use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::MaterialPtr,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    material: MaterialPtr,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}
pub struct XZRect {
    material: MaterialPtr,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
pub struct YZRect {
    material: MaterialPtr,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: &MaterialPtr) -> XYRect {
        XYRect {
            material: Arc::clone(material),
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: &MaterialPtr) -> XZRect {
        XZRect {
            material: Arc::clone(material),
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: &MaterialPtr) -> YZRect {
        YZRect {
            material: Arc::clone(material),
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension,
        // so pad the Z dimension a small amount.
        let padding = 0.0001;
        let output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - padding),
            Point3::new(self.x1, self.y1, self.k + padding),
        );

        Some(output_box)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Some(Arc::clone(&self.material));
        rec.p = ray.at(t);
        true
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension,
        // so pad the Y dimension a small amount.
        let padding = 0.0001;
        let output_box = AABB::new(
            Point3::new(self.x0, self.k - padding, self.z0),
            Point3::new(self.x1, self.k + padding, self.z1),
        );

        Some(output_box)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Some(Arc::clone(&self.material));
        rec.p = ray.at(t);
        true
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension,
        // so pad the X dimension a small amount.
        let padding = 0.0001;
        let output_box = AABB::new(
            Point3::new(self.k - padding, self.y0, self.z0),
            Point3::new(self.k + padding, self.y1, self.z1),
        );

        Some(output_box)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Some(Arc::clone(&self.material));
        rec.p = ray.at(t);
        true
    }
}
