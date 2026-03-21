extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut progress = SpeechTherapyProgress::new();
    progress.add_session("Session 1", "Patient showed improvement in pronunciation.");
    progress.add_session("Session 2", "Patient struggled with vowel sounds.");
    progress.add_session("Session 3", "Patient made significant progress.");
    
    let latest_notes = progress.get_latest_notes();
    if let Some(notes) = latest_notes {
        println!("Latest Notes: {}", notes);
    }
    
    let total_sessions = progress.total_sessions();
    println!("Total Sessions: {}", total_sessions);

    loop {}
}

pub struct SpeechTherapyProgress {
    sessions: Vec<(String, String)>,
}

impl SpeechTherapyProgress {
    pub fn new() -> Self {
        SpeechTherapyProgress {
            sessions: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session_name: &str, notes: &str) {
        let name = String::from(session_name);
        let note = String::from(notes);
        self.sessions.push((name, note));
    }

    pub fn get_latest_notes(&self) -> Option<&String> {
        if let Some(last_session) = self.sessions.last() {
            Some(&last_session.1)
        } else {
            None
        }
    }

    pub fn total_sessions(&self) -> usize {
        self.sessions.len()
    }

    pub fn get_all_notes(&self) -> Vec<&String> {
        self.sessions.iter().map(|(_, note)| note).collect()
    }

    pub fn clear_progress(&mut self) {
        self.sessions.clear();
    }
}
