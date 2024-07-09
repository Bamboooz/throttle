use eframe::egui;

use crate::gui::app::components::settings;

pub fn app_header(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        egui::Frame::default()
            .inner_margin(10.0)
            .outer_margin(0.0)
            .fill(egui::Color32::from_rgba_unmultiplied(25, 25, 25, 180))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("throttle [Alt+T]");
                    
                    settings::settings_btn(ui);
                });
            });
     });
}
