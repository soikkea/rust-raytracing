use std::sync::Arc;

use rand::Rng;

use crate::{
    hittable::{HitRecord, Hittable, HittablePtr},
    material::{Isotropic, MaterialPtr},
    texture::TexturePtr,
    vec3::{Color, Vec3},
};

pub struct ConstantMedium {
    boundary: HittablePtr,
    phase_function: MaterialPtr,
    negative_inverse_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: &HittablePtr, density: f64, texture: &TexturePtr) -> ConstantMedium {
        ConstantMedium {
            boundary: Arc::clone(boundary),
            phase_function: Arc::new(Isotropic::new(texture)),
            negative_inverse_density: (-1.0 / density),
        }
    }

    pub fn new_from_color(boundary: &HittablePtr, density: f64, color: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: Arc::clone(boundary),
            phase_function: Arc::new(Isotropic::new_from_color(&color)),
            negative_inverse_density: (-1.0 / density),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::Aabb> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        // Print occasional sample when debugging. To enable, set enable_debug true.
        let enable_debug: bool = false;
        let mut rng = rand::thread_rng();
        let debugging = enable_debug && (rng.gen::<f64>() < 0.00001);

        let mut rec1 = HitRecord::empty();
        let mut rec2 = HitRecord::empty();

        if !self
            .boundary
            .hit(ray, -f64::INFINITY, f64::INFINITY, &mut rec1)
        {
            return false;
        }

        if !self
            .boundary
            .hit(ray, rec1.t + 0.0001, f64::INFINITY, &mut rec2)
        {
            return false;
        }

        if debugging {
            eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {hit_distance}");
            eprintln!("rec.t        = {}", rec.t);
            eprintln!("rec.p        = {}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.material = Some(Arc::clone(&self.phase_function));

        true
    }
}
