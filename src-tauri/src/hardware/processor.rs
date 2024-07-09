use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    system: System,
    pub usage: f64,
    pub temperature: f64,
}

impl Cpu {
    pub fn init() -> Self {
        let refresh_kind = CpuRefreshKind::new()
            .with_cpu_usage()
            .without_frequency();
        
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(refresh_kind)
                .without_memory()
                .without_processes(),
        );
        
        Self {
            system,
            usage: 0.0,
            temperature: 0.0,
        }
    }
    
    fn get_usage(&mut self) -> f64 {
        self.system.refresh_cpu();
        
        let cpus = self.system.cpus();
        let cpu_count = cpus.len() as f64;
        let mut total_usage = 0.0;
        
        for cpu in cpus {
            total_usage += cpu.cpu_usage() as f64;
        }
        
        let avg_usage = total_usage / cpu_count;
        
        avg_usage
    }
    
    #[cfg(target_os = "windows")]
    fn get_temperature(&mut self) -> f64 {
        -1.0 // To-do
    }
    
    #[cfg(target_os = "linux")]
    fn get_temperature(&mut self) -> f64 {
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
    
    pub fn refresh_all(&mut self) {
        self.usage = self.get_usage();
        self.temperature = self.get_temperature();
    }
}
