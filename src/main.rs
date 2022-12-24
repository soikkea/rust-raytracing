use std::io::{self, Write};

use std::time::Instant;

use rand::Rng;

use crate::color::write_color;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn ray_color(ray: &ray::Ray, world: &dyn hittable::Hittable, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return vec3::Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = hittable::HitRecord::empty();

    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        let target = &hit_record.p + &hit_record.normal + vec3::random_in_unit_sphere();
        let direction = target - &hit_record.p;
        return 0.5 * ray_color(&ray::Ray::new(&hit_record.p, &direction), world, depth - 1);
    }
    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera

    let camera = camera::Camera::new();

    // Render

    let start = Instant::now();

    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT}\n255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = vec3::Color::origin();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u: f64 = (i as f64 + u_rand) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + v_rand) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&mut io::stdout().lock(), pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone.\nTime elapsed while rendering: {:?}", duration);
}
