mod processor;
mod memory;
mod graphics;

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use processor::Cpu;
use graphics::Gpu;
use memory::Ram;

lazy_static! {
    static ref CPU: Cpu = Cpu::init();
    static ref GPU: Gpu = Gpu::init();
    static ref RAM: Ram = Ram::init();
}

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
    HwInfo {
        cpu_usage: CPU.get_usage(),
        cpu_temp: CPU.get_temperature(),
        gpu_usage: GPU.get_usage(),
        gpu_temp: GPU.get_temperature(),
        ram_usage: RAM.get_usage(),
    }
}
