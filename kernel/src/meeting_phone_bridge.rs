extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn meeting_phone_bridge_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_phone_bridge_exit() {
    // Cleanup logic for the module
}

pub struct MeetingPhoneBridge {
    participants: Vec<String>,
    call_active: bool,
}

impl MeetingPhoneBridge {
    pub fn new() -> Self {
        MeetingPhoneBridge {
            participants: Vec::new(),
            call_active: false,
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        let index = self.participants.iter().position(|p| p == participant);
        match index {
            Some(i) => {
                self.participants.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn start_call(&mut self) {
        if !self.call_active && !self.participants.is_empty() {
            self.call_active = true;
        }
    }

    pub fn end_call(&mut self) {
        self.call_active = false;
    }

    pub fn list_participants(&self) -> Vec<String> {
        self.participants.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meeting_phone_bridge() {
        let mut bridge = MeetingPhoneBridge::new();
        assert!(!bridge.call_active);
        assert!(bridge.list_participants().is_empty());

        bridge.add_participant(String::from("Alice"));
        bridge.add_participant(String::from("Bob"));
        assert_eq!(bridge.list_participants(), vec![String::from("Alice"), String::from("Bob")]);

        assert!(bridge.remove_participant("Alice"));
        assert!(!bridge.remove_participant("Charlie"));
        assert_eq!(bridge.list_participants(), vec![String::from("Bob")]);

        bridge.start_call();
        assert!(bridge.call_active);

        bridge.end_call();
        assert!(!bridge.call_active);
    }
}
