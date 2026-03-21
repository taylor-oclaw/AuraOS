extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyMember {
    name: String,
    age: u8,
    relation: String,
}

impl FamilyMember {
    pub fn new(name: &str, age: u8, relation: &str) -> Self {
        FamilyMember {
            name: String::from(name),
            age,
            relation: String::from(relation),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_relation(&self) -> &str {
        &self.relation
    }
}

struct Family {
    members: Vec<FamilyMember>,
}

impl Family {
    pub fn new() -> Self {
        Family {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: FamilyMember) {
        self.members.push(member);
    }

    pub fn remove_member_by_name(&mut self, name: &str) {
        self.members.retain(|m| m.get_name() != name);
    }

    pub fn get_members(&self) -> &[FamilyMember] {
        &self.members
    }

    pub fn find_member_by_name(&self, name: &str) -> Option<&FamilyMember> {
        self.members.iter().find(|m| m.get_name() == name)
    }
}

pub extern "C" fn create_family() -> *mut Family {
    Box::into_raw(Box::new(Family::new()))
}

pub extern "C" fn add_member_to_family(family: *mut Family, name: &str, age: u8, relation: &str) {
    unsafe {
        (*family).add_member(FamilyMember::new(name, age, relation));
    }
}

pub extern "C" fn remove_member_from_family_by_name(family: *mut Family, name: &str) {
    unsafe {
        (*family).remove_member_by_name(name);
    }
}

pub extern "C" fn get_members_count(family: *const Family) -> usize {
    unsafe { (*family).get_members().len() }
}

pub extern "C" fn find_member_by_name(family: *const Family, name: &str) -> Option<&FamilyMember> {
    unsafe { (*family).find_member_by_name(name) }
}
