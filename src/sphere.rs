use crate::{hittable::Hittable, vec3};

pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
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

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = &(rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
