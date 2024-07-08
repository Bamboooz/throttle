use std::time::Duration;

pub mod processor;
pub mod memory;
pub mod graphics;

pub const HW_UPDATE_INTERVAL: Duration = Duration::from_millis(1000);
