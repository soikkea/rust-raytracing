use crate::vec3;

#[derive(Debug)]
pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: &vec3::Point3, direction: &vec3::Vec3) -> Ray {
        Ray {
            orig: vec3::Point3 { e: origin.e },
            dir: vec3::Point3 { e: direction.e },
        }
    }

    pub fn empty() -> Ray {
        Ray {
            orig: vec3::Point3::origin(),
            dir: vec3::Point3::origin(),
        }
    }

    pub fn origin(&self) -> &vec3::Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        &self.orig + (t * &self.dir)
    }
}
