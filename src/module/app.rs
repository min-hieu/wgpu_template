use eframe::{
    egui_wgpu,
    egui
};

use crate::module::image;

pub struct MainApp {
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().expect("WGPU not initialized");

        let image_render_resources = image::ImageRenderResources::new(wgpu_render_state);

        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(image_render_resources);

        Self {}
    }

    fn drawImage(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(800.0), egui::Sense::drag());

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            image::ImageRenderCallback {},
        ));
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.drawImage(ui);
            });
        });
    }
}
