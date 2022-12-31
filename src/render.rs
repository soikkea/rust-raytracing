use std::{
    error::Error,
    io::{self, Write},
    sync::{
        mpsc::{channel, RecvError},
        Arc,
    },
    time::Instant,
};

use rand::Rng;

use crate::{
    bvh::BVHNode,
    color,
    hittable::{HitRecord, Hittable},
    material::ScatterResult,
    ray::{Ray},
    scenes::{Scene, SceneConfig},
    vec3::{Color},
};

pub struct RenderConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub file_name: String,
    pub scene: Scene,
}

impl RenderConfig {
    pub fn with_aspec_ratio(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        file_name: String,
        scene: Scene,
    ) -> RenderConfig {
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        RenderConfig {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            file_name,
            scene,
        }
    }

    pub fn to_scene(&self) -> SceneConfig {
        SceneConfig::get_scene(self)
    }
}

impl RenderConfig {
    pub fn aspect_ratio(&self) -> f64 {
        self.image_width as f64 / self.image_height as f64
    }
}

fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = HitRecord::empty();

    // If the ray hits nothing, return the background color.
    if !world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        return *background;
    }

    let material = hit_record
        .material
        .as_ref()
        .expect("HitRecord should contain material");
    let emitted = material.emitted(hit_record.u, hit_record.v, &hit_record.p);

    let scatter_result = material.scatter(ray, &hit_record);

    match scatter_result {
        None => emitted,
        Some(ScatterResult {
            attenuation,
            scattered,
        }) => emitted + attenuation * ray_color(&scattered, background, world, depth - 1),
    }
}

fn render(config: &RenderConfig) -> Result<image::RgbImage, RecvError> {
    // Image
    let image_width = config.image_width;
    let image_height = config.image_height;
    let samples_per_pixel = config.samples_per_pixel;
    let max_depth = config.max_depth;

    let mut image = image::RgbImage::new(image_width, image_height);

    let scene = config.to_scene();

    // World

    let world = scene.world;
    let bvh = BVHNode::new(&world.objects, 0.0, 1.0);

    // Camera

    let camera = scene.camera;

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
                let mut pixel_color = Color::origin();
                for _ in 0..samples_per_pixel {
                    let u_rand: f64 = rng.gen();
                    let v_rand: f64 = rng.gen();
                    let u: f64 = (i as f64 + u_rand) / (image_width - 1) as f64;
                    let v: f64 = (j as f64 + v_rand) / (image_height - 1) as f64;
                    let ray = thread_camera.get_ray(u, v);
                    pixel_color +=
                        ray_color(&ray, &scene.background, thread_world.as_ref(), max_depth);
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
    image.save(file_name)?;
    Ok(())
}

pub fn render_and_save(config: RenderConfig) -> Result<(), Box<dyn Error>> {
    let image = render(&config)?;

    save_image(image, &config.file_name)?;

    Ok(())
}
