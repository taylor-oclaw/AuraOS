extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantMorningBrief {
    user_name: String,
    date: String,
    weather: String,
    tasks: Vec<String>,
    reminders: Vec<String>,
}

impl AuraAssistantMorningBrief {
    pub fn new(user_name: &str, date: &str, weather: &str) -> Self {
        AuraAssistantMorningBrief {
            user_name: String::from(user_name),
            date: String::from(date),
            weather: String::from(weather),
            tasks: Vec::new(),
            reminders: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn remove_task(&mut self, task_index: usize) -> Option<String> {
        if task_index < self.tasks.len() {
            Some(self.tasks.remove(task_index))
        } else {
            None
        }
    }

    pub fn add_reminder(&mut self, reminder: &str) {
        self.reminders.push(String::from(reminder));
    }

    pub fn remove_reminder(&mut self, reminder_index: usize) -> Option<String> {
        if reminder_index < self.reminders.len() {
            Some(self.reminders.remove(reminder_index))
        } else {
            None
        }
    }

    pub fn get_morning_brief(&self) -> String {
        let mut brief = format!("Good morning, {}!\nToday is {}\nWeather: {}\n\nTasks:\n", self.user_name, self.date, self.weather);
        for (index, task) in self.tasks.iter().enumerate() {
            brief.push_str(&format!("{}. {}\n", index + 1, task));
        }
        brief.push_str("\nReminders:\n");
        for (index, reminder) in self.reminders.iter().enumerate() {
            brief.push_str(&format!("{}. {}\n", index + 1, reminder));
        }
        brief
    }
}
