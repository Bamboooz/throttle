use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Device, Nvml};

pub struct Gpu {
    pub usage: f64,
    pub temperature: f64,
}

fn usage(device: &Device) -> f64 {
    match device.utilization_rates() {
        Ok(utilization) => utilization.gpu as f64,
        Err(_) => -1.0,
    }
}

fn temperature(device: &Device) -> f64 {
    match device.temperature(TemperatureSensor::Gpu) {
        Ok(temperature) => temperature as f64,
        Err(_) => -1.0,
    }
}

pub fn gpu() -> Gpu {
    let err = Gpu {
        usage: -1.0,
        temperature: -1.0,
    };

    let nvml = Nvml::init().ok();

    let nvml = match nvml {
        Some(nvml) => nvml,
        None => return err,
    };

    let device = match nvml.device_by_index(0) {
        Ok(device) => device,
        Err(_) => return err,
    };

    let gpu = Gpu {
        usage: usage(&device),
        temperature: temperature(&device),
    };

    let _ = nvml.shutdown();

    gpu
}
