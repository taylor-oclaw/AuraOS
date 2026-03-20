extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillBillingHook {
    user_id: u32,
    skills: Vec<String>,
    balance: f64,
}

impl SkillBillingHook {
    pub fn new(user_id: u32) -> Self {
        SkillBillingHook {
            user_id,
            skills: Vec::new(),
            balance: 0.0,
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_name: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn charge_user(&mut self, amount: f64) -> bool {
        if amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn credit_user(&mut self, amount: f64) {
        self.balance += amount;
    }
}
