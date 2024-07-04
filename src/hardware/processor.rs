use sysinfo::System;

pub fn cpu_usage(system: &mut System) -> f64 {
    system.refresh_cpu();
    
    let cpus = system.cpus();
    let cpu_count = cpus.len() as f64;
    let mut total_usage = 0.0 as f64;

    for cpu in cpus {
        total_usage += cpu.cpu_usage() as f64;
    }

    let usage_perc = total_usage / cpu_count;

    usage_perc.round()
}

#[cfg(target_os = "windows")]
pub fn cpu_temp() -> f64 {
    let a = 0.0 as f64;

    a.round()
}

#[cfg(target_os = "linux")]
pub fn cpu_temp() -> f64 {
    let temp_file_path = "/sys/class/thermal/thermal_zone0/temp";

    let mut temp_file = File::open(temp_file_path)?;

    let mut temp_str = String::new();
    temp_file.read_to_string(&mut temp_str)?;

    let temp_str_trimmed = temp_str.trim();
    let temperature: f64 = temp_str_trimmed.parse().unwrap_or(-1.0);

    let temp_celsius = temperature / 1000.0;

    Ok(temp_celsius)
}
