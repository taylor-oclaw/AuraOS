extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut family = SkillScopeFamily::new();
    family.add_member("Alice".into(), vec!["Programming", "Machine Learning"].into());
    family.add_member("Bob".into(), vec!["Networking", "Security"].into());

    println!("Family Members:");
    for member in family.get_members() {
        println!("Name: {}, Skills: {:?}", member.name, member.skills);
    }

    if let Some(member) = family.find_member_by_name("Alice") {
        println!("Found Alice with skills: {:?}", member.skills);
    } else {
        println!("Alice not found.");
    }

    family.remove_member("Bob");
    println!("After removing Bob:");
    for member in family.get_members() {
        println!("Name: {}, Skills: {:?}", member.name, member.skills);
    }
}

pub struct SkillScopeFamily {
    members: Vec<Member>,
}

impl SkillScopeFamily {
    pub fn new() -> Self {
        SkillScopeFamily {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: String, skills: Vec<String>) {
        let member = Member { name, skills };
        self.members.push(member);
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn find_member_by_name(&self, name: &str) -> Option<&Member> {
        self.members.iter().find(|member| member.name == name)
    }

    pub fn remove_member(&mut self, name: &str) {
        self.members.retain(|member| member.name != name);
    }
}

struct Member {
    name: String,
    skills: Vec<String>,
}
