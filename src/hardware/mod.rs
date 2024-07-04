use std::time::Duration;
use nvml_wrapper::Nvml;
use sysinfo::System;

mod processor;
mod graphics;
mod memory;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(1000);

pub struct HwInfo {
    pub cpu_usage: f64,
    pub cpu_temp: f64,
    pub gpu_usage: f64,
    pub gpu_temp: f64,
    pub ram_usage: f64,
}

pub fn hw_info(system: &mut System, nvml: &Nvml) -> HwInfo {
    let gpu_usage = match graphics::gpu_usage(nvml) {
        Ok(val) => val,
        Err(_) => -1.0,
    };

    let gpu_temp = match graphics::gpu_temp(nvml) {
        Ok(val) => val,
        Err(_) => -1.0,
    };

    HwInfo {
        cpu_usage: processor::cpu_usage(system),
        cpu_temp: processor::cpu_temp(),
        gpu_usage: gpu_usage,
        gpu_temp: gpu_temp,
        ram_usage: memory::ram_usage(system),
    }
}
