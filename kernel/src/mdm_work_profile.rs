extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = WorkProfile::new("Alice", 30, "Engineer");
    profile.update_age(31);
    profile.add_skill(String::from("Rust Programming"));
    profile.add_skill(String::from("AI Development"));
    profile.set_department(String::from("Research and Development"));

    // Simulate some operations
    let skills = profile.get_skills();
    for skill in skills {
        println!("Skill: {}", skill);
    }

    loop {}
}

pub struct WorkProfile {
    name: String,
    age: u32,
    position: String,
    department: String,
    skills: Vec<String>,
}

impl WorkProfile {
    pub fn new(name: &str, age: u32, position: &str) -> Self {
        WorkProfile {
            name: String::from(name),
            age,
            position: String::from(position),
            department: String::from("Not Assigned"),
            skills: Vec::new(),
        }
    }

    pub fn update_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn set_department(&mut self, department: String) {
        self.department = department;
    }

    pub fn get_skills(&self) -> &Vec<String> {
        &self.skills
    }

    pub fn get_profile_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Position: {}, Department: {}",
            self.name, self.age, self.position, self.department
        )
    }
}
