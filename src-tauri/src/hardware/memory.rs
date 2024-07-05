use sysinfo::System;

pub fn ram_usage(system: &mut System) -> f64 {
    system.refresh_memory();

    let total_memory = system.total_memory() as f64;
    let used_memory = system.used_memory() as f64;

    let usage_perc = used_memory / total_memory * 100.0;

    usage_perc.round()
}
