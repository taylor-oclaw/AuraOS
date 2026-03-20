extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let ai_runtime = AiRuntimeNative::new();
    ai_runtime.initialize_system();
    loop {}
}

pub struct AiRuntimeNative {
    system_initialized: bool,
    processes: Vec<String>,
    memory_usage: usize,
    cpu_load: f32,
    disk_space: usize,
}

impl AiRuntimeNative {
    pub fn new() -> Self {
        AiRuntimeNative {
            system_initialized: false,
            processes: Vec::new(),
            memory_usage: 0,
            cpu_load: 0.0,
            disk_space: 0,
        }
    }

    pub fn initialize_system(&mut self) {
        // Simulate system initialization
        self.system_initialized = true;
        println!("System initialized!");
    }

    pub fn add_process(&mut self, process_name: &str) {
        if self.system_initialized {
            let name = String::from(process_name);
            self.processes.push(name);
            println!("Process added: {}", process_name);
        } else {
            println!("System not initialized. Cannot add process.");
        }
    }

    pub fn remove_process(&mut self, process_name: &str) {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
            println!("Process removed: {}", process_name);
        } else {
            println!("Process not found.");
        }
    }

    pub fn get_memory_usage(&self) -> usize {
        self.memory_usage
    }

    pub fn update_cpu_load(&mut self, load: f32) {
        self.cpu_load = load;
        println!("CPU Load updated to: {}", load);
    }

    pub fn check_disk_space(&self) -> usize {
        self.disk_space
    }
}
