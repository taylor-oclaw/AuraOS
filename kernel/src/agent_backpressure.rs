extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_backpressure_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_backpressure_exit() {
    // Cleanup logic for the module
}

pub struct AgentBackpressure {
    queue: Vec<String>,
    max_capacity: usize,
}

impl AgentBackpressure {
    pub fn new(max_capacity: usize) -> Self {
        AgentBackpressure {
            queue: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_task(&mut self, task: String) -> Result<(), &'static str> {
        if self.queue.len() >= self.max_capacity {
            Err("Queue is full")
        } else {
            self.queue.push(task);
            Ok(())
        }
    }

    pub fn remove_task(&mut self) -> Option<String> {
        self.queue.pop()
    }

    pub fn get_queue_size(&self) -> usize {
        self.queue.len()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() == self.max_capacity
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub extern "C" fn agent_backpressure_add_task(task: *const u8, task_len: usize) -> i32 {
    let module = unsafe { &mut *(0x1000 as *mut AgentBackpressure) }; // Example address
    if task.is_null() || task_len == 0 {
        return -1;
    }
    let task_str = unsafe { core::slice::from_raw_parts(task, task_len) };
    let task_string = String::from_utf8_lossy(task_str).into_owned();
    match module.add_task(task_string) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub extern "C" fn agent_backpressure_remove_task() -> *const u8 {
    let module = unsafe { &mut *(0x1000 as *mut AgentBackpressure) }; // Example address
    if let Some(task) = module.remove_task() {
        let task_bytes: Vec<u8> = task.into_bytes();
        let ptr = task_bytes.as_ptr();
        core::mem::forget(task_bytes); // Prevent deallocation
        ptr
    } else {
        core::ptr::null()
    }
}

pub extern "C" fn agent_backpressure_get_queue_size() -> usize {
    let module = unsafe { &*(0x1000 as *const AgentBackpressure) }; // Example address
    module.get_queue_size()
}

pub extern "C" fn agent_backpressure_is_full() -> bool {
    let module = unsafe { &*(0x1000 as *const AgentBackpressure) }; // Example address
    module.is_full()
}

pub extern "C" fn agent_backpressure_is_empty() -> bool {
    let module = unsafe { &*(0x1000 as *const AgentBackpressure) }; // Example address
    module.is_empty()
}
