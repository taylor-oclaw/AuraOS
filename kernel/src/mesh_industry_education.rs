extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_industry_education_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_industry_education_exit() {
    // Cleanup logic for the module
}

pub struct MeshIndustryEducation {
    courses: Vec<String>,
    students: Vec<String>,
    instructors: Vec<String>,
    enrollments: Vec<(String, String)>, // (student, course)
    grades: Vec<(String, String, u8)>,  // (student, course, grade)
}

impl MeshIndustryEducation {
    pub fn new() -> Self {
        MeshIndustryEducation {
            courses: Vec::new(),
            students: Vec::new(),
            instructors: Vec::new(),
            enrollments: Vec::new(),
            grades: Vec::new(),
        }
    }

    pub fn add_course(&mut self, course_name: &str) {
        self.courses.push(String::from(course_name));
    }

    pub fn add_student(&mut self, student_name: &str) {
        self.students.push(String::from(student_name));
    }

    pub fn add_instructor(&mut self, instructor_name: &str) {
        self.instructors.push(String::from(instructor_name));
    }

    pub fn enroll_student(&mut self, student_name: &str, course_name: &str) -> Result<(), String> {
        if !self.students.contains(&String::from(student_name)) {
            return Err(String::from("Student not found"));
        }
        if !self.courses.contains(&String::from(course_name)) {
            return Err(String::from("Course not found"));
        }
        self.enrollments.push((String::from(student_name), String::from(course_name)));
        Ok(())
    }

    pub fn assign_grade(&mut self, student_name: &str, course_name: &str, grade: u8) -> Result<(), String> {
        if !self.students.contains(&String::from(student_name)) {
            return Err(String::from("Student not found"));
        }
        if !self.courses.contains(&String::from(course_name)) {
            return Err(String::from("Course not found"));
        }
        self.grades.push((String::from(student_name), String::from(course_name), grade));
        Ok(())
    }

    pub fn get_grades(&self, student_name: &str) -> Vec<(String, u8)> {
        self.grades
            .iter()
            .filter(|&&(ref s, _, _)| s == student_name)
            .map(|(_, ref course, grade)| (course.clone(), *grade))
            .collect()
    }
}
