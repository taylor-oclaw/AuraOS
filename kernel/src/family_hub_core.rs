extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut family_hub = FamilyHubCore::new();
    family_hub.add_member("Alice".into());
    family_hub.add_member("Bob".into());
    family_hub.add_member("Charlie".into());

    println!("Family members: {:?}", family_hub.get_members());

    if family_hub.is_member_present("Alice") {
        println!("Alice is present in the family.");
    } else {
        println!("Alice is not present in the family.");
    }

    let removed = family_hub.remove_member("Bob");
    if removed {
        println!("Bob has been removed from the family.");
    } else {
        println!("Failed to remove Bob from the family.");
    }

    println!("Family members after removal: {:?}", family_hub.get_members());

    loop {}
}

pub struct FamilyHubCore {
    members: Vec<String>,
}

impl FamilyHubCore {
    pub fn new() -> Self {
        FamilyHubCore {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: String) {
        if !self.is_member_present(&name) {
            self.members.push(name);
        }
    }

    pub fn remove_member(&mut self, name: &str) -> bool {
        let position = self.members.iter().position(|member| member == name);
        match position {
            Some(index) => {
                self.members.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn is_member_present(&self, name: &str) -> bool {
        self.members.contains(&String::from(name))
    }

    pub fn family_size(&self) -> usize {
        self.members.len()
    }
}
