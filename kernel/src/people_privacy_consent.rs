extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PeoplePrivacyConsent {
    consent_id: u32,
    user_id: String,
    data_types: Vec<String>,
    consent_date: String,
    expiration_date: Option<String>,
}

impl PeoplePrivacyConsent {
    pub fn new(consent_id: u32, user_id: &str, data_types: &[&str], consent_date: &str) -> Self {
        PeoplePrivacyConsent {
            consent_id,
            user_id: String::from(user_id),
            data_types: data_types.iter().map(|&s| String::from(s)).collect(),
            consent_date: String::from(consent_date),
            expiration_date: None,
        }
    }

    pub fn add_data_type(&mut self, data_type: &str) {
        if !self.data_types.contains(&String::from(data_type)) {
            self.data_types.push(String::from(data_type));
        }
    }

    pub fn remove_data_type(&mut self, data_type: &str) {
        self.data_types.retain(|dt| dt != data_type);
    }

    pub fn set_expiration_date(&mut self, expiration_date: &str) {
        self.expiration_date = Some(String::from(expiration_date));
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expiration_date) = &self.expiration_date {
            // Simple comparison assuming dates are in YYYY-MM-DD format
            return expiration_date < &self.consent_date;
        }
        false
    }

    pub fn get_data_types(&self) -> Vec<String> {
        self.data_types.clone()
    }
}
