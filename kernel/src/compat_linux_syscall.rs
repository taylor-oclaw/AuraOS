extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompatLinuxSyscall {
    // Example fields, replace with actual needed fields
    syscall_table: Vec<String>,
}

impl CompatLinuxSyscall {
    pub fn new() -> Self {
        CompatLinuxSyscall {
            syscall_table: Vec::new(),
        }
    }

    pub fn add_syscall(&mut self, syscall_name: &str) {
        self.syscall_table.push(String::from(syscall_name));
    }

    pub fn remove_syscall(&mut self, syscall_name: &str) -> bool {
        if let Some(index) = self.syscall_table.iter().position(|s| s == syscall_name) {
            self.syscall_table.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_syscalls(&self) -> Vec<String> {
        self.syscall_table.clone()
    }

    pub fn has_syscall(&self, syscall_name: &str) -> bool {
        self.syscall_table.contains(&String::from(syscall_name))
    }

    pub fn count_syscalls(&self) -> usize {
        self.syscall_table.len()
    }
}
