use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    pub usage: f64,
    pub temperature: f64,
}

fn usage(system: &System) -> f64 {
    let cpus = system.cpus();
    let cpu_count = cpus.len() as f64;
    let mut total_usage = 0.0;
    
    for cpu in cpus {
        total_usage += cpu.cpu_usage() as f64;
    }
    
    let avg_usage = total_usage / cpu_count;
    
    avg_usage
}

#[cfg(target_os = "windows")]
fn temperature() -> f64 {
    -1.0 // To-do
}

#[cfg(target_os = "linux")]
fn temperature() -> f64 {
    use std::fs;
    
    let temp_file_path = "/sys/class/thermal/thermal_zone0/temp";

    let temp_str = match fs::read_to_string(temp_file_path) {
        Ok(temp_str) => temp_str,
        Err(_) => String::new(),
    };
    
    match temp_str.trim().parse::<i32>() {
        Ok(temp) => temp as f64 / 1000.0,
        Err(_) => -1.0,
    }
}

pub fn cpu() -> Cpu {
    let cpu_refresh_kind = CpuRefreshKind::new()
        .with_cpu_usage()
        .without_frequency();

    let refresh_kind: RefreshKind = RefreshKind::new()
        .with_cpu(cpu_refresh_kind)
        .without_memory()
        .without_processes();

    let mut system = System::new_with_specifics(refresh_kind);

    system.refresh_cpu_specifics(cpu_refresh_kind);

    Cpu {
        usage: usage(&system),
        temperature: temperature(),
    }
}
