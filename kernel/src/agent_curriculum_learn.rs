extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut curriculum = Curriculum::new();
    curriculum.add_course("AI Fundamentals");
    curriculum.add_course("Machine Learning");
    curriculum.add_course("Deep Learning");
    curriculum.add_course("Neural Networks");
    curriculum.add_course("Computer Vision");

}

pub struct Curriculum {
    courses: Vec<String>,
}

impl Curriculum {
    pub fn new() -> Self {
        Curriculum { courses: Vec::new() }
    }

    pub fn add_course(&mut self, course_name: &str) {
        self.courses.push(String::from(course_name));
    }

    pub fn remove_course(&mut self, course_name: &str) {
        if let Some(index) = self.courses.iter().position(|c| c == course_name) {
            self.courses.remove(index);
        }
    }

    pub fn get_courses(&self) -> Vec<String> {
        self.courses.clone()
    }

    pub fn has_course(&self, course_name: &str) -> bool {
        self.courses.contains(&String::from(course_name))
    }

    pub fn count_courses(&self) -> usize {
        self.courses.len()
    }
}
