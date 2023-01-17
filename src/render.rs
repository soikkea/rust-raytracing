use std::{
    error::Error,
    io::{self, Write},
    sync::{
        mpsc::{channel, Receiver, RecvError, Sender},
        Arc,
    },
    time::Instant,
    vec,
};

use image::Rgb;
use rand::Rng;

use crate::{
    bvh::BVHNode,
    color::{self, Background},
    hittable::{HitRecord, Hittable},
    material::ScatterResult,
    ray::Ray,
    scenes::{Scene, SceneConfig},
    vec3::Color,
};

pub struct RenderConfig {
    pub file_name: String,
    pub scene: SceneConfig,
}

impl RenderConfig {
    pub fn new(file_name: String, scene: Scene) -> RenderConfig {
        let scene = SceneConfig::get_scene(&scene);
        RenderConfig { file_name, scene }
    }
}

type Pixel = (u32, u32, Rgb<u8>);

pub struct ThreadedRenderer {
    image_width: usize,
    image_height: usize,
    pixels: Box<[u8]>,
    threadpool: Option<threadpool::ThreadPool>,
    sender: Sender<Pixel>,
    receiver: Receiver<Pixel>,
    pixel_counter: usize,
}

impl Default for ThreadedRenderer {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            image_width: 0,
            image_height: 0,
            pixels: Box::new([]),
            threadpool: None,
            sender,
            receiver,
            pixel_counter: 0,
        }
    }
}

impl ThreadedRenderer {
    pub fn start_render(&mut self, scene: SceneConfig) {
        if let Some(_) = self.threadpool {
            eprintln!("Render already in progress...");
            return;
        }
        self.pixel_counter = 0;
        let (width, height) = scene.image_size();
        self.init_pixels(width as _, height as _);

        let pool = threadpool::ThreadPool::new(num_cpus::get() - 1);
        render_with_threadpool(&pool, &self.sender, &scene);
        self.threadpool = Some(pool);
    }

    pub fn check_progress(&mut self) -> bool {
        if let Some(_) = &self.threadpool {
            if self.is_render_finished() {
                self.threadpool.take();
            }
        }

        let mut new_pixels: Vec<Pixel> = Vec::new();

        loop {
            match self.receiver.try_recv() {
                Ok(pixel) => new_pixels.push(pixel),
                Err(_) => {
                    for pixel in &new_pixels {
                        self.set_pixel(pixel);
                        self.pixel_counter += 1;
                    }
                    break;
                }
            }
        }

        if new_pixels.is_empty() {
            return false;
        }
        true
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn is_render_in_progress(&self) -> bool {
        self.threadpool.is_some()
    }

    pub fn get_image_size(&self) -> [usize; 2] {
        [self.image_width, self.image_height]
    }

    fn is_render_finished(&self) -> bool {
        self.pixel_counter >= self.image_height * self.image_width
    }

    fn init_pixels(&mut self, width: usize, height: usize) {
        self.image_width = width;
        self.image_height = height;
        let vec_size = width * height * 4;
        self.pixels = vec![0; vec_size].into_boxed_slice();
    }

    fn set_pixel(&mut self, pixel: &Pixel) {
        let index = pixel.0 as usize * 4 + (pixel.1 as usize * self.image_width * 4);
        for i in 0..3 {
            self.pixels[index + i] = pixel.2[i];
        }
        self.pixels[index + 3] = 0xFF;
    }
}

fn ray_color(ray: &Ray, background: &Background, world: &impl Hittable, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = HitRecord::empty();

    // If the ray hits nothing, return the background color.
    if !world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        return background.background_color(&ray.direction);
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

fn render(scene: SceneConfig) -> Result<image::RgbImage, RecvError> {
    // Image
    let (image_width, image_height) = scene.image_size();
    let samples_per_pixel = scene.samples_per_pixel;
    let max_depth = scene.max_depth;

    let mut image = image::RgbImage::new(image_width, image_height);

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

fn render_with_threadpool(pool: &threadpool::ThreadPool, tx: &Sender<Pixel>, scene: &SceneConfig) {
    let (image_width, image_height) = scene.image_size();
    let bvh = BVHNode::new(&scene.world.objects, 0.0, 1.0);
    let world_arc = Arc::new(bvh);
    let camera_arc = Arc::new(scene.camera);
    let samples_per_pixel = scene.samples_per_pixel;
    let max_depth = scene.max_depth;
    let background = scene.background;

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
                    pixel_color += ray_color(&ray, &background, thread_world.as_ref(), max_depth);
                }
                let pixel = color::color_to_pixel(pixel_color, samples_per_pixel);
                let y = image_height - 1 - j;
                tx.send((i, y, pixel)).expect("Could not send data!");
            }
        });
    }
}

fn save_image(image: image::RgbImage, file_name: &str) -> Result<(), image::ImageError> {
    image.save(file_name)?;
    Ok(())
}

pub fn render_and_save(config: RenderConfig) -> Result<(), Box<dyn Error>> {
    let image = render(config.scene)?;

    save_image(image, &config.file_name)?;

    Ok(())
}
