use std::time::{Duration, Instant};

use crate::{
    render::ThreadedRenderer,
    scenes::{Scene, SceneConfig},
};

pub struct Gui {
    renderer: ThreadedRenderer,
    texture: Option<egui::TextureHandle>,
    render_start_time: Option<Instant>,
    render_time: Option<Duration>,
    scene: Scene,
    num_cpus: usize,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            renderer: Default::default(),
            texture: None,
            render_start_time: None,
            render_time: None,
            scene: Scene::Random,
            num_cpus: 1,
        }
    }
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            num_cpus: num_cpus::get(),
            ..Default::default()
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.renderer.is_render_in_progress() {
            ctx.request_repaint();
        } else {
            if let Some(start) = self.render_start_time.take() {
                self.render_time = Some(start.elapsed());
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                })
            })
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_enabled_ui(!self.renderer.is_render_in_progress(), |ui| {
                self.side_panel(ui);
            })
        });

        let margin = egui::style::Margin::same(0.0);

        let frame = egui::Frame {
            inner_margin: margin,
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        self.try_load_texture(ctx);

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| match self.texture.as_ref() {
                Some(texture) => {
                    ui.image(texture, texture.size_vec2());
                }
                None => {
                    ui.spinner();
                }
            });
    }
}

impl Gui {
    fn start_render(&mut self) {
        let scene = SceneConfig::get_scene(&self.scene);
        self.renderer.start_render(scene);
        self.render_start_time = Some(Instant::now());
    }

    fn try_load_texture(&mut self, ctx: &egui::Context) {
        let progress = self.renderer.check_progress();
        if progress {
            let size = self.renderer.get_image_size();
            let pixels = self.renderer.get_pixels();
            let image = egui::ColorImage::from_rgba_unmultiplied(size, pixels);
            self.texture = Some(ctx.load_texture("rendered_image", image, Default::default()));
        }
    }

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Controls");

        egui::ComboBox::from_label("Scene")
            .selected_text(format!("{:?}", self.scene))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.scene, Scene::Random, "Random");
                ui.selectable_value(&mut self.scene, Scene::TwoSpheres, "TwoSpheres");
                ui.selectable_value(&mut self.scene, Scene::TwoPerlinSpheres, "TwoPerlinSpheres");
                ui.selectable_value(&mut self.scene, Scene::Earth, "Earth");
                ui.selectable_value(&mut self.scene, Scene::SimpleLight, "SimpleLight");
                ui.selectable_value(&mut self.scene, Scene::CornellBox, "CornellBox");
                ui.selectable_value(&mut self.scene, Scene::CornellSmoke, "CornellSmoke");
                ui.selectable_value(&mut self.scene, Scene::FinalScene, "FinalScene");
            });

        ui.add(
            egui::Slider::new(&mut self.renderer.threads_to_use, 1..=self.num_cpus).text("Threads"),
        );

        if ui.button("Render").clicked() {
            self.start_render();
        }

        let [width, height] = self.renderer.get_image_size();
        ui.label(format!("Image size {} x {}", width, height));

        if self.renderer.is_render_in_progress() {
            ui.spinner();
        } else {
            if let Some(duration) = self.render_time {
                ui.label(format!("Rendering took {:?}", duration));
            }
        }
    }
}
