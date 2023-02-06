use crate::vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new_at_t0(origin: vec3::Point3, direction: vec3::Vec3) -> Ray {
        Ray::new(origin, direction, 0.0)
    }

    pub fn new(origin: vec3::Point3, direction: vec3::Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn empty() -> Ray {
        Ray {
            origin: vec3::Point3::origin(),
            direction: vec3::Point3::origin(),
            time: 0.0,
        }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + (t * self.direction)
    }
}
