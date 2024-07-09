use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use crate::hardware::NVML;

pub fn usage() -> f64 {
    let mut nvml_instance = NVML.lock().unwrap();

    let nvml = match &mut *nvml_instance {
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

pub fn temperature() -> f64 {
    let mut nvml_instance = NVML.lock().unwrap();

    let nvml = match &mut *nvml_instance {
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

pub fn deinitialize() {
    let mut nvml_instance = NVML.lock().unwrap();

    if let Some(nvml) = nvml_instance.take() {
        let _ = nvml.shutdown();
    }
}
