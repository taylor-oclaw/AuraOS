extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_continuous_auth_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_continuous_auth_exit() {
    // Cleanup logic for the module
}

pub struct SecurityContinuousAuth {
    user_id: String,
    auth_methods: Vec<String>,
    session_active: bool,
    failed_attempts: u32,
    max_failed_attempts: u32,
}

impl SecurityContinuousAuth {
    pub fn new(user_id: &str, max_failed_attempts: u32) -> Self {
        SecurityContinuousAuth {
            user_id: String::from(user_id),
            auth_methods: Vec::new(),
            session_active: false,
            failed_attempts: 0,
            max_failed_attempts,
        }
    }

    pub fn add_auth_method(&mut self, method: &str) {
        self.auth_methods.push(String::from(method));
    }

    pub fn authenticate(&mut self, method: &str) -> bool {
        if self.session_active {
            return true;
        }

        if self.auth_methods.contains(&String::from(method)) {
            self.session_active = true;
            self.failed_attempts = 0;
            true
        } else {
            self.failed_attempts += 1;
            self.failed_attempts >= self.max_failed_attempts
        }
    }

    pub fn end_session(&mut self) {
        self.session_active = false;
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn is_session_active(&self) -> bool {
        self.session_active
    }
}

#[no_mangle]
pub extern "C" fn security_continuous_auth_create(user_id: *const u8, max_failed_attempts: u32) -> *mut SecurityContinuousAuth {
    let user_id_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(user_id, 0)) };
    Box::into_raw(Box::new(SecurityContinuousAuth::new(user_id_str, max_failed_attempts)))
}

#[no_mangle]
pub extern "C" fn security_continuous_auth_destroy(auth: *mut SecurityContinuousAuth) {
    unsafe { drop(Box::from_raw(auth)); }
}
