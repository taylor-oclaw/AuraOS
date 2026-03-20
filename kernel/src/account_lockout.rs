extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccountLockout {
    username: String,
    failed_attempts: u32,
    max_attempts: u32,
    lock_duration: u64, // in seconds
    locked_until: u64,  // timestamp in seconds since epoch
}

impl AccountLockout {
    pub fn new(username: &str, max_attempts: u32, lock_duration: u64) -> Self {
        AccountLockout {
            username: String::from(username),
            failed_attempts: 0,
            max_attempts,
            lock_duration,
            locked_until: 0,
        }
    }

    pub fn record_attempt(&mut self) {
        if !self.is_locked() {
            self.failed_attempts += 1;
            if self.failed_attempts >= self.max_attempts {
                let current_time = self.current_time();
                self.locked_until = current_time + self.lock_duration;
            }
        }
    }

    pub fn is_locked(&self) -> bool {
        let current_time = self.current_time();
        self.locked_until > current_time
    }

    pub fn reset_attempts(&mut self) {
        if !self.is_locked() {
            self.failed_attempts = 0;
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn current_time(&self) -> u64 {
        // Placeholder for getting the current time in seconds since epoch
        // In a real kernel module, you would use a system call or hardware timer
        0
    }
}
