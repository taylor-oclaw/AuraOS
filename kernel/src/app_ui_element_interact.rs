extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn app_ui_element_interact_init() {
    // Initialization logic for the module
}

pub extern "C" fn app_ui_element_interact_exit() {
    // Cleanup logic for the module
}

pub struct UIElement {
    id: u32,
    name: String,
    properties: Vec<(String, String)>,
}

impl UIElement {
    pub fn new(id: u32, name: &str) -> Self {
        UIElement {
            id,
            name: String::from(name),
            properties: Vec::new(),
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.push((String::from(key), String::from(value)));
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn remove_property(&mut self, key: &str) {
        self.properties.retain(|(k, _)| k != key);
    }

    pub fn get_all_properties(&self) -> Vec<(&String, &String)> {
        self.properties.iter().map(|(k, v)| (k, v)).collect()
    }
}

pub extern "C" fn create_ui_element(id: u32, name: *const u8, name_len: usize) -> *mut UIElement {
    let name_slice = unsafe { core::slice::from_raw_parts(name, name_len) };
    let name_str = String::from_utf8_lossy(name_slice).into_owned();
    Box::leak(Box::new(UIElement::new(id, &name_str)))
}

pub extern "C" fn add_ui_element_property(element: *mut UIElement, key: *const u8, key_len: usize, value: *const u8, value_len: usize) {
    let key_slice = unsafe { core::slice::from_raw_parts(key, key_len) };
    let key_str = String::from_utf8_lossy(key_slice).into_owned();
    let value_slice = unsafe { core::slice::from_raw_parts(value, value_len) };
    let value_str = String::from_utf8_lossy(value_slice).into_owned();

    unsafe {
        (*element).add_property(&key_str, &value_str);
    }
}

pub extern "C" fn get_ui_element_property(element: *const UIElement, key: *const u8, key_len: usize) -> *const String {
    let key_slice = unsafe { core::slice::from_raw_parts(key, key_len) };
    let key_str = String::from_utf8_lossy(key_slice).into_owned();

    match unsafe { (*element).get_property(&key_str) } {
        Some(value) => value as *const String,
        None => core::ptr::null(),
    }
}

pub extern "C" fn remove_ui_element_property(element: *mut UIElement, key: *const u8, key_len: usize) {
    let key_slice = unsafe { core::slice::from_raw_parts(key, key_len) };
    let key_str = String::from_utf8_lossy(key_slice).into_owned();

    unsafe {
        (*element).remove_property(&key_str);
    }
}

pub extern "C" fn get_all_ui_element_properties(element: *const UIElement) -> Vec<(*const String, *const String)> {
    let properties = unsafe { (*element).get_all_properties() };
    properties.into_iter().map(|(k, v)| (k as *const String, v as *const String)).collect()
}
