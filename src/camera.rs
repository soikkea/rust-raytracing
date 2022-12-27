use crate::{ray, vec3};

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new(
        look_from: vec3::Point3,
        look_at: vec3::Point3,
        v_up: vec3::Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(&(look_from - look_at));
        let u = vec3::unit_vector(&vec3::cross(&v_up, &w));
        let v = vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let direction =
            &self.lower_left_corner + s * self.horizontal + t * self.vertical - &self.origin;
        ray::Ray::new(&self.origin, &direction)
    }
}
