extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeLockControl {
    lock_status: bool,
    user_access_list: Vec<String>,
    log: Vec<String>,
}

impl SmartHomeLockControl {
    pub fn new() -> Self {
        SmartHomeLockControl {
            lock_status: true,
            user_access_list: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.lock_status
    }

    pub fn unlock(&mut self, user_id: &str) -> Result<(), String> {
        if self.user_access_list.contains(&String::from(user_id)) {
            self.lock_status = false;
            self.log.push(String::from("info"));
            Ok(())
        } else {
            Err(String::from("Access denied"))
        }
    }

    pub fn lock(&mut self) -> Result<(), String> {
        if !self.lock_status {
            self.lock_status = true;
            self.log.push(String::from("Locked"));
            Ok(())
        } else {
            Err(String::from("Already locked"))
        }
    }

    pub fn add_user(&mut self, user_id: &str) -> Result<(), String> {
        if !self.user_access_list.contains(&String::from(user_id)) {
            self.user_access_list.push(String::from(user_id));
            self.log.push(String::from("info"));
            Ok(())
        } else {
            Err(String::from("User already exists"))
        }
    }

    pub fn remove_user(&mut self, user_id: &str) -> Result<(), String> {
        if let Some(index) = self.user_access_list.iter().position(|x| x == user_id) {
            self.user_access_list.remove(index);
            self.log.push(String::from("info"));
            Ok(())
        } else {
            Err(String::from("User not found"))
        }
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }
}
