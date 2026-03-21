extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthHydrationRemind {
    reminders: Vec<String>,
    water_intake_goal: u32,
    current_water_intake: u32,
}

impl HealthHydrationRemind {
    pub fn new(water_intake_goal: u32) -> Self {
        HealthHydrationRemind {
            reminders: Vec::new(),
            water_intake_goal,
            current_water_intake: 0,
        }
    }

    pub fn add_reminder(&mut self, reminder: String) {
        self.reminders.push(reminder);
    }

    pub fn get_reminders(&self) -> &Vec<String> {
        &self.reminders
    }

    pub fn log_water_intake(&mut self, amount: u32) {
        self.current_water_intake += amount;
    }

    pub fn check_hydration_status(&self) -> String {
        if self.current_water_intake >= self.water_intake_goal {
            String::from("Hydration goal met!")
        } else {
            let remaining = self.water_intake_goal - self.current_water_intake;
            format!("Drink {} more ml to meet your hydration goal.", remaining)
        }
    }

    pub fn reset_hydration(&mut self) {
        self.current_water_intake = 0;
    }
}
