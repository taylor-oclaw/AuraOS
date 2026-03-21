extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let coworker = CoworkerIntroduce::new("Alice", 30, "AI Research");
    coworker.greet();
    coworker.change_department("Machine Learning");
    coworker.promote(2);
    coworker.add_skill("Natural Language Processing");
    coworker.display_info();
}

pub struct CoworkerIntroduce {
    name: String,
    age: u8,
    department: String,
    years_of_experience: u8,
    skills: Vec<String>,
}

impl CoworkerIntroduce {
    pub fn new(name: &str, age: u8, department: &str) -> Self {
        CoworkerIntroduce {
            name: String::from(name),
            age,
            department: String::from(department),
            years_of_experience: 0,
            skills: Vec::new(),
        }
    }

    pub fn greet(&self) {
    }

    pub fn change_department(&mut self, new_department: &str) {
        self.department = String::from(new_department);
    }

    pub fn promote(&mut self, years: u8) {
        self.years_of_experience += years;
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn display_info(&self) {
    }
}
