extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_contact_share_team_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn rel_contact_share_team_exit() {
    // Cleanup code for the module
}

pub struct ContactShareTeam {
    team_name: String,
    members: Vec<String>,
}

impl ContactShareTeam {
    pub fn new(team_name: &str) -> Self {
        ContactShareTeam {
            team_name: String::from(team_name),
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member_name: &str) {
        self.members.push(String::from(member_name));
    }

    pub fn remove_member(&mut self, member_name: &str) -> bool {
        if let Some(index) = self.members.iter().position(|m| m == member_name) {
            self.members.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_team_name(&self) -> &str {
        &self.team_name
    }

    pub fn list_members(&self) -> &[String] {
        &self.members
    }

    pub fn is_member(&self, member_name: &str) -> bool {
        self.members.contains(&String::from(member_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_share_team() {
        let mut team = ContactShareTeam::new("AI Team");
        assert_eq!(team.get_team_name(), "AI Team");
        assert!(!team.is_member("Alice"));

        team.add_member("Alice");
        assert!(team.is_member("Alice"));
        assert_eq!(team.list_members().len(), 1);

        team.add_member("Bob");
        assert!(team.is_member("Bob"));
        assert_eq!(team.list_members().len(), 2);

        team.remove_member("Alice");
        assert!(!team.is_member("Alice"));
        assert_eq!(team.list_members().len(), 1);
    }
}
