extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GroupManager {
    groups: Vec<String>,
}

impl GroupManager {
    pub fn new() -> Self {
        GroupManager {
            groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group_name: &str) -> bool {
        if !self.groups.contains(&String::from(group_name)) {
            self.groups.push(String::from(group_name));
            true
        } else {
            false
        }
    }

    pub fn remove_group(&mut self, group_name: &str) -> bool {
        let pos = self.groups.iter().position(|g| g == group_name);
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

    pub fn group_exists(&self, group_name: &str) -> bool {
        self.groups.contains(&String::from(group_name))
    }

    pub fn number_of_groups(&self) -> usize {
        self.groups.len()
    }
}
