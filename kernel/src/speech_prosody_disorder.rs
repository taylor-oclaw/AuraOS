extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut prosody = SpeechProsodyDisorder::new();
    prosody.add_disorder("Stuttering");
    prosody.add_disorder("Tourette's Syndrome");
    prosody.add_disorder("Dysarthria");

    if prosody.has_disorder("Stuttering") {
        prosody.remove_disorder("Stuttering");
    }

    let disorders = prosody.get_all_disorders();
    for disorder in disorders.iter() {
        // Process each disorder
    }

    loop {}
}

pub struct SpeechProsodyDisorder {
    disorders: Vec<String>,
}

impl SpeechProsodyDisorder {
    pub fn new() -> Self {
        SpeechProsodyDisorder {
            disorders: Vec::new(),
        }
    }

    pub fn add_disorder(&mut self, disorder: &str) {
        if !self.disorders.contains(&disorder.to_string()) {
            self.disorders.push(disorder.to_string());
        }
    }

    pub fn remove_disorder(&mut self, disorder: &str) {
        self.disorders.retain(|d| d != disorder);
    }

    pub fn has_disorder(&self, disorder: &str) -> bool {
        self.disorders.contains(&disorder.to_string())
    }

    pub fn get_all_disorders(&self) -> Vec<String> {
        self.disorders.clone()
    }
}
