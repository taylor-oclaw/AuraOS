extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct TrustDecentralizedID {
    id: String,
    attributes: Vec<String>,
    revocation_status: bool,
    owner: String,
    creation_time: u64,
}

impl TrustDecentralizedID {
    pub fn new(id: &str, owner: &str) -> Self {
        TrustDecentralizedID {
            id: String::from(id),
            attributes: Vec::new(),
            revocation_status: false,
            owner: String::from(owner),
            creation_time: 0, // Placeholder for actual time
        }
    }

    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(String::from(attribute));
    }

    pub fn remove_attribute(&mut self, attribute: &str) -> bool {
        if let Some(pos) = self.attributes.iter().position(|x| x == attribute) {
            self.attributes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_revoked(&self) -> bool {
        self.revocation_status
    }

    pub fn revoke(&mut self) {
        self.revocation_status = true;
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }
}
