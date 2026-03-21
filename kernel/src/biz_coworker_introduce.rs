extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
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
        println!("Hello, my name is {} and I am working in the {} department.", self.name, self.department);
    }

    pub fn change_department(&mut self, new_department: &str) {
        self.department = String::from(new_department);
        println!("Department changed to {}", self.department);
    }

    pub fn promote(&mut self, years: u8) {
        self.years_of_experience += years;
        println!("Promoted by {} years. Total experience is now {} years.", years, self.years_of_experience);
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
        println!("Added new skill: {}", skill);
    }

    pub fn display_info(&self) {
        println!("Coworker Information:");
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Department: {}", self.department);
        println!("Years of Experience: {}", self.years_of_experience);
        println!("Skills: {:?}", self.skills);
    }
}
