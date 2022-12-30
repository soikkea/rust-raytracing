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

    let config = render::RenderConfig::with_aspec_ratio(
        400,
        16.0 / 9.0,
        100,
        50,
        file_name,
        scenes::Scene::TwoPerlinSpheres,
    );

    if let Err(e) = render::render_and_save(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
