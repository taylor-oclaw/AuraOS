extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut detector = FocusAppDetector::new();
    detector.add_app("app1");
    detector.add_app("app2");
    detector.set_focus(true);
    println!("Focus: {}", detector.is_focused());
    println!("Apps: {:?}", detector.get_apps());
    detector.remove_app("app1");
    println!("Apps after removal: {:?}", detector.get_apps());
}

pub struct FocusAppDetector {
    apps: Vec<String>,
    is_focused: bool,
}

impl FocusAppDetector {
    pub fn new() -> Self {
        FocusAppDetector {
            apps: Vec::new(),
            is_focused: false,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            self.apps.remove(index);
        }
    }

    pub fn get_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
}
