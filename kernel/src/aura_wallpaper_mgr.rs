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

struct WallpaperManager {
    wallpapers: Vec<String>,
    current_wallpaper_index: usize,
}

impl WallpaperManager {
    pub fn new() -> Self {
        WallpaperManager {
            wallpapers: Vec::new(),
            current_wallpaper_index: 0,
        }
    }

    pub fn add_wallpaper(&mut self, wallpaper_path: &str) {
        self.wallpapers.push(String::from(wallpaper_path));
    }

    pub fn remove_wallpaper(&mut self, index: usize) -> Option<String> {
        if index < self.wallpapers.len() {
            Some(self.wallpapers.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_wallpaper(&self) -> Option<&String> {
        if !self.wallpapers.is_empty() {
            Some(&self.wallpapers[self.current_wallpaper_index])
        } else {
            None
        }
    }

    pub fn set_next_wallpaper(&mut self) {
        if !self.wallpapers.is_empty() {
            self.current_wallpaper_index = (self.current_wallpaper_index + 1) % self.wallpapers.len();
        }
    }

    pub fn set_previous_wallpaper(&mut self) {
        if !self.wallpapers.is_empty() {
            self.current_wallpaper_index = if self.current_wallpaper_index == 0 {
                self.wallpapers.len() - 1
            } else {
                self.current_wallpaper_index - 1
            };
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_new() -> *mut WallpaperManager {
    Box::into_raw(Box::new(WallpaperManager::new()))
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_add_wallpaper(mgr: *mut WallpaperManager, wallpaper_path: *const u8) {
    unsafe {
        if let Some(wmgr) = mgr.as_mut() {
            let c_str = core::ffi::CStr::from_ptr(wallpaper_path as *const _);
            if let Ok(s) = c_str.to_str() {
                wmgr.add_wallpaper(s);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_remove_wallpaper(mgr: *mut WallpaperManager, index: usize) -> *mut String {
    unsafe {
        if let Some(wmgr) = mgr.as_mut() {
            if let Some(removed) = wmgr.remove_wallpaper(index) {
                return Box::into_raw(Box::new(removed));
            }
        }
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_get_current_wallpaper(mgr: *const WallpaperManager) -> *const String {
    unsafe {
        if let Some(wmgr) = mgr.as_ref() {
            if let Some(current) = wmgr.get_current_wallpaper() {
                return current as *const _;
            }
        }
        core::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_set_next_wallpaper(mgr: *mut WallpaperManager) {
    unsafe {
        if let Some(wmgr) = mgr.as_mut() {
            wmgr.set_next_wallpaper();
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_wallpaper_mgr_set_previous_wallpaper(mgr: *mut WallpaperManager) {
    unsafe {
        if let Some(wmgr) = mgr.as_mut() {
            wmgr.set_previous_wallpaper();
        }
    }
}
