extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod runtime_manager {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct RuntimeManager {
        processes: Vec<String>,
        max_processes: usize,
    }

    impl RuntimeManager {
        pub fn new(max_processes: usize) -> Self {
            RuntimeManager {
                processes: Vec::new(),
                max_processes,
            }
        }

        pub fn add_process(&mut self, process_name: &str) -> bool {
            if self.processes.len() < self.max_processes {
                self.processes.push(String::from(process_name));
                true
            } else {
                false
            }
        }

        pub fn remove_process(&mut self, process_name: &str) -> bool {
            let pos = self.processes.iter().position(|p| p == process_name);
            if let Some(index) = pos {
                self.processes.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_processes(&self) -> Vec<String> {
            self.processes.clone()
        }

        pub fn get_process_count(&self) -> usize {
            self.processes.len()
        }

        pub fn is_full(&self) -> bool {
            self.processes.len() >= self.max_processes
        }
    }
}
