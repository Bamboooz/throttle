use hardware::UPDATE_INTERVAL;

use sysinfo::System;
use nvml_wrapper::Nvml;
use nvml_wrapper::error::NvmlError;

mod hardware;

fn main() -> Result<(), NvmlError> {
    let mut system = System::new();
    let nvml = Nvml::init()?;

    loop {
        std::thread::sleep(UPDATE_INTERVAL);
        
        let hw = hardware::hw_info(&mut system, &nvml);

        println!("cpu_usage: {}%, cpu_temp: {}C, gpu_usage: {}%, gpu_temp: {}C, ram_temp: {}%", hw.cpu_usage, hw.cpu_temp, hw.gpu_usage, hw.gpu_temp, hw.ram_usage);
    }
}
