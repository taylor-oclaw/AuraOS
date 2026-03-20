extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn cgroup_manager_init() {
    // Initialization logic for the cgroup manager
}

#[no_mangle]
pub extern "C" fn cgroup_manager_exit() {
    // Cleanup logic for the cgroup manager
}

pub struct CGroupManager {
    groups: Vec<String>,
}

impl CGroupManager {
    pub fn new() -> Self {
        CGroupManager {
            groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, name: &str) -> bool {
        if !self.groups.contains(&String::from(name)) {
            self.groups.push(String::from(name));
            true
        } else {
            false
        }
    }

    pub fn remove_group(&mut self, name: &str) -> bool {
        let pos = self.groups.iter().position(|x| x == name);
        if let Some(index) = pos {
            self.groups.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_groups(&self) -> Vec<String> {
        self.groups.clone()
    }

    pub fn group_exists(&self, name: &str) -> bool {
        self.groups.contains(&String::from(name))
    }

    pub fn clear_all_groups(&mut self) {
        self.groups.clear();
    }
}
