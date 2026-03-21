extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let family = FamilyHub::new(String::from("Smith"));
    family.add_member(String::from("John"), 30);
    family.add_member(String::from("Jane"), 28);
    family.remove_member(String::from("John"));
    let members = family.get_members();
    let oldest_age = family.oldest_member_age();
    loop {}
}

pub struct FamilyHub {
    name: String,
    members: Vec<(String, u32)>,
}

impl FamilyHub {
    pub fn new(name: String) -> Self {
        FamilyHub {
            name,
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: String, age: u32) {
        self.members.push((name, age));
    }

    pub fn remove_member(&mut self, name: String) {
        self.members.retain(|&(ref member_name, _)| member_name != &name);
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn oldest_member_age(&self) -> Option<u32> {
        self.members.iter().max_by_key(|&(_, age)| age).map(|&(_, age)| age)
    }

    pub fn family_name(&self) -> &str {
        &self.name
    }
}
