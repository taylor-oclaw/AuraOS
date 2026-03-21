extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn priority_energy_aware_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn priority_energy_aware_exit() {
    // Cleanup logic for the module
}

pub struct PriorityEnergyAware {
    tasks: Vec<Task>,
}

impl PriorityEnergyAware {
    pub fn new() -> Self {
        PriorityEnergyAware {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, task_id: usize) -> Option<Task> {
        self.tasks.iter().position(|t| t.id == task_id).map(|index| self.tasks.remove(index))
    }

    pub fn get_task_by_id(&self, task_id: usize) -> Option<&Task> {
        self.tasks.iter().find(|&t| t.id == task_id)
    }

    pub fn update_task_priority(&mut self, task_id: usize, new_priority: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.priority = new_priority;
            true
        } else {
            false
        }
    }

    pub fn get_sorted_tasks_by_priority(&self) -> Vec<&Task> {
        let mut sorted_tasks = self.tasks.iter().collect::<Vec<_>>();
        sorted_tasks.sort_by_key(|&task| task.priority);
        sorted_tasks
    }
}

pub struct Task {
    id: usize,
    name: String,
    priority: u32,
    energy_consumption: u32,
}

impl Task {
    pub fn new(id: usize, name: String, priority: u32, energy_consumption: u32) -> Self {
        Task {
            id,
            name,
            priority,
            energy_consumption,
        }
    }
}
