extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut detector = ToneEmpathyDetectNeed::new();
    detector.train("user1", "I am feeling sad.");
    detector.train("user2", "I am happy today!");
    detector.train("user3", "I need help with my project.");

    if detector.detect_need("Can you assist me?") {
        println!("Help needed detected!");
    } else {
        println!("No help needed detected.");
    }

    loop {}
}

pub struct ToneEmpathyDetectNeed {
    user_data: Vec<(String, String)>,
}

impl ToneEmpathyDetectNeed {
    pub fn new() -> Self {
        ToneEmpathyDetectNeed {
            user_data: Vec::new(),
        }
    }

    pub fn train(&mut self, user_id: &str, text: &str) {
        self.user_data.push((String::from(user_id), String::from(text)));
    }

    pub fn detect_need(&self, input_text: &str) -> bool {
        // Simple heuristic to detect need for help
        let keywords = ["help", "assist", "need"];
        for keyword in keywords.iter() {
            if input_text.contains(keyword) {
                return true;
            }
        }
        false
    }

    pub fn get_user_data(&self, user_id: &str) -> Option<&String> {
        self.user_data.iter()
            .find(|&&(ref id, _)| id == user_id)
            .map(|(_, text)| text)
    }

    pub fn list_users(&self) -> Vec<String> {
        self.user_data.iter()
            .map(|(id, _)| id.clone())
            .collect()
    }
}
