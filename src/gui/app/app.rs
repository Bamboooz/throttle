// Color palette
// Usage/temp text - rgb(111, 173, 0)
// Usage/temp unit - rgb(115, 115, 115)
// Header - rgba(25, 25, 25, 0.7)
// Header text - rgb(245, 245, 245)
// Item odd - rgb(48, 48, 48, 0.7)
// Item even - rgba(40, 40, 40, 0.7)

use eframe::egui;
use eframe::egui::{CentralPanel, Frame, Color32};

use crate::gui::app::components::header;
use crate::gui::app::components::item;
use crate::hardware::processor::Cpu;
use crate::hardware::graphics::Gpu;
use crate::hardware::memory::Ram;

pub struct Application {
    cpu: Cpu,
    gpu: Gpu,
    ram: Ram,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            cpu: Cpu::init(),
            gpu: Gpu::init(),
            ram: Ram::init(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Visible(false));
        let main_frame = Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };
        
        CentralPanel::default().frame(main_frame).show(ctx, |ui| {
            ui.vertical(|ui| {
                header::app_header(ui);
                
                item::app_sensor_item(ui, "CPU Usage", self.cpu.usage, "%");
                item::app_sensor_item(ui, "CPU Temp", self.cpu.temperature, "°C");
                item::app_sensor_item(ui, "GPU Usage", self.gpu.usage, "%");
                item::app_sensor_item(ui, "GPU Temp", self.gpu.temperature, "°C");
                item::app_sensor_item(ui, "RAM Usage", self.ram.usage, "%");
            });
        });
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.gpu.deinitialize(); // nvml requires deinitialization
    }
}
