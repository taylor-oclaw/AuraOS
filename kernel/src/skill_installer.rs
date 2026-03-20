extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let installer = SkillInstaller::new();
    installer.install_skill("AI-Optimization");
    installer.install_skill("Machine-Learning");
    installer.list_installed_skills();
}

pub struct SkillInstaller {
    installed_skills: Vec<String>,
}

impl SkillInstaller {
    pub fn new() -> Self {
        SkillInstaller {
            installed_skills: Vec::new(),
        }
    }

    pub fn install_skill(&mut self, skill_name: &str) {
        if !self.is_skill_installed(skill_name) {
            self.installed_skills.push(String::from(skill_name));
            println!("Skill {} installed.", skill_name);
        } else {
            println!("Skill {} is already installed.", skill_name);
        }
    }

    pub fn uninstall_skill(&mut self, skill_name: &str) {
        if let Some(index) = self.installed_skills.iter().position(|s| s == skill_name) {
            self.installed_skills.remove(index);
            println!("Skill {} uninstalled.", skill_name);
        } else {
            println!("Skill {} not found.", skill_name);
        }
    }

    pub fn is_skill_installed(&self, skill_name: &str) -> bool {
        self.installed_skills.contains(&String::from(skill_name))
    }

    pub fn list_installed_skills(&self) {
        if self.installed_skills.is_empty() {
            println!("No skills installed.");
        } else {
            println!("Installed Skills:");
            for skill in &self.installed_skills {
                println!("{}", skill);
            }
        }
    }

    pub fn update_skill(&mut self, old_name: &str, new_name: &str) {
        if let Some(index) = self.installed_skills.iter().position(|s| s == old_name) {
            self.installed_skills[index] = String::from(new_name);
            println!("Skill {} updated to {}.", old_name, new_name);
        } else {
            println!("Skill {} not found for update.", old_name);
        }
    }
}
