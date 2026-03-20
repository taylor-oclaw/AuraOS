extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

struct AuraScreenshotMgr {
    screenshots: Vec<String>,
    max_screenshots: usize,
}

impl AuraScreenshotMgr {
    pub fn new(max_screenshots: usize) -> Self {
        AuraScreenshotMgr {
            screenshots: Vec::new(),
            max_screenshots,
        }
    }

    pub fn add_screenshot(&mut self, screenshot: String) -> Result<(), &'static str> {
        if self.screenshots.len() >= self.max_screenshots {
            Err("Maximum number of screenshots reached")
        } else {
            self.screenshots.push(screenshot);
            Ok(())
        }
    }

    pub fn get_screenshot(&self, index: usize) -> Option<&String> {
        self.screenshots.get(index)
    }

    pub fn remove_screenshot(&mut self, index: usize) -> Result<String, &'static str> {
        if let Some(screenshot) = self.screenshots.remove(index) {
            Ok(screenshot)
        } else {
            Err("Screenshot not found")
        }
    }

    pub fn list_screenshots(&self) -> Vec<&String> {
        self.screenshots.iter().collect()
    }

    pub fn clear_screenshots(&mut self) {
        self.screenshots.clear();
    }
}
