extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum NpuModel { Int8, Float16, Float32 }

pub struct NpuTask {
    pub id: u64,
    pub model_name: String,
    pub input_size: usize,
    pub output_size: usize,
    pub completed: bool,
}

pub struct NpuDriver {
    pub tasks: Vec<NpuTask>,
    pub next_id: u64,
    pub total_inferences: u64,
    pub available_cores: u8,
    pub active_tasks: u8,
}

impl NpuDriver {
    pub fn new(cores: u8) -> Self {
        Self { tasks: Vec::new(), next_id: 1, total_inferences: 0, available_cores: cores, active_tasks: 0 }
    }
    pub fn submit(&mut self, model: &str, input_size: usize, output_size: usize) -> Option<u64> {
        if self.active_tasks >= self.available_cores { return None; }
        let id = self.next_id; self.next_id += 1;
        self.tasks.push(NpuTask { id, model_name: String::from(model), input_size, output_size, completed: false });
        self.active_tasks += 1;
        Some(id)
    }
    pub fn complete(&mut self, id: u64) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.completed = true; self.total_inferences += 1;
            if self.active_tasks > 0 { self.active_tasks -= 1; }
        }
    }
    pub fn utilization(&self) -> f32 { self.active_tasks as f32 / self.available_cores as f32 }
    pub fn pending(&self) -> usize { self.tasks.iter().filter(|t| !t.completed).count() }
    pub fn total(&self) -> u64 { self.total_inferences }
}
