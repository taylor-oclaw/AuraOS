extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_speaker_diarize_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_speaker_diarize_exit() {
    // Cleanup logic for the module
}

pub struct MeetingSpeakerDiarizer {
    speakers: Vec<String>,
    current_speaker: Option<usize>,
}

impl MeetingSpeakerDiarizer {
    pub fn new() -> Self {
        MeetingSpeakerDiarizer {
            speakers: Vec::new(),
            current_speaker: None,
        }
    }

    pub fn add_speaker(&mut self, name: String) {
        self.speakers.push(name);
    }

    pub fn remove_speaker(&mut self, index: usize) -> Option<String> {
        if index < self.speakers.len() {
            Some(self.speakers.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_speaker(&self) -> Option<&String> {
        self.current_speaker.map(|index| &self.speakers[index])
    }

    pub fn switch_to_next_speaker(&mut self) {
        if let Some(current_index) = self.current_speaker {
            self.current_speaker = Some((current_index + 1) % self.speakers.len());
        } else if !self.speakers.is_empty() {
            self.current_speaker = Some(0);
        }
    }

    pub fn get_all_speakers(&self) -> &[String] {
        &self.speakers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove_speaker() {
        let mut diarizer = MeetingSpeakerDiarizer::new();
        diarizer.add_speaker(String::from("Alice"));
        assert_eq!(diarizer.get_all_speakers().len(), 1);
        assert_eq!(diarizer.remove_speaker(0), Some(String::from("Alice")));
        assert_eq!(diarizer.get_all_speakers().len(), 0);
    }

    #[test]
    fn test_switch_to_next_speaker() {
        let mut diarizer = MeetingSpeakerDiarizer::new();
        diarizer.add_speaker(String::from("Alice"));
        diarizer.add_speaker(String::from("Bob"));
        diarizer.switch_to_next_speaker();
        assert_eq!(diarizer.get_current_speaker(), Some(&String::from("Alice")));
        diarizer.switch_to_next_speaker();
        assert_eq!(diarizer.get_current_speaker(), Some(&String::from("Bob")));
    }
}
