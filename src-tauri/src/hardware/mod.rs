mod processor;
mod memory;
mod graphics;

use std::sync::Mutex;
use sysinfo::{System, MemoryRefreshKind, CpuRefreshKind, RefreshKind};
use nvml_wrapper::Nvml;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SYSTEM: Mutex<System> = Mutex::new(System::new_with_specifics(RefreshKind::new()
        .with_memory(MemoryRefreshKind::new().with_ram().without_swap())
        .with_cpu(CpuRefreshKind::new().with_cpu_usage().without_frequency())
        .without_processes()));
    pub static ref NVML: Mutex<Option<Nvml>> = Mutex::new(Nvml::init().ok());
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
        cpu_usage: processor::usage().round(),
        cpu_temp: processor::temperature().round(),
        gpu_usage: graphics::usage().round(),
        gpu_temp: graphics::temperature().round(),
        ram_usage: memory::usage().round(),
    }
}
