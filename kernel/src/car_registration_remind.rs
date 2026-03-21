extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CarRegistrationRemind {
    registrations: Vec<(String, u32)>, // (license plate, days until expiration)
}

impl CarRegistrationRemind {
    pub fn new() -> Self {
        CarRegistrationRemind {
            registrations: Vec::new(),
        }
    }

    pub fn add_registration(&mut self, license_plate: String, days_until_expiration: u32) {
        self.registrations.push((license_plate, days_until_expiration));
    }

    pub fn remove_registration(&mut self, license_plate: &str) -> bool {
        let pos = self
            .registrations
            .iter()
            .position(|(lp, _)| lp == license_plate);
        if let Some(index) = pos {
            self.registrations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_days_until_expiration(&self, license_plate: &str) -> Option<u32> {
        self.registrations
            .iter()
            .find(|(lp, _)| lp == license_plate)
            .map(|(_, days)| *days)
    }

    pub fn update_days_until_expiration(
        &mut self,
        license_plate: &str,
        new_days: u32,
     -> bool {
        if let Some((_, days)) = self
            .registrations
            .iter_mut()
            .find(|(lp, _)| lp == license_plate)
        {
            *days = new_days;
            true
        } else {
            false
        }
    }

    pub fn list_all_registrations(&self) -> Vec<(String, u32)> {
        self.registrations.clone()
    }
}
