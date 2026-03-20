extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Rust module loaded!");
    0
}

pub struct WasmSkillLoader {
    skills: Vec<String>,
}

impl WasmSkillLoader {
    pub fn new() -> Self {
        WasmSkillLoader {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn has_skill(&self, skill_name: &str) -> bool {
        self.skills.contains(&String::from(skill_name))
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}
