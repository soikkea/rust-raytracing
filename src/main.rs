#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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

    if let Some(_) = cli.output.as_deref() {
        run_terminal(cli);
    } else {
        run_gui();
    }
}

fn run_terminal(args: Cli) {
    let file_name = args.output.expect("Should have output");

    let config = render::RenderConfig::new(file_name, scenes::Scene::Random);

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
