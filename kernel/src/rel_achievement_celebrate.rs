extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_achievement_celebrate_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_achievement_celebrate_exit() {
    // Cleanup logic for the module
}

pub struct Achievement {
    name: String,
    description: String,
    points: u32,
    completed: bool,
}

impl Achievement {
    pub fn new(name: &str, description: &str, points: u32) -> Self {
        Achievement {
            name: String::from(name),
            description: String::from(description),
            points,
            completed: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_points(&self) -> u32 {
        self.points
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub struct AchievementManager {
    achievements: Vec<Achievement>,
}

impl AchievementManager {
    pub fn new() -> Self {
        AchievementManager {
            achievements: Vec::new(),
        }
    }

    pub fn add_achievement(&mut self, achievement: Achievement) {
        self.achievements.push(achievement);
    }

    pub fn get_all_achievements(&self) -> &Vec<Achievement> {
        &self.achievements
    }

    pub fn find_achievement_by_name(&self, name: &str) -> Option<&Achievement> {
        self.achievements.iter().find(|&a| a.get_name() == name)
    }

    pub fn complete_achievement_by_name(&mut self, name: &str) {
        if let Some(achievement) = self.find_achievement_by_name(name) {
            achievement.complete();
        }
    }

    pub fn total_points(&self) -> u32 {
        self.achievements.iter().filter(|&a| a.is_completed()).map(|a| a.get_points()).sum()
    }
}
