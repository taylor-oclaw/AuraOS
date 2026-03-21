extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_mentor_mentee_init() {
}

pub extern "C" fn rel_mentor_mentee_exit() {
}

pub struct MentorMenteeRelationship {
    mentor: String,
    mentees: Vec<String>,
}

impl MentorMenteeRelationship {
    pub fn new(mentor: &str) -> Self {
        MentorMenteeRelationship {
            mentor: String::from(mentor),
            mentees: Vec::new(),
        }
    }

    pub fn add_mentee(&mut self, mentee: &str) {
        self.mentees.push(String::from(mentee));
    }

    pub fn remove_mentee(&mut self, mentee: &str) -> bool {
        if let Some(index) = self.mentees.iter().position(|m| m == mentee) {
            self.mentees.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_mentees(&self) -> Vec<String> {
        self.mentees.clone()
    }

    pub fn count_mentees(&self) -> usize {
        self.mentees.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mentor = "Alice";
        let mmr = MentorMenteeRelationship::new(mentor);
        assert_eq!(mmr.mentor, String::from(mentor));
        assert_eq!(mmr.mentees.len(), 0);
    }

    #[test]
    fn test_add_remove_mentee() {
        let mentor = "Bob";
        let mut mmr = MentorMenteeRelationship::new(mentor);
        let mentee1 = "Charlie";
        let mentee2 = "David";

        mmr.add_mentee(mentee1);
        assert_eq!(mmr.count_mentees(), 1);

        mmr.add_mentee(mentee2);
        assert_eq!(mmr.count_mentees(), 2);

        assert!(mmr.remove_mentee(mentee1));
        assert_eq!(mmr.count_mentees(), 1);

        assert!(!mmr.remove_mentee("Eve"));
        assert_eq!(mmr.count_mentees(), 1);
    }

    #[test]
    fn test_list_mentees() {
        let mentor = "Eve";
        let mut mmr = MentorMenteeRelationship::new(mentor);
        let mentee1 = "Frank";
        let mentee2 = "Grace";

        mmr.add_mentee(mentee1);
        mmr.add_mentee(mentee2);

        let mentees = mmr.list_mentees();
        assert_eq!(mentees.len(), 2);
        assert!(mentees.contains(&String::from(mentee1)));
        assert!(mentees.contains(&String::from(mentee2)));
    }
}
