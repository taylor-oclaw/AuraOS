extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let exercise = SpeechTherapyExercise::new("Pronunciation Drill", 5);
    exercise.start_session();
    loop {}
}

pub struct SpeechTherapyExercise {
    name: String,
    max_attempts: usize,
    current_attempt: usize,
    exercises: Vec<String>,
}

impl SpeechTherapyExercise {
    pub fn new(name: &str, max_attempts: usize) -> Self {
        let mut exercises = Vec::new();
        exercises.push(String::from("Repeat 'hello'"));
        exercises.push(String::from("Repeat 'world'"));
        exercises.push(String::from("Repeat 'rust'"));
        exercises.push(String::from("Repeat 'kernel'"));
        exercises.push(String::from("Repeat 'module'"));

        SpeechTherapyExercise {
            name: String::from(name),
            max_attempts,
            current_attempt: 0,
            exercises,
        }
    }

    pub fn start_session(&mut self) {
        println!("Starting {} session...", self.name);
        self.current_attempt = 0;
    }

    pub fn next_exercise(&self) -> Option<&String> {
        if self.current_attempt < self.exercises.len() {
            Some(&self.exercises[self.current_attempt])
        } else {
            None
        }
    }

    pub fn complete_exercise(&mut self) {
        if self.current_attempt < self.exercises.len() {
            self.current_attempt += 1;
        }
    }

    pub fn is_session_complete(&self) -> bool {
        self.current_attempt >= self.exercises.len()
    }

    pub fn reset_session(&mut self) {
        println!("Resetting {} session...", self.name);
        self.current_attempt = 0;
    }
}
