use crate::hardware::SYSTEM;

pub fn usage() -> f64 {
    let mut system = SYSTEM.lock().unwrap();

    system.refresh_cpu();
    
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
pub fn temperature() -> f64 {
    -1.0 // To-do
}

#[cfg(target_os = "linux")]
pub fn temperature() -> f64 {
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
