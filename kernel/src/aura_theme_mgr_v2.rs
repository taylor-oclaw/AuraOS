extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

struct AuraThemeMgr {
    themes: Vec<String>,
    current_theme: Option<usize>,
}

impl AuraThemeMgr {
    pub fn new() -> Self {
        AuraThemeMgr {
            themes: Vec::new(),
            current_theme: None,
        }
    }

    pub fn add_theme(&mut self, theme_name: &str) {
        let theme = String::from(theme_name);
        self.themes.push(theme);
    }

    pub fn remove_theme(&mut self, theme_name: &str) -> bool {
        if let Some(index) = self.themes.iter().position(|t| t == theme_name) {
            self.themes.remove(index);
            if self.current_theme == Some(index) {
                self.current_theme = None;
            }
            true
        } else {
            false
        }
    }

    pub fn set_current_theme(&mut self, theme_name: &str) -> bool {
        if let Some(index) = self.themes.iter().position(|t| t == theme_name) {
            self.current_theme = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_theme(&self) -> Option<&String> {
        self.current_theme.map(|index| &self.themes[index])
    }

    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.iter().collect()
    }
}

pub extern "C" fn aura_theme_mgr_new() -> *mut AuraThemeMgr {
    Box::into_raw(Box::new(AuraThemeMgr::new()))
}

pub extern "C" fn aura_theme_mgr_add_theme(mgr: *mut AuraThemeMgr, theme_name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(theme_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*mgr).add_theme(name);
        }
    }
}

pub extern "C" fn aura_theme_mgr_remove_theme(mgr: *mut AuraThemeMgr, theme_name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(theme_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*mgr).remove_theme(name)
        } else {
            false
        }
    }
}

pub extern "C" fn aura_theme_mgr_set_current_theme(mgr: *mut AuraThemeMgr, theme_name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(theme_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*mgr).set_current_theme(name)
        } else {
            false
        }
    }
}

pub extern "C" fn aura_theme_mgr_get_current_theme(mgr: *const AuraThemeMgr, buffer: *mut u8, len: usize) -> usize {
    unsafe {
        if let Some(theme) = (*mgr).get_current_theme() {
            let bytes_written = theme.as_bytes().len().min(len);
            core::ptr::copy_nonoverlapping(theme.as_ptr(), buffer, bytes_written);
            bytes_written
        } else {
            0
        }
    }
}

pub extern "C" fn aura_theme_mgr_list_themes(mgr: *const AuraThemeMgr, buffer: *mut u8, len: usize) -> usize {
    unsafe {
        let themes = (*mgr).list_themes();
        let mut total_bytes_written = 0;
        for theme in themes.iter() {
            if total_bytes_written + theme.len() + 1 > len {
                break;
            }
            core::ptr::copy_nonoverlapping(theme.as_ptr(), buffer.add(total_bytes_written), theme.len());
            total_bytes_written += theme.len();
            *buffer.add(total_bytes_written) = b'\0';
            total_bytes_written += 1;
        }
        total_bytes_written
    }
}

pub extern "C" fn aura_theme_mgr_free(mgr: *mut AuraThemeMgr) {
    unsafe {
        drop(Box::from_raw(mgr));
    }
}
