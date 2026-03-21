extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let coworker = CoworkerIntroduce::new("Alice", 30, "AI Developer");
    coworker.greet();
    coworker.work();
    coworker.take_break();
    coworker.learn_new_skill("Machine Learning");
    coworker.share_experience("Rust Programming");
}

pub struct CoworkerIntroduce {
    name: String,
    age: u8,
    role: String,
}

impl CoworkerIntroduce {
    pub fn new(name: &str, age: u8, role: &str) -> Self {
        CoworkerIntroduce {
            name: String::from(name),
            age,
            role: String::from(role),
        }
    }

    pub fn greet(&self) {
        let message = format!("Hello, my name is {} and I am a {}. Nice to meet you!", self.name, self.role);
        unsafe { println!("{}", message); }
    }

    pub fn work(&self) {
        let task = "Developing AI algorithms";
        let message = format!("{} is currently working on {}", self.name, task);
        unsafe { println!("{}", message); }
    }

    pub fn take_break(&self) {
        let activity = "Taking a coffee break";
        let message = format!("{} needs to {}", self.name, activity);
        unsafe { println!("{}", message); }
    }

    pub fn learn_new_skill(&mut self, skill: &str) {
        self.role.push_str(format!(", learning {}", skill).as_str());
        let message = format!("{} is now a {}", self.name, self.role);
        unsafe { println!("{}", message); }
    }

    pub fn share_experience(&self, experience: &str) {
        let message = format!("{} has valuable experience in {}", self.name, experience);
        unsafe { println!("{}", message); }
    }
}
