extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct BizTeamWorkloadView {
    team_name: String,
    workload_items: Vec<String>,
}

impl BizTeamWorkloadView {
    pub fn new(team_name: &str) -> Self {
        BizTeamWorkloadView {
            team_name: String::from(team_name),
            workload_items: Vec::new(),
        }
    }

    pub fn add_workload_item(&mut self, item: &str) {
        self.workload_items.push(String::from(item));
    }

    pub fn remove_workload_item(&mut self, index: usize) -> Option<String> {
        if index < self.workload_items.len() {
            Some(self.workload_items.remove(index))
        } else {
            None
        }
    }

    pub fn get_team_name(&self) -> &str {
        &self.team_name
    }

    pub fn get_workload_count(&self) -> usize {
        self.workload_items.len()
    }

    pub fn list_workload_items(&self) -> Vec<&str> {
        self.workload_items.iter().map(|item| item.as_str()).collect()
    }
}
