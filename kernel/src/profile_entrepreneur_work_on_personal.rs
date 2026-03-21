extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileEntrepreneurWorkOnPersonal::new("John Doe".into(), 30, "AI Development".into());

    profile.add_project("Project Alpha", 2023);
    profile.add_skill("Machine Learning");
    profile.add_achievement("Best AI Hackathon Winner 2022");
    profile.update_experience_years(5);
    let summary = profile.get_summary();

    // Normally, you would send this summary to the kernel or log it
    // For demonstration, we'll just loop indefinitely
    loop {}
}

pub struct ProfileEntrepreneurWorkOnPersonal {
    name: String,
    age: u32,
    field_of_interest: String,
    projects: Vec<(String, u32)>, // (project_name, year)
    skills: Vec<String>,
    achievements: Vec<String>,
    experience_years: u32,
}

impl ProfileEntrepreneurWorkOnPersonal {
    pub fn new(name: String, age: u32, field_of_interest: String) -> Self {
        ProfileEntrepreneurWorkOnPersonal {
            name,
            age,
            field_of_interest,
            projects: Vec::new(),
            skills: Vec::new(),
            achievements: Vec::new(),
            experience_years: 0,
        }
    }

    pub fn add_project(&mut self, project_name: &str, year: u32) {
        self.projects.push((project_name.into(), year));
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(skill.into());
    }

    pub fn add_achievement(&mut self, achievement: &str) {
        self.achievements.push(achievement.into());
    }

    pub fn update_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("info");

        if !self.projects.is_empty() {
            summary.push_str("Projects:\n");
            for (project_name, year) in &self.projects {
                summary.push_str(&String::from("info")\n", project_name, year);
            }
        }

        if !self.skills.is_empty() {
            summary.push_str("Skills:\n");
            for skill in &self.skills {
                summary.push_str(&String::from("info"));
            }
        }

        if !self.achievements.is_empty() {
            summary.push_str("Achievements:\n");
            for achievement in &self.achievements {
                summary.push_str(&String::from("info"));
            }
        }

        summary
    }
}
