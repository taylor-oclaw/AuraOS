#![no_std]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(const_option)]
#![feature(const_vec_new)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_mut_refs)]
#![feature(const_ptr_offset)]
#![feature(const_cast)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct Group {
    name: String,
    members: Vec<String>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            name: name.to_string(),
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: &str) {
        self.members.push(member.to_string());
    }

    pub fn remove_member(&mut self, member: &str) {
        if let Some(index) = self.members.iter().position(|m| m == member) {
            self.members.remove(index);
        }
    }

    pub fn get_members(&self) -> Vec<&String> {
        self.members.iter().collect()
    }

    pub fn has_member(&self, member: &str) -> bool {
        self.members.contains(member)
    }
}

struct GroupManager {
    groups: Vec<Group>,
}

impl GroupManager {
    pub fn new() -> Self {
        GroupManager { groups: Vec::new() }
    }

    pub fn create_group(&mut self, name: &str) {
        self.groups.push(Group::new(name));
    }

    pub fn add_member_to_group(&mut self, group_name: &str, member: &str) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == group_name) {
            group.add_member(member);
        }
    }

    pub fn remove_member_from_group(&mut self, group_name: &str, member: &str) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == group_name) {
            group.remove_member(member);
        }
    }

    pub fn get_members_of_group(&self, group_name: &str) -> Vec<&String> {
        if let Some(group) = self.groups.iter().find(|g| g.name == group_name) {
            group.get_members()
        } else {
            Vec::new()
        }
    }
}