extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CpuHotplug {
    cpus: Vec<u32>,
}

impl CpuHotplug {
    pub fn new() -> Self {
        CpuHotplug { cpus: Vec::new() }
    }

    pub fn add_cpu(&mut self, cpu_id: u32) -> bool {
        if !self.cpus.contains(&cpu_id) {
            self.cpus.push(cpu_id);
            true
        } else {
            false
        }
    }

    pub fn remove_cpu(&mut self, cpu_id: u32) -> bool {
        let pos = self.cpus.iter().position(|&x| x == cpu_id);
        if let Some(index) = pos {
            self.cpus.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_cpus(&self) -> Vec<u32> {
        self.cpus.clone()
    }

    pub fn is_cpu_online(&self, cpu_id: u32) -> bool {
        self.cpus.contains(&cpu_id)
    }

    pub fn count_cpus(&self) -> usize {
        self.cpus.len()
    }
}
