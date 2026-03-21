extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_skill_approve_init() {
    println!("mam_skill_approve module initialized");
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_exit() {
    println!("mam_skill_approve module exited");
}

pub struct MamSkillApprove {
    skills: Vec<String>,
    approved_skills: Vec<String>,
}

impl MamSkillApprove {
    pub fn new() -> Self {
        MamSkillApprove {
            skills: Vec::new(),
            approved_skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        if !self.skills.contains(&skill) {
            self.skills.push(skill);
        }
    }

    pub fn approve_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            let approved_skill = self.skills.remove(index);
            self.approved_skills.push(approved_skill);
            true
        } else {
            false
        }
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn list_approved_skills(&self) -> Vec<String> {
        self.approved_skills.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mam_skill_approve() {
        let mut mam = MamSkillApprove::new();
        mam.add_skill(String::from("Rust"));
        mam.add_skill(String::from("AI"));

        assert_eq!(mam.list_skills(), vec![String::from("Rust"), String::from("AI")]);
        assert_eq!(mam.approve_skill("Rust"), true);
        assert_eq!(mam.list_approved_skills(), vec![String::from("Rust")]);
        assert_eq!(mam.remove_skill("AI"), true);
        assert_eq!(mam.list_skills(), Vec::<String>::new());
    }
}
