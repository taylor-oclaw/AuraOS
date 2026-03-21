extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileWFHBurnoutDetect {
    user_profiles: Vec<UserProfile>,
}

impl ProfileWFHBurnoutDetect {
    pub fn new() -> Self {
        ProfileWFHBurnoutDetect {
            user_profiles: Vec::new(),
        }
    }

    pub fn add_user_profile(&mut self, profile: UserProfile) {
        self.user_profiles.push(profile);
    }

    pub fn remove_user_profile(&mut self, user_id: &str) -> bool {
        let pos = self.user_profiles.iter().position(|p| p.id == user_id);
        if let Some(index) = pos {
            self.user_profiles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_user_profile(&self, user_id: &str) -> Option<&UserProfile> {
        self.user_profiles.iter().find(|p| p.id == user_id)
    }

    pub fn detect_burnout(&self, user_id: &str) -> bool {
        if let Some(profile) = self.get_user_profile(user_id) {
            // Simple burnout detection logic (example)
            profile.work_hours > 80 && profile.stress_level > 7
        } else {
            false
        }
    }

    pub fn update_user_work_hours(&mut self, user_id: &str, hours: u32) -> bool {
        if let Some(profile) = self.user_profiles.iter_mut().find(|p| p.id == user_id) {
            profile.work_hours = hours;
            true
        } else {
            false
        }
    }

    pub fn update_user_stress_level(&mut self, user_id: &str, level: u8) -> bool {
        if let Some(profile) = self.user_profiles.iter_mut().find(|p| p.id == user_id) {
            profile.stress_level = level;
            true
        } else {
            false
        }
    }
}

struct UserProfile {
    id: String,
    work_hours: u32,
    stress_level: u8, // 0-10 scale
}

impl UserProfile {
    pub fn new(id: String, work_hours: u32, stress_level: u8) -> Self {
        UserProfile {
            id,
            work_hours,
            stress_level,
        }
    }
}
