extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct GameModeAchievement {
    name: String,
    description: String,
    achieved: bool,
    progress: u32,
    max_progress: u32,
}

impl GameModeAchievement {
    pub fn new(name: &str, description: &str, max_progress: u32) -> Self {
        GameModeAchievement {
            name: String::from(name),
            description: String::from(description),
            achieved: false,
            progress: 0,
            max_progress,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn is_achieved(&self) -> bool {
        self.achieved
    }

    pub fn get_progress(&self) -> u32 {
        self.progress
    }

    pub fn update_progress(&mut self, amount: u32) {
        if !self.achieved {
            self.progress = core::cmp::min(self.progress + amount, self.max_progress);
            if self.progress == self.max_progress {
                self.achieved = true;
            }
        }
    }

    pub fn reset(&mut self) {
        self.achieved = false;
        self.progress = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_creation() {
        let achievement = GameModeAchievement::new("Test Achievement", "This is a test.", 100);
        assert_eq!(achievement.get_name(), "Test Achievement");
        assert_eq!(achievement.get_description(), "This is a test.");
        assert!(!achievement.is_achieved());
        assert_eq!(achievement.get_progress(), 0);
    }

    #[test]
    fn test_update_progress() {
        let mut achievement = GameModeAchievement::new("Test Achievement", "This is a test.", 100);
        achievement.update_progress(50);
        assert_eq!(achievement.get_progress(), 50);
        assert!(!achievement.is_achieved());

        achievement.update_progress(60);
        assert_eq!(achievement.get_progress(), 100);
        assert!(achievement.is_achieved());
    }

    #[test]
    fn test_reset() {
        let mut achievement = GameModeAchievement::new("Test Achievement", "This is a test.", 100);
        achievement.update_progress(100);
        assert!(achievement.is_achieved());

        achievement.reset();
        assert!(!achievement.is_achieved());
        assert_eq!(achievement.get_progress(), 0);
    }
}
