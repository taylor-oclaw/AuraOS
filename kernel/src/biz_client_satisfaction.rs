extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct ClientFeedback {
    client_id: u32,
    feedback_text: String,
    rating: u8, // 1 to 5 scale
    timestamp: u64, // Unix timestamp
}

impl ClientFeedback {
    pub fn new(client_id: u32, feedback_text: &str, rating: u8, timestamp: u64) -> Self {
        ClientFeedback {
            client_id,
            feedback_text: String::from(feedback_text),
            rating,
            timestamp,
        }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn get_feedback_text(&self) -> &str {
        &self.feedback_text
    }

    pub fn get_rating(&self) -> u8 {
        self.rating
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn update_feedback(&mut self, new_text: &str, new_rating: u8) {
        self.feedback_text = String::from(new_text);
        self.rating = new_rating;
    }
}

struct ClientFeedbackManager {
    feedbacks: Vec<ClientFeedback>,
}

impl ClientFeedbackManager {
    pub fn new() -> Self {
        ClientFeedbackManager {
            feedbacks: Vec::new(),
        }
    }

    pub fn add_feedback(&mut self, feedback: ClientFeedback) {
        self.feedbacks.push(feedback);
    }

    pub fn get_all_feedbacks(&self) -> &Vec<ClientFeedback> {
        &self.feedbacks
    }

    pub fn get_feedback_by_client_id(&self, client_id: u32) -> Option<&ClientFeedback> {
        self.feedbacks.iter().find(|&f| f.get_client_id() == client_id)
    }

    pub fn remove_feedback_by_client_id(&mut self, client_id: u32) {
        self.feedbacks.retain(|f| f.get_client_id() != client_id);
    }
}
