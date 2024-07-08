// Color palette
// Usage/temp text - rgb(111, 173, 0)
// Usage/temp unit - rgb(115, 115, 115)
// Header - rgba(25, 25, 25, 0.7)
// Header text - rgb(245, 245, 245)
// Item odd - rgb(48, 48, 48, 0.7)
// Item even - rgba(40, 40, 40, 0.7)

use std::time::Instant;
use eframe::egui;
use eframe::egui::CentralPanel;

use crate::gui::components::header;
use crate::gui::components::item;
use crate::hardware::processor::Cpu;
use crate::hardware::graphics::Gpu;
use crate::hardware::memory::Ram;
use crate::hardware::HW_UPDATE_INTERVAL;

pub struct Application {
    cpu: Cpu,
    gpu: Gpu,
    ram: Ram,
    last_update_time: Instant,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            ram: Ram::new(),
            last_update_time: Instant::now(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Visible(false));
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update_time);
        
        if elapsed >= HW_UPDATE_INTERVAL {
            self.cpu.refresh_all();
            self.gpu.refresh_all();
            self.ram.refresh_all();
            
            self.last_update_time = now;
        }
        
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                header::app_header(ui);
                
                item::app_sensor_item(ui, "CPU Usage", self.cpu.usage, "%");
                item::app_sensor_item(ui, "CPU Temp", self.cpu.temperature, "°C");
                item::app_sensor_item(ui, "GPU Usage", self.gpu.usage, "%");
                item::app_sensor_item(ui, "GPU Temp", self.gpu.temperature, "°C");
                item::app_sensor_item(ui, "RAM Usage", self.ram.usage, "%");
            });
        });
        
        ctx.request_repaint_after(HW_UPDATE_INTERVAL);
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.gpu.deinitialize(); // nvml requires deinitialization
    }
}
