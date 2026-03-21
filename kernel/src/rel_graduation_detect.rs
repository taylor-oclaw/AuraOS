extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GraduationDetector {
    students: Vec<String>,
    courses_passed: usize,
}

impl GraduationDetector {
    pub fn new(students: Vec<String>) -> Self {
        GraduationDetector {
            students,
            courses_passed: 0,
        }
    }

    pub fn add_student(&mut self, student_name: String) {
        self.students.push(student_name);
    }

    pub fn remove_student(&mut self, student_name: &str) {
        if let Some(index) = self.students.iter().position(|s| s == student_name) {
            self.students.remove(index);
        }
    }

    pub fn record_course_passed(&mut self) {
        self.courses_passed += 1;
    }

    pub fn get_students(&self) -> &Vec<String> {
        &self.students
    }

    pub fn is_graduation_ready(&self) -> bool {
        // Assuming a student needs to pass at least 4 courses to graduate
        self.courses_passed >= 4 && !self.students.is_empty()
    }
}
