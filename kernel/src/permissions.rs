extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub const ADMIN_UID: u32 = 0;

const ALL_CAPABILITIES_MASK: u32 = Permission::Read.mask()
    | Permission::Write.mask()
    | Permission::Execute.mask()
    | Permission::Admin.mask()
    | Permission::Network.mask()
    | Permission::Audio.mask()
    | Permission::Camera.mask()
    | Permission::Storage.mask()
    | Permission::ProcessControl.mask();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Permission {
    Read = 0,
    Write = 1,
    Execute = 2,
    Admin = 3,
    Network = 4,
    Audio = 5,
    Camera = 6,
    Storage = 7,
    ProcessControl = 8,
}

impl Permission {
    pub const fn mask(self) -> u32 {
        1u32 << (self as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CapabilitySet(u32);

impl CapabilitySet {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(ALL_CAPABILITIES_MASK)
    }

    pub fn grant(&mut self, perm: Permission) {
        self.0 |= perm.mask();
    }

    pub fn revoke(&mut self, perm: Permission) {
        self.0 &= !perm.mask();
    }

    pub const fn has(&self, perm: Permission) -> bool {
        (self.0 & perm.mask()) != 0
    }

    pub const fn has_all(&self, other_set: Self) -> bool {
        (self.0 & other_set.0) == other_set.0
    }

    pub const fn has_any(&self, other_set: Self) -> bool {
        (self.0 & other_set.0) != 0
    }
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub uid: u32,
    pub name: String,
    pub capabilities: CapabilitySet,
    pub groups: Vec<String>,
}

pub struct SecurityManager {
    pub users: Vec<UserProfile>,
    next_uid: u32,
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityManager {
    pub fn new() -> Self {
        let mut users = Vec::new();
        users.push(UserProfile {
            uid: ADMIN_UID,
            name: String::from("admin"),
            capabilities: CapabilitySet::all(),
            groups: alloc::vec![String::from("admin")],
        });

        Self { users, next_uid: 1 }
    }

    pub fn check_permission(&self, uid: u32, perm: Permission) -> bool {
        self.users
            .iter()
            .find(|user| user.uid == uid)
            .map(|user| user.capabilities.has(perm))
            .unwrap_or(false)
    }

    pub fn create_user(&mut self, name: &str) -> u32 {
        let uid = self.next_uid;
        self.next_uid += 1;

        self.users.push(UserProfile {
            uid,
            name: String::from(name),
            capabilities: CapabilitySet::new(),
            groups: Vec::new(),
        });

        uid
    }

    pub fn add_to_group(&mut self, uid: u32, group: &str) -> bool {
        let Some(user) = self.users.iter_mut().find(|user| user.uid == uid) else {
            return false;
        };

        if !user.groups.iter().any(|existing| existing == group) {
            user.groups.push(String::from(group));
        }

        true
    }

    pub fn remove_user(&mut self, uid: u32) -> bool {
        if uid == ADMIN_UID {
            return false;
        }

        let Some(index) = self.users.iter().position(|user| user.uid == uid) else {
            return false;
        };

        self.users.remove(index);
        true
    }
}
