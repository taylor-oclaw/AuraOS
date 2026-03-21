extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Rust module loaded!");
    0
}

pub struct SpeechDeafSpeechAdapt {
    // Example fields, replace with actual logic
    enabled: bool,
    language: String,
    keywords: Vec<String>,
    sensitivity: u8,
    log_enabled: bool,
}

impl SpeechDeafSpeechAdapt {
    pub fn new() -> Self {
        SpeechDeafSpeechAdapt {
            enabled: true,
            language: String::from("en"),
            keywords: Vec::new(),
            sensitivity: 50, // Example value
            log_enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        if self.log_enabled {
            println!("SpeechDeafSpeechAdapt enabled.");
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        if self.log_enabled {
            println!("SpeechDeafSpeechAdapt disabled.");
        }
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
        if self.log_enabled {
            println!("Language set to {}", self.language);
        }
    }

    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.push(keyword);
        if self.log_enabled {
            println!("Keyword added: {}", keyword);
        }
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        let pos = self.keywords.iter().position(|k| k == keyword);
        if let Some(index) = pos {
            self.keywords.remove(index);
            if self.log_enabled {
                println!("Keyword removed: {}", keyword);
            }
            true
        } else {
            false
        }
    }

    pub fn set_sensitivity(&mut self, sensitivity: u8) {
        self.sensitivity = sensitivity;
        if self.log_enabled {
            println!("Sensitivity set to {}", self.sensitivity);
        }
    }

    pub fn toggle_logging(&mut self) {
        self.log_enabled = !self.log_enabled;
        if self.log_enabled {
            println!("Logging enabled.");
        } else {
            println!("Logging disabled.");
        }
    }
}
