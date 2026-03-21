extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_participant_detect_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_participant_detect_exit() {
    // Cleanup logic for the module
}

pub struct Participant {
    name: String,
    role: String,
    present: bool,
}

impl Participant {
    pub fn new(name: &str, role: &str) -> Self {
        Participant {
            name: String::from(name),
            role: String::from(role),
            present: false,
        }
    }

    pub fn mark_present(&mut self) {
        self.present = true;
    }

    pub fn mark_absent(&mut self) {
        self.present = false;
    }

    pub fn is_present(&self) -> bool {
        self.present
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }
}

pub struct Meeting {
    participants: Vec<Participant>,
}

impl Meeting {
    pub fn new() -> Self {
        Meeting {
            participants: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn remove_participant(&mut self, name: &str) {
        self.participants.retain(|p| p.get_name() != name);
    }

    pub fn mark_all_present(&mut self) {
        for participant in &mut self.participants {
            participant.mark_present();
        }
    }

    pub fn mark_all_absent(&mut self) {
        for participant in &mut self.participants {
            participant.mark_absent();
        }
    }

    pub fn get_participant_status(&self, name: &str) -> Option<bool> {
        self.participants.iter().find(|p| p.get_name() == name).map(|p| p.is_present())
    }
}
