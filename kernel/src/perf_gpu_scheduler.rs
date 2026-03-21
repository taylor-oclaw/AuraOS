extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct GpuTask {
    id: u32,
    priority: u8,
    workload: String,
}

impl GpuTask {
    pub fn new(id: u32, priority: u8, workload: &str) -> Self {
        GpuTask {
            id,
            priority,
            workload: workload.into(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_priority(&self) -> u8 {
        self.priority
    }

    pub fn set_priority(&mut self, new_priority: u8) {
        self.priority = new_priority;
    }

    pub fn get_workload(&self) -> &str {
        &self.workload
    }

    pub fn update_workload(&mut self, new_workload: &str) {
        self.workload = new_workload.into();
    }
}

pub struct GpuScheduler {
    tasks: Vec<GpuTask>,
}

impl GpuScheduler {
    pub fn new() -> Self {
        GpuScheduler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: GpuTask) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) -> Option<GpuTask> {
        self.tasks.retain(|t| t.id != id);
        self.tasks.iter().find(|&t| t.id == id).cloned()
    }

    pub fn get_tasks(&self) -> &[GpuTask] {
        &self.tasks
    }

    pub fn schedule_task(&mut self, id: u32) -> Option<&GpuTask> {
        self.tasks.iter().find(|&t| t.id == id)
    }
}
