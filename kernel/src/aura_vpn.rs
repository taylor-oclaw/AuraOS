extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AuraVPN {
    connections: Vec<String>,
    active: bool,
}

impl AuraVPN {
    pub fn new() -> Self {
        AuraVPN {
            connections: Vec::new(),
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_connection(&mut self, connection: String) {
        if !self.connections.contains(&connection) {
            self.connections.push(connection);
        }
    }

    pub fn remove_connection(&mut self, connection: &str) {
        self.connections.retain(|c| c != connection);
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connections.clone()
    }
}

#[no_mangle]
pub extern "C" fn aura_vpn_new() -> *mut AuraVPN {
    Box::into_raw(Box::new(AuraVPN::new()))
}

#[no_mangle]
pub extern "C" fn aura_vpn_activate(vpn: *mut AuraVPN) {
    unsafe { (*vpn).activate(); }
}

#[no_mangle]
pub extern "C" fn aura_vpn_deactivate(vpn: *mut AuraVPN) {
    unsafe { (*vpn).deactivate(); }
}

#[no_mangle]
pub extern "C" fn aura_vpn_is_active(vpn: *const AuraVPN) -> bool {
    unsafe { (*vpn).is_active() }
}

#[no_mangle]
pub extern "C" fn aura_vpn_add_connection(vpn: *mut AuraVPN, connection: *const u8, len: usize) {
    let connection_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(connection, len)) };
    unsafe { (*vpn).add_connection(String::from(connection_str)); }
}

#[no_mangle]
pub extern "C" fn aura_vpn_remove_connection(vpn: *mut AuraVPN, connection: *const u8, len: usize) {
    let connection_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(connection, len)) };
    unsafe { (*vpn).remove_connection(connection_str); }
}

#[no_mangle]
pub extern "C" fn aura_vpn_list_connections(vpn: *const AuraVPN) -> Vec<String> {
    unsafe { (*vpn).list_connections() }
}

#[no_mangle]
pub extern "C" fn aura_vpn_free(vpn: *mut AuraVPN) {
    if !vpn.is_null() {
        unsafe { Box::from_raw(vpn); }
    }
}
