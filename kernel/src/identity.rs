//! Aura Identity — User profiles and authentication
//! Portable identity across AuraOS machines.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub id: u64,
    pub name: String,
    pub face_enrolled: bool,
    pub voice_enrolled: bool,
    pub typing_pattern: bool,
    pub preferences: UserPreferences,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub timezone_offset: i8,
    pub companion_name: String,
    pub companion_voice: String,
    pub text_size: u8,
    pub reduce_motion: bool,
}

impl UserPreferences {
    pub fn default_prefs() -> Self {
        UserPreferences {
            theme: String::from("dark"),
            language: String::from("en"),
            timezone_offset: -5,
            companion_name: String::from("Aura"),
            companion_voice: String::from("default"),
            text_size: 16,
            reduce_motion: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    Face,
    Voice,
    TypingPattern,
    PhoneProximity,
    Pin,
    Recovery,
}

pub struct IdentityStore {
    pub users: Vec<UserIdentity>,
    pub active_user: Option<u64>,
    next_id: u64,
}

impl IdentityStore {
    pub fn new() -> Self {
        IdentityStore {
            users: Vec::new(),
            active_user: None,
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.users.push(UserIdentity {
            id,
            name: String::from(name),
            face_enrolled: false,
            voice_enrolled: false,
            typing_pattern: false,
            preferences: UserPreferences::default_prefs(),
            created_at: 0, // TODO: RTC
        });
        
        // Auto-activate if first user
        if self.users.len() == 1 {
            self.active_user = Some(id);
        }
        
        id
    }

    pub fn get_active(&self) -> Option<&UserIdentity> {
        let id = self.active_user?;
        self.users.iter().find(|u| u.id == id)
    }

    pub fn switch_user(&mut self, id: u64) -> bool {
        if self.users.iter().any(|u| u.id == id) {
            self.active_user = Some(id);
            true
        } else {
            false
        }
    }

    pub fn enroll_face(&mut self, id: u64) {
        if let Some(u) = self.users.iter_mut().find(|u| u.id == id) {
            u.face_enrolled = true;
        }
    }

    pub fn enroll_voice(&mut self, id: u64) {
        if let Some(u) = self.users.iter_mut().find(|u| u.id == id) {
            u.voice_enrolled = true;
        }
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Serialize identity for transfer to another machine
    pub fn export_identity(&self, id: u64) -> Option<Vec<u8>> {
        let user = self.users.iter().find(|u| u.id == id)?;
        let mut data = Vec::new();
        
        // Simple serialization: name_len(u16) + name + flags + prefs
        let name_bytes = user.name.as_bytes();
        data.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        data.extend_from_slice(name_bytes);
        data.push(user.face_enrolled as u8);
        data.push(user.voice_enrolled as u8);
        data.push(user.typing_pattern as u8);
        data.push(user.preferences.timezone_offset as u8);
        data.push(user.preferences.text_size);
        data.push(user.preferences.reduce_motion as u8);
        
        Some(data)
    }
}
