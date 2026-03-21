extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod miniapp_developer_sdk {
    use core::fmt::{Debug, Formatter};
    use alloc::boxed::Box;

    pub struct MiniApp {
        name: String,
        version: u32,
        features: Vec<String>,
        active_users: u32,
        is_active: bool,
    }

    impl MiniApp {
        pub fn new(name: &str, version: u32) -> Self {
            MiniApp {
                name: String::from(name),
                version,
                features: Vec::new(),
                active_users: 0,
                is_active: false,
            }
        }

        pub fn add_feature(&mut self, feature: &str) {
            self.features.push(String::from(feature));
        }

        pub fn activate(&mut self) {
            self.is_active = true;
        }

        pub fn deactivate(&mut self) {
            self.is_active = false;
        }

        pub fn increment_users(&mut self) {
            self.active_users += 1;
        }

        pub fn decrement_users(&mut self) {
            if self.active_users > 0 {
                self.active_users -= 1;
            }
        }

        pub fn get_info(&self) -> String {
            let mut info = format!("Name: {}, Version: {}", self.name, self.version);
            info.push_str("\nFeatures:\n");
            for feature in &self.features {
                info.push_str(format!("- {}\n", feature).as_str());
            }
            info.push_str(format!("Active Users: {}\n", self.active_users).as_str());
            info.push_str(format!("Is Active: {}", if self.is_active { "Yes" } else { "No" }).as_str());
            info
        }
    }

    impl Debug for MiniApp {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "MiniApp {{ name: {}, version: {}, features: {:?}, active_users: {}, is_active: {} }}",
                   self.name, self.version, self.features, self.active_users, self.is_active)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_miniapp_creation() {
            let app = MiniApp::new("TestApp", 1);
            assert_eq!(app.name, "TestApp");
            assert_eq!(app.version, 1);
            assert!(app.features.is_empty());
            assert_eq!(app.active_users, 0);
            assert!(!app.is_active);
        }

        #[test]
        fn test_add_feature() {
            let mut app = MiniApp::new("TestApp", 1);
            app.add_feature("Feature1");
            assert_eq!(app.features.len(), 1);
            assert_eq!(app.features[0], "Feature1");
        }

        #[test]
        fn test_activate_deactivate() {
            let mut app = MiniApp::new("TestApp", 1);
            app.activate();
            assert!(app.is_active);
            app.deactivate();
            assert!(!app.is_active);
        }

        #[test]
        fn test_user_count() {
            let mut app = MiniApp::new("TestApp", 1);
            app.increment_users();
            assert_eq!(app.active_users, 1);
            app.decrement_users();
            assert_eq!(app.active_users, 0);
        }
    }
}
