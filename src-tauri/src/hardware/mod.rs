mod processor;
mod memory;
mod graphics;

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use processor::Cpu;
use graphics::Gpu;
use memory::Ram;

lazy_static! {
    static ref CPU: Mutex<Cpu> = Mutex::new(Cpu::init());
    static ref GPU: Gpu = Gpu::init();
    static ref RAM: Mutex<Ram> = Mutex::new(Ram::init());
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
    let mut cpu = CPU.lock().unwrap();
    let mut ram = RAM.lock().unwrap();

    HwInfo {
        cpu_usage: cpu.get_usage().round(),
        cpu_temp: cpu.get_temperature().round(),
        gpu_usage: GPU.get_usage().round(),
        gpu_temp: GPU.get_temperature().round(),
        ram_usage: ram.get_usage().round(),
    }
}
