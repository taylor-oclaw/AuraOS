extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = NonCompeteTracker::new();
    tracker.add_process("AI-Process1".into());
    tracker.add_process("AI-Process2".into());
    tracker.remove_process("AI-Process1".into());
    if tracker.is_process_active("AI-Process2".into()) {
        // Do something
    }
    let active_processes = tracker.get_active_processes();
    for process in active_processes.iter() {
        // Process each active process
    }

    loop {}
}

pub struct NonCompeteTracker {
    processes: Vec<String>,
}

impl NonCompeteTracker {
    pub fn new() -> Self {
        NonCompeteTracker {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, name: String) {
        if !self.processes.contains(&name) {
            self.processes.push(name);
        }
    }

    pub fn remove_process(&mut self, name: String) {
        self.processes.retain(|process| process != &name);
    }

    pub fn is_process_active(&self, name: String) -> bool {
        self.processes.contains(&name)
    }

    pub fn get_active_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn clear_all_processes(&mut self) {
        self.processes.clear();
    }
}
