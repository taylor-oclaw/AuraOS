extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeDotNet {
    processes: Vec<String>,
}

impl RuntimeDotNet {
    pub fn new() -> Self {
        RuntimeDotNet {
            processes: Vec::new(),
        }
    }

    pub fn start_process(&mut self, process_name: &str) {
        let name = String::from(process_name);
        if !self.processes.contains(&name) {
            self.processes.push(name);
            println!("Started process: {}", process_name);
        } else {
            println!("Process already running: {}", process_name);
        }
    }

    pub fn stop_process(&mut self, process_name: &str) {
        let name = String::from(process_name);
        if let Some(index) = self.processes.iter().position(|x| *x == name) {
            self.processes.remove(index);
            println!("Stopped process: {}", process_name);
        } else {
            println!("Process not found: {}", process_name);
        }
    }

    pub fn list_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn is_process_running(&self, process_name: &str) -> bool {
        self.processes.contains(process_name)
    }

    pub fn restart_process(&mut self, process_name: &str) {
        if self.is_process_running(process_name) {
            self.stop_process(process_name);
            self.start_process(process_name);
        } else {
            println!("Process not found to restart: {}", process_name);
        }
    }
}
