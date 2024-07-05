use nvml_wrapper::{error::NvmlError, Nvml};
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

pub fn gpu_usage(nvml: &Nvml) -> Result<f64, NvmlError> {
    let device = nvml.device_by_index(0)?;
    let utilization = device.utilization_rates()?;

    let usage = utilization.gpu as f64;

    Ok(usage.round())
}

pub fn gpu_temp(nvml: &Nvml) -> Result<f64, NvmlError> {
    let device = nvml.device_by_index(0)?;
    let temperature = device.temperature(TemperatureSensor::Gpu)? as f64;

    Ok(temperature.round())
}
