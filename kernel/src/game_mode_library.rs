extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod game_mode_library {
    use super::*;

    pub struct GameModeLibrary {
        modes: Vec<String>,
        current_mode: usize,
    }

    impl GameModeLibrary {
        pub fn new() -> Self {
            GameModeLibrary {
                modes: Vec::new(),
                current_mode: 0,
            }
        }

        pub fn add_mode(&mut self, mode_name: &str) {
            self.modes.push(String::from(mode_name));
        }

        pub fn remove_mode(&mut self, mode_name: &str) -> bool {
            if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
                self.modes.remove(index);
                true
            } else {
                false
            }
        }

        pub fn set_current_mode(&mut self, mode_name: &str) -> bool {
            if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
                self.current_mode = index;
                true
            } else {
                false
            }
        }

        pub fn get_current_mode(&self) -> Option<&String> {
            self.modes.get(self.current_mode)
        }

        pub fn list_modes(&self) -> &Vec<String> {
            &self.modes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::game_mode_library::*;

    #[test]
    fn test_game_mode_library() {
        let mut library = GameModeLibrary::new();
        assert_eq!(library.list_modes().len(), 0);

        library.add_mode("Survival");
        library.add_mode("Creative");
        assert_eq!(library.list_modes().len(), 2);
        assert_eq!(library.get_current_mode(), Some(&String::from("Survival")));

        assert!(library.set_current_mode("Creative"));
        assert_eq!(library.get_current_mode(), Some(&String::from("Creative")));

        assert!(library.remove_mode("Survival"));
        assert_eq!(library.list_modes().len(), 1);
        assert_eq!(library.get_current_mode(), Some(&String::from("Creative")));

        assert!(!library.set_current_mode("Adventure"));
        assert_eq!(library.get_current_mode(), Some(&String::from("Creative")));
    }
}
