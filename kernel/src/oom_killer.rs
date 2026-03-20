extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OomKiller {
    processes: Vec<Process>,
}

impl OomKiller {
    pub fn new() -> Self {
        OomKiller {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, pid: u32, name: String, memory_usage: usize) {
        let process = Process { pid, name, memory_usage };
        self.processes.push(process);
    }

    pub fn remove_process(&mut self, pid: u32) -> Option<Process> {
        self.processes.retain(|p| p.pid != pid);
        self.processes.iter().find(|&p| p.pid == pid).cloned()
    }

    pub fn find_process_by_pid(&self, pid: u32) -> Option<&Process> {
        self.processes.iter().find(|&p| p.pid == pid)
    }

    pub fn find_processes_by_name(&self, name: &str) -> Vec<&Process> {
        self.processes.iter().filter(|&p| p.name == name).collect()
    }

    pub fn get_oom_candidate(&mut self) -> Option<Process> {
        if let Some((index, _)) = self.processes.iter().enumerate().min_by_key(|&(_, p)| p.memory_usage) {
            Some(self.processes.remove(index))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Process {
    pid: u32,
    name: String,
    memory_usage: usize,
}
