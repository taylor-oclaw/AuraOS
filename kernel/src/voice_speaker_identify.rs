extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    let mut speaker = VoiceSpeakerIdentify::new();
    speaker.add_speaker("Alice");
    speaker.add_speaker("Bob");
    speaker.add_speaker("Charlie");

    if speaker.is_known_speaker("Alice") {
        speaker.identify_speaker("Alice");
    } else {
        speaker.unknown_speaker("Alice");
    }

    let speakers = speaker.get_all_speakers();
    for speaker in speakers.iter() {
        println!("Speaker: {}", speaker);
    }

    0
}

pub struct VoiceSpeakerIdentify {
    speakers: Vec<String>,
}

impl VoiceSpeakerIdentify {
    pub fn new() -> Self {
        VoiceSpeakerIdentify {
            speakers: Vec::new(),
        }
    }

    pub fn add_speaker(&mut self, name: &str) {
        self.speakers.push(String::from(name));
    }

    pub fn is_known_speaker(&self, name: &str) -> bool {
        self.speakers.contains(&String::from(name))
    }

    pub fn identify_speaker(&self, name: &str) {
        if let Some(index) = self.speakers.iter().position(|x| x == name) {
            println!("Speaker identified: {}", self.speakers[index]);
        }
    }

    pub fn unknown_speaker(&self, name: &str) {
        println!("Unknown speaker: {}", name);
    }

    pub fn get_all_speakers(&self) -> Vec<String> {
        self.speakers.clone()
    }
}
