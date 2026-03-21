extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod kernel_module {
    use super::*;

    pub struct CoworkerDepartment {
        name: String,
        members: Vec<String>,
    }

    impl CoworkerDepartment {
        pub fn new(name: &str) -> Self {
            CoworkerDepartment {
                name: String::from(name),
                members: Vec::new(),
            }
        }

        pub fn add_member(&mut self, member_name: &str) {
            self.members.push(String::from(member_name));
        }

        pub fn remove_member(&mut self, member_name: &str) -> bool {
            if let Some(index) = self.members.iter().position(|m| m == member_name) {
                self.members.remove(index);
                true
            } else {
                false
            }
        }

        pub fn get_members(&self) -> &[String] {
            &self.members
        }

        pub fn has_member(&self, member_name: &str) -> bool {
            self.members.contains(&String::from(member_name))
        }

        pub fn department_size(&self) -> usize {
            self.members.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coworker_department() {
        let mut dept = CoworkerDepartment::new("AI Team");
        assert_eq!(dept.department_size(), 0);

        dept.add_member("Alice");
        dept.add_member("Bob");
        assert_eq!(dept.department_size(), 2);
        assert!(dept.has_member("Alice"));
        assert!(dept.has_member("Bob"));

        let members = dept.get_members();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0], "Alice");
        assert_eq!(members[1], "Bob");

        assert!(dept.remove_member("Alice"));
        assert!(!dept.has_member("Alice"));
        assert_eq!(dept.department_size(), 1);

        assert!(!dept.remove_member("Charlie"));
        assert_eq!(dept.department_size(), 1);
    }
}
