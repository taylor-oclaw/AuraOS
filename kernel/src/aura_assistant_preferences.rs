extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraAssistantPreferences {
    user_name: String,
    language: String,
    theme: String,
    notifications_enabled: bool,
    privacy_mode: bool,
}

impl AuraAssistantPreferences {
    pub fn new(user_name: &str, language: &str, theme: &str) -> Self {
        AuraAssistantPreferences {
            user_name: String::from(user_name),
            language: String::from(language),
            theme: String::from(theme),
            notifications_enabled: true,
            privacy_mode: false,
        }
    }

    pub fn set_user_name(&mut self, user_name: &str) {
        self.user_name = String::from(user_name);
    }

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }

    pub fn toggle_notifications(&mut self) {
        self.notifications_enabled = !self.notifications_enabled;
    }

    pub fn are_notifications_enabled(&self) -> bool {
        self.notifications_enabled
    }

    pub fn set_privacy_mode(&mut self, enabled: bool) {
        self.privacy_mode = enabled;
    }

    pub fn is_privacy_mode_enabled(&self) -> bool {
        self.privacy_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences() {
        let mut prefs = AuraAssistantPreferences::new("Alice", "English", "Dark");
        assert_eq!(prefs.get_user_name(), "Alice");
        assert!(prefs.are_notifications_enabled());
        assert!(!prefs.is_privacy_mode_enabled());

        prefs.set_user_name("Bob");
        assert_eq!(prefs.get_user_name(), "Bob");

        prefs.toggle_notifications();
        assert!(!prefs.are_notifications_enabled());

        prefs.set_privacy_mode(true);
        assert!(prefs.is_privacy_mode_enabled());
    }
}
