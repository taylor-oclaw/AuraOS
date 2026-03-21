extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut student = ProfileModeStudent::new(String::from("Alice"), 20, vec!["Math", "Science"]);
    student.add_course("History");
    student.remove_course("Science");
    student.update_age(21);
    student.display_info();
    loop {}
}

pub struct ProfileModeStudent {
    name: String,
    age: u8,
    courses: Vec<String>,
}

impl ProfileModeStudent {
    pub fn new(name: String, age: u8, courses: Vec<&str>) -> Self {
        let course_vec: Vec<String> = courses.into_iter().map(|s| s.to_string()).collect();
        ProfileModeStudent { name, age, courses: course_vec }
    }

    pub fn add_course(&mut self, course: &str) {
        if !self.courses.contains(&course.to_string()) {
            self.courses.push(course.to_string());
        }
    }

    pub fn remove_course(&mut self, course: &str) {
        self.courses.retain(|c| c != course);
    }

    pub fn update_age(&mut self, new_age: u8) {
        if new_age > 0 {
            self.age = new_age;
        }
    }

    pub fn display_info(&self) {
        // Simulate displaying information
        let _ = &self.name; // Placeholder for actual display logic
        let _ = &self.age; // Placeholder for actual display logic
        let _ = &self.courses; // Placeholder for actual display logic
    }
}
