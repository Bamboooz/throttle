mod processor;
mod memory;
mod graphics;

use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref NVML: Mutex<Option<Nvml>> = Mutex::new(Nvml::init().ok());
    pub static ref SYSTEM: Mutex<System> = Mutex::new(System::new_with_specifics(RefreshKind::new()
        .with_memory(MemoryRefreshKind::new().with_ram().without_swap())
        .with_cpu(CpuRefreshKind::new().with_cpu_usage().without_frequency())
        .without_processes()));
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
    let cpu = processor::cpu();
    let gpu = graphics::gpu();
    let ram = memory::memory();

    HwInfo {
        cpu_usage: cpu.usage.round(),
        cpu_temp: cpu.temperature.round(),
        gpu_usage: gpu.usage.round(),
        gpu_temp: gpu.temperature.round(),
        ram_usage: ram.usage.round(),
    }
}
