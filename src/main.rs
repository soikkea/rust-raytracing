use std::env;
use std::process;

pub mod aabb;
pub mod aarect;
pub mod box_struct;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod constant_medium;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod moving_sphere;
pub mod perlin;
pub mod ray;
pub mod render;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod vec3;

fn main() {
    let mut args = env::args();

    // Skip first argument
    args.next();
    let file_name = match args.next() {
        Some(arg) => arg,
        None => String::from("image.png"),
    };

    let mut config = render::RenderConfig::new(file_name, scenes::Scene::FinalScene);

    config.scene.samples_per_pixel = 10;

    if let Err(e) = render::render_and_save(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
