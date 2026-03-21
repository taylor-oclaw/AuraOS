extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct Stakeholder {
    name: String,
    role: String,
    importance_level: u8, // 1 to 5
}

impl Stakeholder {
    pub fn new(name: &str, role: &str, importance_level: u8) -> Self {
        Stakeholder {
            name: String::from(name),
            role: String::from(role),
            importance_level,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    pub fn get_importance_level(&self) -> u8 {
        self.importance_level
    }

    pub fn set_importance_level(&mut self, level: u8) {
        if level >= 1 && level <= 5 {
            self.importance_level = level;
        }
    }

    pub fn promote(&mut self) {
        if self.importance_level < 5 {
            self.importance_level += 1;
        }
    }
}

struct MeetingStakeholderBrief {
    stakeholders: Vec<Stakeholder>,
}

impl MeetingStakeholderBrief {
    pub fn new() -> Self {
        MeetingStakeholderBrief {
            stakeholders: Vec::new(),
        }
    }

    pub fn add_stakeholder(&mut self, stakeholder: Stakeholder) {
        self.stakeholders.push(stakeholder);
    }

    pub fn remove_stakeholder_by_name(&mut self, name: &str) {
        self.stakeholders.retain(|s| s.name != name);
    }

    pub fn get_stakeholder_by_name(&self, name: &str) -> Option<&Stakeholder> {
        self.stakeholders.iter().find(|s| s.name == name)
    }

    pub fn list_all_stakeholders(&self) -> Vec<String> {
        self.stakeholders
            .iter()
            .map(|s| String::from("info"))
            .collect()
    }
}
