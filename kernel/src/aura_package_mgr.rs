extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

struct AuraPackageMgr {
    packages: Vec<String>,
}

impl AuraPackageMgr {
    pub fn new() -> Self {
        AuraPackageMgr {
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package_name: &str) {
        self.packages.push(String::from(package_name));
    }

    pub fn remove_package(&mut self, package_name: &str) {
        if let Some(index) = self.packages.iter().position(|p| p == package_name) {
            self.packages.remove(index);
        }
    }

    pub fn list_packages(&self) -> Vec<String> {
        self.packages.clone()
    }

    pub fn has_package(&self, package_name: &str) -> bool {
        self.packages.contains(&String::from(package_name))
    }

    pub fn count_packages(&self) -> usize {
        self.packages.len()
    }
}

pub extern "C" fn aura_package_mgr_new() -> *mut AuraPackageMgr {
    Box::into_raw(Box::new(AuraPackageMgr::new()))
}

pub extern "C" fn aura_package_mgr_add_package(ptr: *mut AuraPackageMgr, package_name: *const u8, len: usize) {
    unsafe {
        if let Some(package_mgr) = ptr.as_mut() {
            let name = core::str::from_utf8_unchecked(core::slice::from_raw_parts(package_name, len));
            package_mgr.add_package(name);
        }
    }
}

pub extern "C" fn aura_package_mgr_remove_package(ptr: *mut AuraPackageMgr, package_name: *const u8, len: usize) {
    unsafe {
        if let Some(package_mgr) = ptr.as_mut() {
            let name = core::str::from_utf8_unchecked(core::slice::from_raw_parts(package_name, len));
            package_mgr.remove_package(name);
        }
    }
}

pub extern "C" fn aura_package_mgr_list_packages(ptr: *mut AuraPackageMgr) -> Vec<String> {
    unsafe {
        if let Some(package_mgr) = ptr.as_ref() {
            package_mgr.list_packages()
        } else {
            Vec::new()
        }
    }
}

pub extern "C" fn aura_package_mgr_has_package(ptr: *mut AuraPackageMgr, package_name: *const u8, len: usize) -> bool {
    unsafe {
        if let Some(package_mgr) = ptr.as_ref() {
            let name = core::str::from_utf8_unchecked(core::slice::from_raw_parts(package_name, len));
            package_mgr.has_package(name)
        } else {
            false
        }
    }
}

pub extern "C" fn aura_package_mgr_count_packages(ptr: *mut AuraPackageMgr) -> usize {
    unsafe {
        if let Some(package_mgr) = ptr.as_ref() {
            package_mgr.count_packages()
        } else {
            0
        }
    }
}
