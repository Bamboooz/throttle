use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

pub struct Gpu {
    nvml: Option<Nvml>,
}

impl Gpu {
    pub fn init() -> Self {
        let nvml = Nvml::init().ok();
        
        Self { nvml }
    }

    pub fn get_usage(&self) -> f64 {
        let nvml = match &self.nvml {
            Some(nvml) => nvml,
            None => return -1.0,
        };
    
        let device = match nvml.device_by_index(0) {
            Ok(device) => device,
            Err(_) => return -1.0,
        };
    
        match device.utilization_rates() {
            Ok(utilization) => utilization.gpu as f64,
            Err(_) => -1.0,
        }
    }
    
    pub fn get_temperature(&self) -> f64 {
        let nvml = match &self.nvml {
            Some(nvml) => nvml,
            None => return -1.0,
        };
    
        let device = match nvml.device_by_index(0) {
            Ok(device) => device,
            Err(_) => return -1.0,
        };
    
        match device.temperature(TemperatureSensor::Gpu) {
            Ok(temperature) => temperature as f64,
            Err(_) => -1.0,
        }
    }
    
    pub fn deinitialize(&mut self) {
        if let Some(nvml) = self.nvml.take() {
            let _ = nvml.shutdown();
        }
    }
}
