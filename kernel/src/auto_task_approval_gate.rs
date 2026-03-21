extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let gate = AutoTaskApprovalGate::new();
    gate.approve_task("task1");
    gate.deny_task("task2");
    gate.list_pending_tasks();
    gate.list_approved_tasks();
    gate.list_denied_tasks();
}

pub struct AutoTaskApprovalGate {
    pending_tasks: Vec<String>,
    approved_tasks: Vec<String>,
    denied_tasks: Vec<String>,
}

impl AutoTaskApprovalGate {
    pub fn new() -> Self {
        AutoTaskApprovalGate {
            pending_tasks: Vec::new(),
            approved_tasks: Vec::new(),
            denied_tasks: Vec::new(),
        }
    }

    pub fn approve_task(&mut self, task_name: &str) {
        if let Some(index) = self.pending_tasks.iter().position(|t| t == task_name) {
            self.approved_tasks.push(self.pending_tasks.remove(index));
        }
    }

    pub fn deny_task(&mut self, task_name: &str) {
        if let Some(index) = self.pending_tasks.iter().position(|t| t == task_name) {
            self.denied_tasks.push(self.pending_tasks.remove(index));
        }
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.pending_tasks.push(task_name.to_string());
    }

    pub fn list_pending_tasks(&self) -> Vec<String> {
        self.pending_tasks.clone()
    }

    pub fn list_approved_tasks(&self) -> Vec<String> {
        self.approved_tasks.clone()
    }

    pub fn list_denied_tasks(&self) -> Vec<String> {
        self.denied_tasks.clone()
    }
}
