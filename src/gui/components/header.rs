use eframe::egui;
use eframe::egui::{CentralPanel, Frame, Color32};

pub fn app_header(ui: &mut egui::Ui) {
    let header_frame = Frame {
        fill: Color32::from_rgba_premultiplied(25, 25, 25, 180),
        ..Default::default()
    };
    
    CentralPanel::default()
        .frame(header_frame)
        .show_inside(ui, |ui| {
            ui.heading("throttle [Alt+T]");
        }
    );
}
