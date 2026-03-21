extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut family = ProfileModeFamily::new();
    family.add_member("Alice".into());
    family.add_member("Bob".into());
    family.remove_member("Alice".into());
    family.set_profile_mode(true);
    let members = family.get_members();
    // Do something with members or other logic
    loop {}
}

pub struct ProfileModeFamily {
    members: Vec<String>,
    profile_mode: bool,
}

impl ProfileModeFamily {
    pub fn new() -> Self {
        ProfileModeFamily {
            members: Vec::new(),
            profile_mode: false,
        }
    }

    pub fn add_member(&mut self, name: String) {
        if !self.members.contains(&name) {
            self.members.push(name);
        }
    }

    pub fn remove_member(&mut self, name: String) {
        self.members.retain(|member| member != &name);
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn set_profile_mode(&mut self, mode: bool) {
        self.profile_mode = mode;
    }

    pub fn is_profile_mode(&self) -> bool {
        self.profile_mode
    }
}
