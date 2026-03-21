extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut project = biz_coworker_project_shared::Project::new(String::from("AI-Native OS"));
    project.add_member(String::from("Alice"));
    project.add_member(String::from("Bob"));
    project.set_description(String::from("Development of AI-native operating system."));
    project.update_status("In Progress");
    let members = project.get_members();
    for member in members {
    }
    loop {}
}

mod biz_coworker_project_shared {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct Project {
        name: String,
        description: String,
        status: String,
        members: Vec<String>,
    }

    impl Project {
        pub fn new(name: String) -> Self {
            Project {
                name,
                description: String::from(""),
                status: String::from("Not Started"),
                members: Vec::new(),
            }
        }

        pub fn add_member(&mut self, member: String) {
            self.members.push(member);
        }

        pub fn set_description(&mut self, description: String) {
            self.description = description;
        }

        pub fn update_status(&mut self, status: String) {
            self.status = status;
        }

        pub fn get_members(&self) -> &Vec<String> {
            &self.members
        }

        pub fn get_description(&self) -> &String {
            &self.description
        }
    }
}
