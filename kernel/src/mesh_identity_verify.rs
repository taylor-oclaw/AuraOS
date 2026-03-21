extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_identity_verify_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_identity_verify_exit() {
    // Cleanup logic for the module
}

pub struct MeshIdentityVerify {
    identities: Vec<String>,
}

impl MeshIdentityVerify {
    pub fn new() -> Self {
        MeshIdentityVerify {
            identities: Vec::new(),
        }
    }

    pub fn add_identity(&mut self, identity: String) {
        if !self.identities.contains(&identity) {
            self.identities.push(identity);
        }
    }

    pub fn remove_identity(&mut self, identity: &str) -> bool {
        let index = self.identities.iter().position(|id| id == identity);
        if let Some(i) = index {
            self.identities.remove(i);
            true
        } else {
            false
        }
    }

    pub fn verify_identity(&self, identity: &str) -> bool {
        self.identities.contains(&String::from(identity))
    }

    pub fn list_identities(&self) -> Vec<String> {
        self.identities.clone()
    }

    pub fn clear_identities(&mut self) {
        self.identities.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_identity_verify() {
        let mut verifier = MeshIdentityVerify::new();

        assert_eq!(verifier.list_identities(), Vec::<String>::new());

        verifier.add_identity(String::from("Alice"));
        verifier.add_identity(String::from("Bob"));

        assert_eq!(
            verifier.list_identities(),
            vec![String::from("Alice"), String::from("Bob")]
        );

        assert!(verifier.verify_identity("Alice"));
        assert!(!verifier.verify_identity("Charlie"));

        assert!(verifier.remove_identity("Alice"));
        assert!(!verifier.remove_identity("Alice"));

        assert_eq!(verifier.list_identities(), vec![String::from("Bob")]);

        verifier.clear_identities();
        assert_eq!(verifier.list_identities(), Vec::<String>::new());
    }
}
