extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn predict_app_preauth_init() {
    // Initialization logic for the module
}

pub extern "C" fn predict_app_preauth_exit() {
    // Cleanup logic for the module
}

pub struct PredictAppPreauth {
    app_name: String,
    permissions: Vec<String>,
    is_active: bool,
}

impl PredictAppPreauth {
    pub fn new(app_name: &str) -> Self {
        PredictAppPreauth {
            app_name: String::from(app_name),
            permissions: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_permission(&mut self, permission: &str) {
        self.permissions.push(String::from(permission));
    }

    pub fn remove_permission(&mut self, permission: &str) -> bool {
        if let Some(index) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&String::from(permission))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_app_preauth() {
        let mut app = PredictAppPreauth::new("TestApp");
        assert_eq!(app.app_name, "TestApp");
        assert!(!app.is_active);
        assert!(app.permissions.is_empty());

        app.activate();
        assert!(app.is_active);

        app.add_permission("read");
        app.add_permission("write");
        assert_eq!(app.permissions.len(), 2);
        assert!(app.has_permission("read"));
        assert!(app.has_permission("write"));

        let removed = app.remove_permission("read");
        assert!(removed);
        assert!(!app.has_permission("read"));
        assert_eq!(app.permissions.len(), 1);

        app.deactivate();
        assert!(!app.is_active);
    }
}
