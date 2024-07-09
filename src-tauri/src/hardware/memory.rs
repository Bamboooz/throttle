use crate::hardware::SYSTEM;

pub fn usage() -> f64 {
    let mut system = SYSTEM.lock().unwrap();

    system.refresh_memory();
    
    let used_memory = system.used_memory() as f64;
    let total_memory = system.total_memory() as f64;
    
    let usage = used_memory / total_memory * 100.0;
    
    usage
}
