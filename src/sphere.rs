use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable, material,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn material::Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &Arc<dyn material::Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material: Arc::clone(material),
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let p = ray.at(root);
        let outward_normal = &(p - self.center) / self.radius;
        rec.t = root;
        rec.p = p;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Option::Some(Arc::clone(&self.material));

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}
