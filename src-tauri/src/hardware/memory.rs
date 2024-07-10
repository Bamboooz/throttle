use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct Memory {
    pub usage: f64,
}

fn usage(system: &System) -> f64 {
    let used_memory = system.used_memory() as f64;
    let total_memory = system.total_memory() as f64;
    
    let usage = used_memory / total_memory * 100.0;
    
    usage
}

pub fn memory(system: &System) -> Memory {
    let memory_refresh_kind = MemoryRefreshKind::new()
        .with_ram()
        .without_swap();

    system.refresh_memory_specifics(memory_refresh_kind);

    Memory {
        usage: usage(system),
    }
}
