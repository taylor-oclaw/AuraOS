extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut family_hub = FamilyHubSkillShare::new();
    
    family_hub.add_skill("Rust Programming");
    family_hub.add_skill("AI Development");
    family_hub.add_skill("Kernel Module Writing");

    println!("Skills: {:?}", family_hub.list_skills());

    if family_hub.has_skill("Rust Programming") {
        println!("We have Rust Programming skill.");
    }

    let removed = family_hub.remove_skill("AI Development");
    if removed {
        println!("AI Development skill removed.");
    }

    println!("Updated Skills: {:?}", family_hub.list_skills());

    loop {}
}

pub struct FamilyHubSkillShare {
    skills: Vec<String>,
}

impl FamilyHubSkillShare {
    pub fn new() -> Self {
        FamilyHubSkillShare {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}
