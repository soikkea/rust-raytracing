use crate::{ray, vec3};

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vec3::Point3::new(0.0, 0.0, 0.0);
        let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            &origin - &horizontal / 2.0 - &vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        let direction =
            &self.lower_left_corner + u * self.horizontal + v * self.vertical - &self.origin;
        ray::Ray::new(&self.origin, &direction)
    }
}
