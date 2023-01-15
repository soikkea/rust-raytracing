use crate::{
    render::ThreadedRenderer,
    scenes::{Scene, SceneConfig},
};

pub struct Gui {
    renderer: ThreadedRenderer,
    texture: Option<egui::TextureHandle>,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            renderer: Default::default(),
            texture: None,
        }
    }
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.renderer.is_render_in_progress() {
            ctx.request_repaint();
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
            ui.heading("Controls");

            if ui.button("Render").clicked() {
                self.start_render(Scene::CornellBox);
            }

            let [width, height] = self.renderer.get_image_size();
            ui.label(format!("Image size {} x {}", width, height));

            if self.renderer.is_render_in_progress() {
                ui.spinner();
            }
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
    fn start_render(&mut self, scene: Scene) {
        let scene = SceneConfig::get_scene(scene);
        self.renderer.start_render(scene);
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
}
