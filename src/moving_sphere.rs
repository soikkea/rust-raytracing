use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable, material, ray,
    vec3::{Point3, Vec3},
};

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub material: Arc<dyn material::Material>,
    pub time0: f64,
    pub time1: f64,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: &Arc<dyn material::Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            radius,
            material: Arc::clone(material),
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl hittable::Hittable for MovingSphere {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
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
        let outward_normal = &(p - self.center(ray.time)) / self.radius;
        rec.t = root;
        rec.p = p;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Option::Some(Arc::clone(&self.material));

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let output_box = box0.surrounding_box(&box1);
        Some(output_box)
    }
}
