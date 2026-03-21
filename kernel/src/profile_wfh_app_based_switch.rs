extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut app_switcher = ProfileWFHAppBasedSwitch::new();
    app_switcher.add_app("Slack", true);
    app_switcher.add_app("Zoom", true);
    app_switcher.add_app("Chrome", false);
    app_switcher.add_app("VSCode", false);

    if app_switcher.is_wfh_app_active("Slack") {
        app_switcher.switch_to_profile("WFH");
    } else {
        app_switcher.switch_to_profile("Normal");
    }

    loop {}
}

pub struct ProfileWFHAppBasedSwitch {
    wfh_apps: Vec<String>,
    current_profile: String,
}

impl ProfileWFHAppBasedSwitch {
    pub fn new() -> Self {
        ProfileWFHAppBasedSwitch {
            wfh_apps: Vec::new(),
            current_profile: String::from("Normal"),
        }
    }

    pub fn add_app(&mut self, app_name: &str, is_wfh_app: bool) {
        if is_wfh_app {
            self.wfh_apps.push(String::from(app_name));
        }
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.wfh_apps.retain(|app| app != app_name);
    }

    pub fn is_wfh_app_active(&self, app_name: &str) -> bool {
        self.wfh_apps.contains(&String::from(app_name))
    }

    pub fn switch_to_profile(&mut self, profile_name: &str) {
        self.current_profile = String::from(profile_name);
    }

    pub fn get_current_profile(&self) -> &str {
        &self.current_profile
    }
}
