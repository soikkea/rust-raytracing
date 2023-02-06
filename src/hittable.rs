use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material,
    ray::{self, Ray},
    vec3::{self, Point3, Vec3},
};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub material: Option<Arc<dyn material::Material>>,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        p: vec3::Point3,
        t: f64,
        ray: &ray::Ray,
        outward_normal: &vec3::Vec3,
        u: f64,
        v: f64,
    ) -> HitRecord {
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
            u,
            v,
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: vec3::Point3::origin(),
            normal: vec3::Vec3::origin(),
            material: None,
            t: 0.0,
            front_face: false,
            u: 0.0,
            v: 0.0,
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
        self.u = other.u;
        self.v = other.v;
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

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: vec3::Point3::origin(),
            normal: vec3::Vec3::origin(),
            material: None,
            t: 0.0,
            front_face: false,
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, _ray: &ray::Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub type HittablePtr = Arc<dyn Hittable>;
pub struct Translate {
    hittable: HittablePtr,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: &HittablePtr, displacement: &Vec3) -> Translate {
        Translate {
            hittable: Arc::clone(hittable),
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self.hittable.bounding_box(time0, time1) {
            Some(output_box) => {
                let output_box = Aabb::new(
                    output_box.min() + self.offset,
                    output_box.max() + self.offset,
                );
                Some(output_box)
            }
            None => None,
        }
    }

    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if !self.hittable.hit(&moved_ray, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let outward_normal = rec.normal;
        rec.set_face_normal(&moved_ray, &outward_normal);

        true
    }
}

pub struct RotateY {
    hittable: HittablePtr,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: &HittablePtr, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bounding_box = hittable.bounding_box(0.0, 1.0);

        let bbox = match bounding_box {
            Some(aabb) => aabb,
            None => Aabb::empty(),
        };

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        if bounding_box.is_some() {
            bounding_box = Some(Aabb::new(min, max));
        }

        RotateY {
            hittable: Arc::clone(hittable),
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bounding_box
    }

    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, 0.0);

        if !self.hittable.hit(&rotated_ray, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_ray, &normal);

        true
    }
}
