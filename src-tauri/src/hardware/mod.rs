use serde::{Serialize, Deserialize};
use sysinfo::System;
use nvml_wrapper::Nvml;

mod processor;
mod graphics;
mod memory;

#[derive(Serialize, Deserialize)]
pub struct HwInfo {
    cpu_usage: f64,
    cpu_temp: f64,
    gpu_usage: f64,
    gpu_temp: f64,
    ram_usage: f64,
}

#[tauri::command]
pub async fn hw_info() -> HwInfo {
    let mut system = System::new();

    let nvml = match Nvml::init() {
        Ok(nvml) => nvml,
        Err(_) => return HwInfo {
            cpu_usage: processor::cpu_usage(&mut system),
            cpu_temp: processor::cpu_temp(),
            gpu_usage: -1.0,
            gpu_temp: -1.0,
            ram_usage: memory::ram_usage(&mut system),
        },
    };

    let gpu_usage = match graphics::gpu_usage(&nvml) {
        Ok(v) => v,
        Err(_) => -1.0,
    };
    
    let gpu_temp = match graphics::gpu_temp(&nvml) {
        Ok(v) => v,
        Err(_) => -1.0,
    };

    HwInfo {
        cpu_usage: processor::cpu_usage(&mut system),
        cpu_temp: processor::cpu_temp(),
        gpu_usage: gpu_usage,
        gpu_temp: gpu_temp,
        ram_usage: memory::ram_usage(&mut system),
    }
}
