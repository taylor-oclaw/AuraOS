extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceForegroundAction {
    action_name: String,
    parameters: Vec<String>,
    priority: u8,
    active: bool,
}

impl VoiceForegroundAction {
    pub fn new(action_name: &str, parameters: Vec<&str>, priority: u8) -> Self {
        VoiceForegroundAction {
            action_name: String::from(action_name),
            parameters: parameters.into_iter().map(String::from).collect(),
            priority,
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_action_name(&self) -> &str {
        &self.action_name
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    pub fn get_priority(&self) -> u8 {
        self.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_foreground_action() {
        let mut action = VoiceForegroundAction::new("play_music", vec!["song1", "artist2"], 5);
        assert_eq!(action.get_action_name(), "play_music");
        assert_eq!(action.get_parameters(), &vec![String::from("song1"), String::from("artist2")]);
        assert_eq!(action.get_priority(), 5);
        assert!(!action.is_active());

        action.activate();
        assert!(action.is_active());

        action.deactivate();
        assert!(!action.is_active());

        action.set_priority(3);
        assert_eq!(action.get_priority(), 3);
    }
}
