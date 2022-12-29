use crate::vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn empty() -> Ray {
        Ray {
            origin: vec3::Point3::origin(),
            direction: vec3::Point3::origin(),
        }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        &self.origin + (t * &self.direction)
    }
}
