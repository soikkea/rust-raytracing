use crate::{ray::Ray, vec3::Point3};

#[derive(Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn empty() -> AABB {
        let point = Point3::new(0.0, 0.0, 0.0);
        AABB {
            minimum: point,
            maximum: point,
        }
    }

    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> &Point3 {
        &self.minimum
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.minimum[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(&self, other: &AABB) -> AABB {
        let small = Point3::new(
            self.min().x().min(other.min().x()),
            self.min().y().min(other.min().y()),
            self.min().z().min(other.min().z()),
        );
        let big = Point3::new(
            self.max().x().max(other.max().x()),
            self.max().y().max(other.max().y()),
            self.max().z().max(other.max().z()),
        );

        AABB::new(small, big)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    #[test]
    fn test_aabb_hit() {
        let minimum = Vec3::origin();
        let maximum = Vec3::new(1.0, 1.0, 1.0);
        let aabb = AABB::new(minimum, maximum);

        let origin = Vec3::new(0.5, -0.5, 0.5);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray::new_at_t0(origin, direction);

        assert!(!aabb.hit(&ray, 0.0, 0.4), "Too short ray hit");

        assert!(aabb.hit(&ray, 0.0, 10.0), "Ray didn't hit");
    }
}
