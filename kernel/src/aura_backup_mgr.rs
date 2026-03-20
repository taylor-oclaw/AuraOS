extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct BackupManager {
    backups: Vec<String>,
}

impl BackupManager {
    pub fn new() -> Self {
        BackupManager {
            backups: Vec::new(),
        }
    }

    pub fn add_backup(&mut self, backup_name: &str) {
        self.backups.push(String::from(backup_name));
    }

    pub fn remove_backup(&mut self, backup_name: &str) {
        if let Some(index) = self.backups.iter().position(|b| b == backup_name) {
            self.backups.remove(index);
        }
    }

    pub fn list_backups(&self) -> Vec<String> {
        self.backups.clone()
    }

    pub fn has_backup(&self, backup_name: &str) -> bool {
        self.backups.contains(&String::from(backup_name))
    }

    pub fn count_backups(&self) -> usize {
        self.backups.len()
    }
}

pub extern "C" fn aura_backup_mgr_new() -> *mut BackupManager {
    Box::into_raw(Box::new(BackupManager::new()))
}

pub extern "C" fn aura_backup_mgr_add_backup(manager: *mut BackupManager, backup_name: *const u8, len: usize) {
    unsafe {
        let manager = &mut *manager;
        let backup_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(backup_name, len));
        manager.add_backup(backup_name_str);
    }
}

pub extern "C" fn aura_backup_mgr_remove_backup(manager: *mut BackupManager, backup_name: *const u8, len: usize) {
    unsafe {
        let manager = &mut *manager;
        let backup_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(backup_name, len));
        manager.remove_backup(backup_name_str);
    }
}

pub extern "C" fn aura_backup_mgr_list_backups(manager: *const BackupManager) -> Vec<String> {
    unsafe { (*manager).list_backups() }
}

pub extern "C" fn aura_backup_mgr_has_backup(manager: *const BackupManager, backup_name: *const u8, len: usize) -> bool {
    unsafe {
        let manager = &*manager;
        let backup_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(backup_name, len));
        manager.has_backup(backup_name_str)
    }
}

pub extern "C" fn aura_backup_mgr_count_backups(manager: *const BackupManager) -> usize {
    unsafe { (*manager).count_backups() }
}
