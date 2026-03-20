extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentSandboxer {
    processes: Vec<String>,
    max_processes: usize,
}

impl AgentSandboxer {
    pub fn new(max_processes: usize) -> Self {
        AgentSandboxer {
            processes: Vec::new(),
            max_processes,
        }
    }

    pub fn add_process(&mut self, process_name: &str) -> Result<(), &'static str> {
        if self.processes.len() >= self.max_processes {
            Err("Maximum number of processes reached")
        } else {
            self.processes.push(process_name.to_string());
            Ok(())
        }
    }

    pub fn remove_process(&mut self, process_name: &str) -> Result<(), &'static str> {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
            Ok(())
        } else {
            Err("Process not found")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_sandboxer() {
        let mut sandboxer = AgentSandboxer::new(3);

        assert_eq!(sandboxer.add_process("process1"), Ok(()));
        assert_eq!(sandboxer.add_process("process2"), Ok(()));
        assert_eq!(sandboxer.add_process("process3"), Ok(()));
        assert_eq!(sandboxer.add_process("process4"), Err("Maximum number of processes reached"));

        assert_eq!(sandboxer.get_process_count(), 3);
        assert_eq!(sandboxer.is_full(), true);

        assert_eq!(sandboxer.remove_process("process2"), Ok(()));
        assert_eq!(sandboxer.list_processes(), vec![String::from("process1"), String::from("process3")]);
        assert_eq!(sandboxer.get_process_count(), 2);
        assert_eq!(sandboxer.is_full(), false);

        assert_eq!(sandboxer.remove_process("process4"), Err("Process not found"));
    }
}
