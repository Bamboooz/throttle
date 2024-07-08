use eframe::egui;

pub fn app_sensor_item(ui: &mut egui::Ui, label: &str, value: f64, unit: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
       
        if value != -1.0 {
            ui.label(format!("{}{}", value.round(), unit));
        } else {
            ui.label("unknown");
        }
    });
}
