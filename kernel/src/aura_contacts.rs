extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AuraContacts {
    contacts: Vec<(String, String)>, // (name, phone_number)
}

impl AuraContacts {
    pub fn new() -> Self {
        AuraContacts {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, name: &str, phone_number: &str) {
        let name = String::from(name);
        let phone_number = String::from(phone_number);
        self.contacts.push((name, phone_number));
    }

    pub fn remove_contact(&mut self, name: &str) -> bool {
        let pos = self.contacts.iter().position(|&(ref n, _)| n == name);
        if let Some(index) = pos {
            self.contacts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_contact(&self, name: &str) -> Option<&String> {
        for (contact_name, phone_number) in &self.contacts {
            if contact_name == name {
                return Some(phone_number);
            }
        }
        None
    }

    pub fn list_contacts(&self) -> Vec<(String, String)> {
        self.contacts.clone()
    }

    pub fn count_contacts(&self) -> usize {
        self.contacts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_contacts() {
        let mut aura_contacts = AuraContacts::new();
        assert_eq!(aura_contacts.count_contacts(), 0);

        aura_contacts.add_contact("Alice", "123-456-7890");
        aura_contacts.add_contact("Bob", "987-654-3210");
        assert_eq!(aura_contacts.count_contacts(), 2);

        let alice_phone = aura_contacts.get_contact("Alice").unwrap();
        assert_eq!(alice_phone, "123-456-7890");

        let bob_removed = aura_contacts.remove_contact("Bob");
        assert!(bob_removed);
        assert_eq!(aura_contacts.count_contacts(), 1);

        let contacts_list = aura_contacts.list_contacts();
        assert_eq!(contacts_list.len(), 1);
        assert_eq!(contacts_list[0], (String::from("Alice"), String::from("123-456-7890")));
    }
}
