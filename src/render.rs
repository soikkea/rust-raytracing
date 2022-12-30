use std::{
    error::Error,
    io::{self, Write},
    sync::{
        mpsc::{channel, RecvError},
        Arc,
    },
    time::Instant,
};

use rand::{Rng, SeedableRng};

use crate::{
    bvh::BVHNode,
    camera, color, hittable,
    hittable_list::HittableList,
    material::{self, Lambertian, Material},
    moving_sphere::MovingSphere,
    ray,
    sphere::Sphere,
    vec3::{self, Color, Point3},
};

pub struct RenderConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub file_name: String,
}

impl RenderConfig {
    pub fn with_aspec_ratio(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        file_name: String,
    ) -> RenderConfig {
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        RenderConfig {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            file_name,
        }
    }
}

fn ray_color(ray: &ray::Ray, world: &dyn hittable::Hittable, depth: u32) -> vec3::Color {
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
    let unit_direction = vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    const RANDOM_SEED: u64 = 2;

    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ))));

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
                let sphere_material: Arc<dyn material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::Color::random() * vec3::Color::random();
                    sphere_material = Arc::new(material::Lambertian::new(&albedo));
                    let center2 = center + vec3::Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        &sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(material::Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        &sphere_material,
                    ))));
                } else {
                    // glass
                    sphere_material = Arc::new(material::Dielectric::new(1.5));
                    world.add(Arc::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        &sphere_material,
                    ))));
                }
            }
        }

        let material: Arc<dyn material::Material> = Arc::new(material::Dielectric::new(1.5));
        world.add(Arc::new(Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            &material,
        ))));

        let material: Arc<dyn material::Material> =
            Arc::new(material::Lambertian::new(&vec3::Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            &material,
        ))));

        let material: Arc<dyn material::Material> =
            Arc::new(material::Metal::new(&vec3::Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            &material,
        ))));
    }

    let material_center: Arc<dyn material::Material> =
        Arc::new(material::Lambertian::new(&vec3::Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn material::Material> = Arc::new(material::Dielectric::new(1.5));
    let material_right: Arc<dyn material::Material> =
        Arc::new(material::Metal::new(&vec3::Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        &material_left,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ))));

    world
}

fn render(config: &RenderConfig) -> Result<image::RgbImage, RecvError> {
    // Image
    let image_width = config.image_width;
    let image_height = config.image_height;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = config.samples_per_pixel;
    let max_depth = config.max_depth;

    let mut image = image::RgbImage::new(image_width, image_height);

    // World

    let world = random_scene();
    let bvh = BVHNode::new(&world.objects, 0.0, 1.0);

    // Camera

    let look_from = vec3::Point3::new(13.0, 2.0, 3.0);
    let look_at = vec3::Point3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::Camera::new_with_time(
        look_from,
        look_at,
        v_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render

    let pool = threadpool::ThreadPool::new(num_cpus::get() - 1);
    let (tx, rx) = channel();

    //let world_arc = Arc::new(world);
    let world_arc = Arc::new(bvh);
    let camera_arc = Arc::new(camera);

    let start = Instant::now();

    for j in (0..image_height).rev() {
        let tx = tx.clone();
        let thread_world = Arc::clone(&world_arc);
        let thread_camera = Arc::clone(&camera_arc);
        pool.execute(move || {
            let mut rng = rand::thread_rng();
            for i in 0..image_width {
                let mut pixel_color = vec3::Color::origin();
                for _ in 0..samples_per_pixel {
                    let u_rand: f64 = rng.gen();
                    let v_rand: f64 = rng.gen();
                    let u: f64 = (i as f64 + u_rand) / (image_width - 1) as f64;
                    let v: f64 = (j as f64 + v_rand) / (image_height - 1) as f64;
                    let ray = thread_camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, thread_world.as_ref(), max_depth);
                }
                let pixel = color::color_to_pixel(pixel_color, samples_per_pixel);
                let y = image_height - 1 - j;
                tx.send((i, y, pixel)).expect("Could not send data!");
            }
        });
    }

    let mut lines_done = 0;
    for i in 0..(image_width * image_height) {
        let (x, y, pixel) = rx.recv()?;
        image.put_pixel(x, y, pixel);
        if i % image_width == 0 {
            lines_done += 1;
            let lines_remaining = image_height - lines_done;
            eprint!("\rScanlines remaining: {lines_remaining} ");
            io::stderr().flush().unwrap();
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone.\nTime elapsed while rendering: {:?}", duration);

    Ok(image)
}

fn save_image(image: image::RgbImage, file_name: &str) -> Result<(), image::ImageError> {
    let _ = image.save(file_name)?;
    Ok(())
}

pub fn render_and_save(config: RenderConfig) -> Result<(), Box<dyn Error>> {
    let image = render(&config)?;

    save_image(image, &config.file_name)?;

    Ok(())
}
