use std::rc::Rc;

use crate::{hittable, material, vec3};

pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f64,
    pub material: Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f64, material: &Rc<dyn material::Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material: Rc::clone(material),
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        _rec: &mut hittable::HitRecord,
    ) -> bool {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = vec3::dot(&oc, ray.direction());
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
        _rec.t = root;
        _rec.p = p;
        _rec.set_face_normal(ray, &outward_normal);
        _rec.material = Option::Some(Rc::clone(&self.material));

        true
    }
}
