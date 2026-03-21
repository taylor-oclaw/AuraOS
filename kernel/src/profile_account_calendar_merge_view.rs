extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileAccountCalendarMergeView {
    profiles: Vec<String>,
    accounts: Vec<String>,
    calendars: Vec<String>,
}

impl ProfileAccountCalendarMergeView {
    pub fn new() -> Self {
        ProfileAccountCalendarMergeView {
            profiles: Vec::new(),
            accounts: Vec::new(),
            calendars: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        self.profiles.push(String::from(profile_name));
    }

    pub fn add_account(&mut self, account_name: &str) {
        self.accounts.push(String::from(account_name));
    }

    pub fn add_calendar(&mut self, calendar_name: &str) {
        self.calendars.push(String::from(calendar_name));
    }

    pub fn get_profiles_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn get_accounts_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn get_calendars_count(&self) -> usize {
        self.calendars.len()
    }
}
