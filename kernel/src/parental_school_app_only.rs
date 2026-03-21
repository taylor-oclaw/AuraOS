extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let app = ParentalSchoolApp::new();
    app.initialize();
    loop {}
}

pub struct ParentalSchoolApp {
    students: Vec<Student>,
    teachers: Vec<Teacher>,
    courses: Vec<Course>,
}

impl ParentalSchoolApp {
    pub fn new() -> Self {
        ParentalSchoolApp {
            students: Vec::new(),
            teachers: Vec::new(),
            courses: Vec::new(),
        }
    }

    pub fn add_student(&mut self, name: String, age: u8) {
        let student = Student { name, age };
        self.students.push(student);
    }

    pub fn add_teacher(&mut self, name: String, subject: String) {
        let teacher = Teacher { name, subject };
        self.teachers.push(teacher);
    }

    pub fn add_course(&mut self, course_name: String, teacher_name: String) -> Result<(), &'static str> {
        if let Some(teacher) = self.teachers.iter().find(|t| t.name == teacher_name) {
            let course = Course { name: course_name, teacher };
            self.courses.push(course);
            Ok(())
        } else {
            Err("Teacher not found")
        }
    }

    pub fn list_students(&self) -> Vec<&Student> {
        self.students.iter().collect()
    }

    pub fn list_courses(&self) -> Vec<&Course> {
        self.courses.iter().collect()
    }
}

struct Student {
    name: String,
    age: u8,
}

struct Teacher {
    name: String,
    subject: String,
}

struct Course<'a> {
    name: String,
    teacher: &'a Teacher,
}
