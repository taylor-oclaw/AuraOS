extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut health_check = SkillHealthCheck::new();
    health_check.add_skill("AI Reasoning", 95);
    health_check.add_skill("Natural Language Processing", 85);
    health_check.add_skill("Computer Vision", 75);
    health_check.add_skill("Machine Learning", 80);
    health_check.add_skill("Data Analysis", 90);

    let average_score = health_check.calculate_average();
    let highest_skill = health_check.get_highest_skill().unwrap_or(String::from("No skills"));
    let lowest_skill = health_check.get_lowest_skill().unwrap_or(String::from("No skills"));

    println!("Average Skill Score: {}", average_score);
    println!("Highest Skill: {}", highest_skill);
    println!("Lowest Skill: {}", lowest_skill);

    if health_check.is_above_average(85) {
        println!("Most skills are above the average.");
    } else {
        println!("Most skills are below or equal to the average.");
    }
}

pub struct SkillHealthCheck {
    skills: Vec<(String, u8)>,
}

impl SkillHealthCheck {
    pub fn new() -> Self {
        SkillHealthCheck { skills: Vec::new() }
    }

    pub fn add_skill(&mut self, skill_name: &str, score: u8) {
        self.skills.push((String::from(skill_name), score));
    }

    pub fn calculate_average(&self) -> f32 {
        if self.skills.is_empty() {
            return 0.0;
        }
        let total_score: u32 = self.skills.iter().map(|(_, &score)| score as u32).sum();
        total_score as f32 / self.skills.len() as f32
    }

    pub fn get_highest_skill(&self) -> Option<String> {
        self.skills.iter()
            .max_by_key(|&(_, &score)| score)
            .map(|(skill_name, _)| skill_name.clone())
    }

    pub fn get_lowest_skill(&self) -> Option<String> {
        self.skills.iter()
            .min_by_key(|&(_, &score)| score)
            .map(|(skill_name, _)| skill_name.clone())
    }

    pub fn is_above_average(&self, threshold: u8) -> bool {
        let average_score = self.calculate_average();
        average_score >= threshold as f32
    }
}
