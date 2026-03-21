extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PerfProcessPriorityAuto {
    process_id: u32,
    priority_level: u8,
    performance_data: Vec<u64>,
}

impl PerfProcessPriorityAuto {
    pub fn new(process_id: u32) -> Self {
        PerfProcessPriorityAuto {
            process_id,
            priority_level: 5, // Default priority level
            performance_data: Vec::new(),
        }
    }

    pub fn get_process_id(&self) -> u32 {
        self.process_id
    }

    pub fn set_priority_level(&mut self, level: u8) {
        if level >= 1 && level <= 10 {
            self.priority_level = level;
        }
    }

    pub fn get_priority_level(&self) -> u8 {
        self.priority_level
    }

    pub fn add_performance_data(&mut self, data: u64) {
        self.performance_data.push(data);
    }

    pub fn calculate_average_performance(&self) -> Option<u64> {
        if self.performance_data.is_empty() {
            None
        } else {
            let sum: u64 = self.performance_data.iter().sum();
            Some(sum / self.performance_data.len() as u64)
        }
    }
}
