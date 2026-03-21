extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskTeamsMessage {
    team_name: String,
    members: Vec<String>,
    tasks: Vec<String>,
}

impl AutoTaskTeamsMessage {
    pub fn new(team_name: &str) -> Self {
        AutoTaskTeamsMessage {
            team_name: String::from(team_name),
            members: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member_name: &str) {
        self.members.push(String::from(member_name));
    }

    pub fn remove_member(&mut self, member_name: &str) -> bool {
        if let Some(index) = self.members.iter().position(|m| m == member_name) {
            self.members.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_task(&mut self, task_description: &str) {
        self.tasks.push(String::from(task_description));
    }

    pub fn remove_task(&mut self, task_description: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task_description) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
