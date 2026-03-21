extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn predict_app_preconnect_api_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn predict_app_preconnect_api_exit() {
    // Cleanup logic for the module
}

pub struct PredictAppPreconnectAPI {
    app_name: String,
    preconnect_data: Vec<u8>,
    prediction_accuracy: f32,
    is_active: bool,
    last_updated: u64,
}

impl PredictAppPreconnectAPI {
    pub fn new(app_name: &str) -> Self {
        PredictAppPreconnectAPI {
            app_name: String::from(app_name),
            preconnect_data: Vec::new(),
            prediction_accuracy: 0.0,
            is_active: false,
            last_updated: 0,
        }
    }

    pub fn set_preconnect_data(&mut self, data: &[u8]) {
        self.preconnect_data.clear();
        self.preconnect_data.extend_from_slice(data);
    }

    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }

    pub fn update_prediction_accuracy(&mut self, accuracy: f32) {
        if accuracy >= 0.0 && accuracy <= 1.0 {
            self.prediction_accuracy = accuracy;
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn update_last_updated(&mut self, timestamp: u64) {
        self.last_updated = timestamp;
    }

    pub fn get_last_updated(&self) -> u64 {
        self.last_updated
    }
}

#[no_mangle]
pub extern "C" fn predict_app_preconnect_api_create(app_name: *const u8, app_name_len: usize) -> *mut PredictAppPreconnectAPI {
    let app_name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(app_name, app_name_len)) };
    Box::into_raw(Box::new(PredictAppPreconnectAPI::new(app_name_str)))
}

#[no_mangle]
pub extern "C" fn predict_app_preconnect_api_destroy(api: *mut PredictAppPreconnectAPI) {
    if !api.is_null() {
        unsafe { drop(Box::from_raw(api)); }
    }
}
