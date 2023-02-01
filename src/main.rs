use std::process;

use clap::Parser;
use cli::Cli;

mod aabb;
mod aarect;
mod box_struct;
mod bvh;
mod camera;
mod cli;
pub mod color;
pub mod constant_medium;
mod gui;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
pub mod render;
pub mod scenes;
mod sphere;
pub mod texture;
pub mod vec3;

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli { no_gui: true, .. } => run_terminal(cli),
        _ => run_gui(),
    }
}

fn run_terminal(args: Cli) {
    let file_name = args.output.expect("Should have output");
    let scene = args.scene.expect("Scene is required");

    let config = render::RenderConfig::new(file_name, scene);

    if let Err(e) = render::render_and_save(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run_gui() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Raytracer",
        native_options,
        Box::new(|cc| Box::new(gui::Gui::new(cc))),
    )
}
