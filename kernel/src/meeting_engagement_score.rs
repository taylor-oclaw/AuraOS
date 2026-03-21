extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn meeting_engagement_score_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_engagement_score_exit() {
    // Cleanup logic for the module
}

pub struct MeetingEngagementScore {
    participants: Vec<String>,
    scores: Vec<u32>,
}

impl MeetingEngagementScore {
    pub fn new(participants: Vec<String>) -> Self {
        let scores = vec![0; participants.len()];
        MeetingEngagementScore { participants, scores }
    }

    pub fn add_participant(&mut self, name: String) {
        self.participants.push(name);
        self.scores.push(0);
    }

    pub fn get_score(&self, participant_index: usize) -> Option<u32> {
        self.scores.get(participant_index).cloned()
    }

    pub fn update_score(&mut self, participant_index: usize, score: u32) {
        if let Some(s) = self.scores.get_mut(participant_index) {
            *s = score;
        }
    }

    pub fn average_score(&self) -> Option<f32> {
        if self.scores.is_empty() {
            None
        } else {
            let total: u32 = self.scores.iter().sum();
            Some(total as f32 / self.scores.len() as f32)
        }
    }

    pub fn top_participant(&self) -> Option<&String> {
        if self.participants.is_empty() {
            None
        } else {
            let max_score = self.scores.iter().max()?;
            let index = self.scores.iter().position(|&s| s == *max_score)?;
            Some(&self.participants[index])
        }
    }
}
