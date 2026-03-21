extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_cpu_share_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_cpu_share_exit() {
    // Cleanup logic for the module
}

pub struct MeshCPUShare {
    cpu_load: Vec<u32>,
    total_tasks: u32,
}

impl MeshCPUShare {
    pub fn new(num_cpus: usize) -> Self {
        MeshCPUShare {
            cpu_load: vec![0; num_cpus],
            total_tasks: 0,
        }
    }

    pub fn add_task(&mut self, cpu_id: usize) {
        if cpu_id < self.cpu_load.len() {
            self.cpu_load[cpu_id] += 1;
            self.total_tasks += 1;
        }
    }

    pub fn remove_task(&mut self, cpu_id: usize) {
        if cpu_id < self.cpu_load.len() && self.cpu_load[cpu_id] > 0 {
            self.cpu_load[cpu_id] -= 1;
            self.total_tasks -= 1;
        }
    }

    pub fn get_cpu_load(&self, cpu_id: usize) -> Option<u32> {
        self.cpu_load.get(cpu_id).cloned()
    }

    pub fn get_total_tasks(&self) -> u32 {
        self.total_tasks
    }

    pub fn rebalance_tasks(&mut self) {
        if self.total_tasks == 0 {
            return;
        }

        let average_load = self.total_tasks / self.cpu_load.len() as u32;
        for load in &mut self.cpu_load {
            *load = average_load;
        }
    }
}
