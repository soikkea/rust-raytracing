use std::io::{self, Write};

use std::time::Instant;

use crate::color::write_color;

pub mod color;
pub mod ray;
pub mod vec3;

fn hit_sphere(center: &vec3::Point3, radius: f64, ray: &ray::Ray) -> bool {
    let oc = ray.origin() - center;
    let a = vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * vec3::dot(&oc, ray.direction());
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(ray: &ray::Ray) -> vec3::Color {
    if hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return vec3::Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = &vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal = &vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = &vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &(origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length));

    // Render

    let start = Instant::now();

    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let direction: vec3::Vec3 = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = ray::Ray::new(origin, &direction);
            let pixel_color = ray_color(&ray);
            write_color(&mut io::stdout().lock(), pixel_color).unwrap();
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone.\nTime elapsed while rendering: {:?}", duration);
}
