extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct Task {
    id: u32,
    name: String,
    priority: u8,
    status: String,
}

impl Task {
    pub fn new(id: u32, name: &str, priority: u8) -> Self {
        Task {
            id,
            name: String::from(name),
            priority,
            status: String::from("Pending"),
        }
    }

    pub fn start(&mut self) {
        self.status = String::from("Running");
    }

    pub fn pause(&mut self) {
        if self.status == "Running" {
            self.status = String::from("Paused");
        }
    }

    pub fn stop(&mut self) {
        self.status = String::from("Stopped");
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }

    pub fn get_task_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|t| format!("ID: {}, Name: {}, Priority: {}, Status: {}", t.id, t.name, t.priority, t.status))
            .collect()
    }
}

#[no_mangle]
pub extern "C" fn aura_task_manager_init() -> *mut TaskManager {
    Box::into_raw(Box::new(TaskManager::new()))
}

#[no_mangle]
pub extern "C" fn aura_task_manager_add_task(task_manager: *mut TaskManager, id: u32, name: *const u8, priority: u8) {
    unsafe {
        let task_name = String::from_utf8_lossy(core::slice::from_raw_parts(name, core::str::lenient_strnlen(name as *const _, 1024)));
        (*task_manager).add_task(Task::new(id, &task_name, priority));
    }
}

#[no_mangle]
pub extern "C" fn aura_task_manager_remove_task(task_manager: *mut TaskManager, id: u32) {
    unsafe {
        (*task_manager).remove_task(id);
    }
}

#[no_mangle]
pub extern "C" fn aura_task_manager_get_task_by_id(task_manager: *const TaskManager, id: u32) -> Option<&Task> {
    unsafe {
        (*task_manager).get_task_by_id(id)
    }
}

#[no_mangle]
pub extern "C" fn aura_task_manager_list_tasks(task_manager: *const TaskManager) -> Vec<String> {
    unsafe {
        (*task_manager).list_tasks()
    }
}
