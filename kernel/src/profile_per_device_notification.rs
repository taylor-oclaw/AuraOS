extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProfilePerDeviceApps {
    device_id: String,
    apps: Vec<String>,
}

impl ProfilePerDeviceApps {
    pub fn new(device_id: &str) -> Self {
        ProfilePerDeviceApps {
            device_id: String::from(device_id),
            apps: Vec::new(),
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        if !self.apps.contains(&String::from(app_name)) {
            self.apps.push(String::from(app_name));
        }
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.apps.retain(|app| app != app_name);
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn has_app(&self, app_name: &str) -> bool {
        self.apps.contains(&String::from(app_name))
    }

    pub fn count_apps(&self) -> usize {
        self.apps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_per_device_apps() {
        let mut profile = ProfilePerDeviceApps::new("device123");

        assert_eq!(profile.device_id, "device123");
        assert_eq!(profile.list_apps(), Vec::<String>::new());
        assert_eq!(profile.count_apps(), 0);

        profile.add_app("app1");
        profile.add_app("app2");
        assert_eq!(profile.list_apps(), vec![String::from("app1"), String::from("app2")]);
        assert_eq!(profile.count_apps(), 2);
        assert!(profile.has_app("app1"));
        assert!(!profile.has_app("app3"));

        profile.remove_app("app1");
        assert_eq!(profile.list_apps(), vec![String::from("app2")]);
        assert_eq!(profile.count_apps(), 1);
        assert!(!profile.has_app("app1"));
    }
}
