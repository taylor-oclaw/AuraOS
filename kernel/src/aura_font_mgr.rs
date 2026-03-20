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

struct AuraFontMgr {
    fonts: Vec<String>,
}

impl AuraFontMgr {
    pub fn new() -> Self {
        AuraFontMgr {
            fonts: Vec::new(),
        }
    }

    pub fn add_font(&mut self, font_name: &str) {
        self.fonts.push(String::from(font_name));
    }

    pub fn remove_font(&mut self, font_name: &str) {
        if let Some(index) = self.fonts.iter().position(|f| f == font_name) {
            self.fonts.remove(index);
        }
    }

    pub fn list_fonts(&self) -> Vec<String> {
        self.fonts.clone()
    }

    pub fn has_font(&self, font_name: &str) -> bool {
        self.fonts.contains(&String::from(font_name))
    }

    pub fn count_fonts(&self) -> usize {
        self.fonts.len()
    }
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_new() -> *mut AuraFontMgr {
    Box::into_raw(Box::new(AuraFontMgr::new()))
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_add_font(font_mgr: *mut AuraFontMgr, font_name: *const u8, len: usize) {
    if let Some(font_mgr) = unsafe { font_mgr.as_mut() } {
        let font_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(font_name, len)) };
        font_mgr.add_font(font_str);
    }
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_remove_font(font_mgr: *mut AuraFontMgr, font_name: *const u8, len: usize) {
    if let Some(font_mgr) = unsafe { font_mgr.as_mut() } {
        let font_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(font_name, len)) };
        font_mgr.remove_font(font_str);
    }
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_list_fonts(font_mgr: *const AuraFontMgr) -> Vec<String> {
    if let Some(font_mgr) = unsafe { font_mgr.as_ref() } {
        font_mgr.list_fonts()
    } else {
        Vec::new()
    }
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_has_font(font_mgr: *const AuraFontMgr, font_name: *const u8, len: usize) -> bool {
    if let Some(font_mgr) = unsafe { font_mgr.as_ref() } {
        let font_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(font_name, len)) };
        font_mgr.has_font(font_str)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn aura_font_mgr_count_fonts(font_mgr: *const AuraFontMgr) -> usize {
    if let Some(font_mgr) = unsafe { font_mgr.as_ref() } {
        font_mgr.count_fonts()
    } else {
        0
    }
}
