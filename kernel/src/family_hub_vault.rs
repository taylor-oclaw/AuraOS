extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct FamilyHubVault {
    members: Vec<String>,
    secrets: Vec<String>,
}

impl FamilyHubVault {
    pub fn new() -> Self {
        FamilyHubVault {
            members: Vec::new(),
            secrets: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: String) {
        if !self.members.contains(&name) {
            self.members.push(name);
        }
    }

    pub fn remove_member(&mut self, name: &str) -> bool {
        let index = self.members.iter().position(|x| x == name);
        match index {
            Some(i) => {
                self.members.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn add_secret(&mut self, secret: String) {
        self.secrets.push(secret);
    }

    pub fn get_secrets_count(&self) -> usize {
        self.secrets.len()
    }

    pub fn list_members(&self) -> Vec<String> {
        self.members.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_hub_vault() {
        let mut vault = FamilyHubVault::new();
        assert_eq!(vault.list_members().len(), 0);
        assert_eq!(vault.get_secrets_count(), 0);

        vault.add_member(String::from("Alice"));
        vault.add_member(String::from("Bob"));
        assert_eq!(vault.list_members().len(), 2);

        vault.remove_member("Alice");
        assert_eq!(vault.list_members().len(), 1);
        assert!(!vault.remove_member("Charlie"));

        vault.add_secret(String::from("Secret1"));
        vault.add_secret(String::from("Secret2"));
        assert_eq!(vault.get_secrets_count(), 2);
    }
}
