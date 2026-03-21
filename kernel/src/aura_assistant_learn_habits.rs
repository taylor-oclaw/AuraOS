extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code if needed
}

pub struct AuraAssistantLearnHabits {
    habits: Vec<String>,
}

impl AuraAssistantLearnHabits {
    pub fn new() -> Self {
        AuraAssistantLearnHabits {
            habits: Vec::new(),
        }
    }

    pub fn add_habit(&mut self, habit: String) {
        if !self.habits.contains(&habit) {
            self.habits.push(habit);
        }
    }

    pub fn remove_habit(&mut self, habit: &str) {
        self.habits.retain(|h| h != habit);
    }

    pub fn list_habits(&self) -> Vec<String> {
        self.habits.clone()
    }

    pub fn has_habit(&self, habit: &str) -> bool {
        self.habits.contains(&String::from(habit))
    }

    pub fn count_habits(&self) -> usize {
        self.habits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_list() {
        let mut assistant = AuraAssistantLearnHabits::new();
        assistant.add_habit(String::from("Read daily"));
        assert!(assistant.has_habit("Read daily"));
        assert_eq!(assistant.count_habits(), 1);

        assistant.remove_habit("Read daily");
        assert!(!assistant.has_habit("Read daily"));
        assert_eq!(assistant.count_habits(), 0);
    }

    #[test]
    fn test_list_habits() {
        let mut assistant = AuraAssistantLearnHabits::new();
        assistant.add_habit(String::from("Exercise"));
        assistant.add_habit(String::from("Meditate"));

        let habits = assistant.list_habits();
        assert_eq!(habits.len(), 2);
        assert!(habits.contains(&String::from("Exercise")));
        assert!(habits.contains(&String::from("Meditate")));
    }
}
