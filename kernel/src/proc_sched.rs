extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub priority: u8,
    pub cpu_time: u64,
    pub memory_kb: u64,
}

pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_pid: Option<u32>,
    pub next_pid: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_pid: None,
            next_pid: 1,
        }
    }

    pub fn spawn(&mut self, name: &str, priority: u8) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;

        self.processes.push(Process {
            pid,
            name: String::from(name),
            state: ProcessState::Ready,
            priority,
            cpu_time: 0,
            memory_kb: 0,
        });

        pid
    }

    pub fn kill(&mut self, pid: u32) {
        if let Some(process) = self.processes.iter_mut().find(|process| process.pid == pid) {
            process.state = ProcessState::Terminated;

            if self.current_pid == Some(pid) {
                self.current_pid = None;
            }
        }
    }

    pub fn block(&mut self, pid: u32) {
        if let Some(process) = self.processes.iter_mut().find(|process| process.pid == pid) {
            if process.state != ProcessState::Terminated {
                process.state = ProcessState::Blocked;
            }

            if self.current_pid == Some(pid) {
                self.current_pid = None;
            }
        }
    }

    pub fn unblock(&mut self, pid: u32) {
        if let Some(process) = self.processes.iter_mut().find(|process| process.pid == pid) {
            if process.state == ProcessState::Blocked {
                process.state = ProcessState::Ready;
            }
        }
    }

    pub fn schedule(&mut self) -> Option<u32> {
        if let Some(pid) = self.current_pid {
            if let Some(process) = self.processes.iter_mut().find(|process| process.pid == pid) {
                if process.state == ProcessState::Running {
                    process.state = ProcessState::Ready;
                }
            }

            self.current_pid = None;
        }

        let mut selected_index = None;
        let mut highest_priority = 0;

        for (index, process) in self.processes.iter().enumerate() {
            if process.state == ProcessState::Ready
                && (selected_index.is_none() || process.priority > highest_priority)
            {
                selected_index = Some(index);
                highest_priority = process.priority;
            }
        }

        if let Some(index) = selected_index {
            self.processes[index].state = ProcessState::Running;
            self.current_pid = Some(self.processes[index].pid);
        }

        self.current_pid
    }

    pub fn tick(&mut self) {
        if let Some(pid) = self.current_pid {
            if let Some(process) = self.processes.iter_mut().find(|process| process.pid == pid) {
                if process.state == ProcessState::Running {
                    process.cpu_time += 1;
                }
            }
        }
    }

    pub fn process_count(&self) -> usize {
        self.processes
            .iter()
            .filter(|process| process.state != ProcessState::Terminated)
            .count()
    }

    pub fn list_processes(&self) -> &[Process] {
        &self.processes
    }
}
