extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyInviteHandler {
    invites: Vec<String>,
}

impl FamilyInviteHandler {
    pub fn new() -> Self {
        FamilyInviteHandler {
            invites: Vec::new(),
        }
    }

    pub fn add_invite(&mut self, invitee: &str) {
        if !self.invites.contains(&String::from(invitee)) {
            self.invites.push(String::from(invitee));
        }
    }

    pub fn remove_invite(&mut self, invitee: &str) {
        self.invites.retain(|i| i != invitee);
    }

    pub fn list_invites(&self) -> Vec<String> {
        self.invites.clone()
    }

    pub fn has_invite(&self, invitee: &str) -> bool {
        self.invites.contains(&String::from(invitee))
    }

    pub fn clear_invites(&mut self) {
        self.invites.clear();
    }
}
