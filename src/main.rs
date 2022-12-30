use std::env;
use std::process;

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod moving_sphere;
pub mod ray;
pub mod render;
pub mod sphere;
pub mod vec3;
pub mod texture;

fn main() {
    let mut args = env::args();

    // Skip first argument
    args.next();
    let file_name = match args.next() {
        Some(arg) => arg,
        None => String::from("image.png"),
    };

    let config = render::RenderConfig {
        image_width: 1200,
        image_height: 800,
        samples_per_pixel: 500,
        max_depth: 50,
        file_name,
    };

    if let Err(e) = render::render_and_save(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
