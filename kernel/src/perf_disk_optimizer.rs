extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PerfDiskOptimizer {
    disk_speed: u32,
    cache_size: u32,
    fragmentation_level: f32,
    read_operations: u64,
    write_operations: u64,
}

impl PerfDiskOptimizer {
    pub fn new(disk_speed: u32, cache_size: u32) -> Self {
        PerfDiskOptimizer {
            disk_speed,
            cache_size,
            fragmentation_level: 0.0,
            read_operations: 0,
            write_operations: 0,
        }
    }

    pub fn update_fragmentation(&mut self, level: f32) {
        if level >= 0.0 && level <= 1.0 {
            self.fragmentation_level = level;
        } else {
            panic!("Fragmentation level must be between 0.0 and 1.0");
        }
    }

    pub fn record_read_operation(&mut self) {
        self.read_operations += 1;
    }

    pub fn record_write_operation(&mut self) {
        self.write_operations += 1;
    }

    pub fn optimize_cache(&self) -> String {
        if self.cache_size > 0 {
            format!("Cache size is optimal for current workload.")
        } else {
            format!("No cache available. Consider increasing cache size.")
        }
    }

    pub fn analyze_performance(&self) -> String {
        let mut performance = String::from("Performance Analysis:\n");
        performance.push_str(&format!("Disk Speed: {} MB/s\n", self.disk_speed));
        performance.push_str(&format!("Cache Size: {} MB\n", self.cache_size));
        performance.push_str(&format!("Fragmentation Level: {:.2}%\n", self.fragmentation_level * 100.0));
        performance.push_str(&format!("Read Operations: {}\n", self.read_operations));
        performance.push_str(&format!("Write Operations: {}", self.write_operations));
        performance
    }
}
