extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut system = MiniAppTemplateSystem::new();
    system.initialize();
    loop {
        system.run();
    }
}

struct MiniAppTemplateSystem {
    apps: Vec<String>,
    current_app_index: usize,
}

impl MiniAppTemplateSystem {
    pub fn new() -> Self {
        MiniAppTemplateSystem {
            apps: Vec::new(),
            current_app_index: 0,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|app| app == app_name) {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn switch_to_next_app(&mut self) {
        if !self.apps.is_empty() {
            self.current_app_index = (self.current_app_index + 1) % self.apps.len();
        }
    }

    pub fn run(&self) {
        if let Some(app) = self.apps.get(self.current_app_index) {
            // Simulate running the current app
        }
    }

    pub fn initialize(&mut self) {
        // Initialize system with some default apps
        self.add_app("App1");
        self.add_app("App2");
        self.add_app("App3");
    }
}
