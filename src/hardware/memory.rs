use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct Ram {
    system: System,
    pub usage: f64,
}

impl Ram {
    pub fn init() -> Self {
        let refresh_kind = MemoryRefreshKind::new()
            .with_ram()
            .without_swap();
        
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_memory(refresh_kind)
                .without_cpu()
                .without_processes(),
        );
        
        Self {
            system,
            usage: 0.0,
        }
    }
    
    fn get_usage(&mut self) -> f64 {
        self.system.refresh_memory();
        
        let used_memory = self.system.used_memory() as f64;
        let total_memory = self.system.total_memory() as f64;
        
        let usage = used_memory / total_memory * 100.0;
        
        usage
    }
    
    pub fn refresh_all(&mut self) {
        self.usage = self.get_usage();
    }
}
