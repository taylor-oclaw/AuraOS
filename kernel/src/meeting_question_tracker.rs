extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_question_tracker_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_question_tracker_exit() {
    // Cleanup logic for the module
}

pub struct MeetingQuestionTracker {
    questions: Vec<String>,
    answers: Vec<String>,
}

impl MeetingQuestionTracker {
    pub fn new() -> Self {
        MeetingQuestionTracker {
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: String) {
        self.questions.push(question);
    }

    pub fn add_answer(&mut self, answer: String) {
        self.answers.push(answer);
    }

    pub fn get_questions(&self) -> &Vec<String> {
        &self.questions
    }

    pub fn get_answers(&self) -> &Vec<String> {
        &self.answers
    }

    pub fn clear_all(&mut self) {
        self.questions.clear();
        self.answers.clear();
    }
}
