use std::process;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod render;
pub mod sphere;
pub mod vec3;

fn main() {
    let config = render::RenderConfig {
        image_width: 1200,
        image_height: 800,
        samples_per_pixel: 500,
        max_depth: 50,
        file_name: "image.png".to_string(),
    };

    if let Err(e) = render::render_and_save(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
