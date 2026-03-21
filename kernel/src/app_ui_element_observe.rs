extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_ui_element_observe_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_ui_element_observe_exit() {
    // Cleanup logic for the module
}

pub struct UIElementObserver {
    elements: Vec<String>,
}

impl UIElementObserver {
    pub fn new() -> Self {
        UIElementObserver {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element_name: &str) {
        self.elements.push(String::from(element_name));
    }

    pub fn remove_element(&mut self, element_name: &str) {
        if let Some(index) = self.elements.iter().position(|x| x == element_name) {
            self.elements.remove(index);
        }
    }

    pub fn get_elements(&self) -> &[String] {
        &self.elements
    }

    pub fn has_element(&self, element_name: &str) -> bool {
        self.elements.contains(&String::from(element_name))
    }

    pub fn clear_elements(&mut self) {
        self.elements.clear();
    }
}

#[no_mangle]
pub extern "C" fn app_ui_element_observe_add_element(observer_ptr: *mut UIElementObserver, element_name: *const u8, length: usize) {
    unsafe {
        if let Some(observer) = observer_ptr.as_mut() {
            let element_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(element_name, length));
            observer.add_element(element_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn app_ui_element_observe_remove_element(observer_ptr: *mut UIElementObserver, element_name: *const u8, length: usize) {
    unsafe {
        if let Some(observer) = observer_ptr.as_mut() {
            let element_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(element_name, length));
            observer.remove_element(element_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn app_ui_element_observe_has_element(observer_ptr: *const UIElementObserver, element_name: *const u8, length: usize) -> bool {
    unsafe {
        if let Some(observer) = observer_ptr.as_ref() {
            let element_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(element_name, length));
            observer.has_element(element_str)
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn app_ui_element_observe_clear_elements(observer_ptr: *mut UIElementObserver) {
    unsafe {
        if let Some(observer) = observer_ptr.as_mut() {
            observer.clear_elements();
        }
    }
}
