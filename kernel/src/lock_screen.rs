extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum AuthMethod {
    Password,
    Pin,
    Biometric,
    Pattern,
}

pub enum LockReason {
    UserLocked,
    Timeout,
    Suspend,
    Boot,
}

pub struct LockScreen {
    pub locked: bool,
    pub auth_method: AuthMethod,
    pub password_hash: [u8; 32],
    pub failed_attempts: u32,
    pub max_attempts: u32,
    pub lock_reason: LockReason,
    pub lockout_until: Option<u64>,
    pub show_notifications: bool,
    pub wallpaper_path: Option<String>,
}

impl LockScreen {
    pub fn new() -> Self {
        Self {
            locked: false,
            auth_method: AuthMethod::Password,
            password_hash: [0; 32],
            failed_attempts: 0,
            max_attempts: 5,
            lock_reason: LockReason::Boot,
            lockout_until: None,
            show_notifications: true,
            wallpaper_path: None,
        }
    }

    pub fn lock(&mut self, reason: LockReason) {
        self.locked = true;
        self.lock_reason = reason;
        self.failed_attempts = 0;
    }

    pub fn attempt_unlock(&mut self, input_hash: &[u8; 32]) -> bool {
        if self.password_hash == *input_hash {
            self.locked = false;
            self.failed_attempts = 0;
            true
        } else {
            self.failed_attempts += 1;
            if self.failed_attempts >= self.max_attempts {
                self.lockout_until = Some(0);
            }
            false
        }
    }

    pub fn is_locked_out(&self) -> bool {
        self.lockout_until.is_some()
    }

    pub fn set_password_hash(&mut self, hash: [u8; 32]) {
        self.password_hash = hash;
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}
