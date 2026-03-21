extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_referral_track_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_referral_track_exit() {
    // Cleanup logic for the module
}

pub struct ReferralTracker {
    referrals: Vec<String>,
}

impl ReferralTracker {
    pub fn new() -> Self {
        ReferralTracker {
            referrals: Vec::new(),
        }
    }

    pub fn add_referral(&mut self, referral: String) {
        self.referrals.push(referral);
    }

    pub fn get_referrals_count(&self) -> usize {
        self.referrals.len()
    }

    pub fn has_referral(&self, referral: &str) -> bool {
        self.referrals.contains(&String::from(referral))
    }

    pub fn remove_referral(&mut self, referral: &str) {
        if let Some(index) = self.referrals.iter().position(|r| r == referral) {
            self.referrals.remove(index);
        }
    }

    pub fn list_referrals(&self) -> Vec<String> {
        self.referrals.clone()
    }
}
