extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct MeetingConsentHandler {
    participants: Vec<String>,
    consent_status: Vec<bool>,
}

impl MeetingConsentHandler {
    pub fn new(participants: Vec<String>) -> Self {
        let consent_status = vec![false; participants.len()];
        MeetingConsentHandler {
            participants,
            consent_status,
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
            self.consent_status.push(false);
        }
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
            self.consent_status.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_participants(&self) -> &Vec<String> {
        &self.participants
    }

    pub fn set_consent(&mut self, participant: &str, consent: bool) -> bool {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.consent_status[index] = consent;
            true
        } else {
            false
        }
    }

    pub fn all_consent_given(&self) -> bool {
        self.consent_status.iter().all(|&status| status)
    }
}
