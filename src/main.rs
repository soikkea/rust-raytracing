use std::io::{self, Write};

use std::rc::Rc;
use std::time::Instant;

use rand::Rng;

use crate::color::write_color;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn ray_color(ray: &ray::Ray, world: &dyn hittable::Hittable, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return vec3::Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = hittable::HitRecord::empty();

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        if let Some(material) = &hit_record.material {
            if let Some(result) = material.scatter(ray, &hit_record) {
                return result.attenuation * ray_color(&result.scattered, world, depth - 1);
            }
        }
        return vec3::Color::origin();
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

    let material_ground: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(&vec3::Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(&vec3::Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn material::Material> = Rc::new(material::Dielectric::new(1.5));
    let material_right: Rc<dyn material::Material> =
        Rc::new(material::Metal::new(&vec3::Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        &material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
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
