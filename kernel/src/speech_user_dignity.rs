extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechUserDignity {
    user_id: u32,
    speech_content: String,
    is_public_speech: bool,
    likes_count: u32,
    comments: Vec<String>,
}

impl SpeechUserDignity {
    pub fn new(user_id: u32, speech_content: &str) -> Self {
        SpeechUserDignity {
            user_id,
            speech_content: String::from(speech_content),
            is_public_speech: false,
            likes_count: 0,
            comments: Vec::new(),
        }
    }

    pub fn set_public(&mut self, public: bool) {
        self.is_public_speech = public;
    }

    pub fn add_like(&mut self) {
        self.likes_count += 1;
    }

    pub fn add_comment(&mut self, comment: &str) {
        self.comments.push(String::from(comment));
    }

    pub fn get_likes_count(&self) -> u32 {
        self.likes_count
    }

    pub fn get_comments(&self) -> &[String] {
        &self.comments
    }
}
