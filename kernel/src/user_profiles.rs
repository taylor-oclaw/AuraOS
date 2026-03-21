extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct UserProfile {
    pub id: u64,
    pub name: String,
    pub trust_zone_level: u8,
    pub agent_budget: u64,
    pub preferences: Vec<(String, String)>,
    pub active: bool,
    pub created_at: u64,
    pub last_login: u64
}

pub struct ProfileManager {
    pub profiles: Vec<UserProfile>,
    pub active_profile: Option<u64>,
    pub next_id: u64
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            active_profile: None,
            next_id: 1
        }
    }

    pub fn create_profile(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.profiles.push(UserProfile {
            id,
            name: String::from(name),
            trust_zone_level: 2,
            agent_budget: 1000,
            preferences: Vec::new(),
            active: false,
            created_at: 0,
            last_login: 0
        };
        id
    }

    pub fn login(&mut self, id: u64) -> bool {
        if let Some(p) = self.profiles.iter_mut().find(|p| p.id == id) {
            p.active = true;
            p.last_login = 0;
            self.active_profile = Some(id);
            true
        } else {
            false
        }
    }

    pub fn logout(&mut self) {
        if let Some(id) = self.active_profile {
            if let Some(p) = self.profiles.iter_mut().find(|p| p.id == id) {
                p.active = false;
            }
        }
        self.active_profile = None;
    }

    pub fn set_preference(&mut self, id: u64, key: &str, value: &str) {
        if let Some(p) = self.profiles.iter_mut().find(|p| p.id == id) {
            if let Some(pref) = p.preferences.iter_mut().find(|(k, _)| k == key) {
                pref.1 = String::from(value);
            } else {
                p.preferences.push((String::from(key), String::from(value)));
            }
        }
    }

    pub fn current_user(&self) -> Option<&UserProfile> {
        self.active_profile.and_then(|id| self.profiles.iter().find(|p| p.id == id))
    }
)}
