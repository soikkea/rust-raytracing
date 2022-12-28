use std::io::{self, Write};

use std::rc::Rc;
use std::time::Instant;

use rand::{Rng, SeedableRng};

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

fn random_scene() -> hittable_list::HittableList {
    const RANDOM_SEED: u64 = 2;

    let mut world = hittable_list::HittableList::new();

    let ground_material: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(&vec3::Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    let mut rng = rand_pcg::Pcg32::seed_from_u64(RANDOM_SEED);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = vec3::Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - vec3::Point3::new(4.0, 0.2, 0.00)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::Color::random() * vec3::Color::random();
                    sphere_material = Rc::new(material::Lambertian::new(&albedo));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, &sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(material::Metal::new(&albedo, fuzz));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, &sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(material::Dielectric::new(1.5));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, &sphere_material)));
                }
            }
        }

        let material: Rc<dyn material::Material> = Rc::new(material::Dielectric::new(1.5));
        world.add(Box::new(sphere::Sphere::new(
            vec3::Point3::new(0.0, 1.0, 0.0),
            1.0,
            &material,
        )));

        let material: Rc<dyn material::Material> =
            Rc::new(material::Lambertian::new(&vec3::Color::new(0.4, 0.2, 0.1)));
        world.add(Box::new(sphere::Sphere::new(
            vec3::Point3::new(-4.0, 1.0, 0.0),
            1.0,
            &material,
        )));

        let material: Rc<dyn material::Material> =
            Rc::new(material::Metal::new(&vec3::Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Box::new(sphere::Sphere::new(
            vec3::Point3::new(4.0, 1.0, 0.0),
            1.0,
            &material,
        )));
    }

    let material_center: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(&vec3::Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn material::Material> = Rc::new(material::Dielectric::new(1.5));
    let material_right: Rc<dyn material::Material> =
        Rc::new(material::Metal::new(&vec3::Color::new(0.8, 0.6, 0.2), 0.0));

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
        -0.45,
        &material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 400; // 1200
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50; // 500
    const MAX_DEPTH: i32 = 50;

    let mut image = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // World

    let world = random_scene();

    // Camera

    let look_from = vec3::Point3::new(13.0, 2.0, 3.0);
    let look_at = vec3::Point3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    let start = Instant::now();

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
            // write_color(&mut io::stdout().lock(), pixel_color, SAMPLES_PER_PIXEL).unwrap();
            let pixel = color::color_to_pixel(pixel_color, SAMPLES_PER_PIXEL);
            let y = IMAGE_HEIGHT - 1 - j;
            image.put_pixel(i, y, pixel);
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone.\nTime elapsed while rendering: {:?}", duration);
    let _ = image
        .save("image.png")
        .expect("Should have been able to save");
}
