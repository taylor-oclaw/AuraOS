extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let action_item = ActionItemAutoExecute::new("Task1", "Description of Task1");
    action_item.execute();
    action_item.update_status("Completed");
    println!("Updated status: {}", action_item.get_status());
    action_item.add_comment(String::from("Comment on Task1"));
    action_item.log_activity(String::from("Activity logged for Task1"));
}

pub struct ActionItemAutoExecute {
    title: String,
    description: String,
    status: String,
    comments: Vec<String>,
    activities: Vec<String>,
}

impl ActionItemAutoExecute {
    pub fn new(title: &str, description: &str) -> Self {
        ActionItemAutoExecute {
            title: String::from(title),
            description: String::from(description),
            status: String::from("Pending"),
            comments: Vec::new(),
            activities: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        // Simulate task execution
        self.status = String::from("Executing");
        println!("Task {} is now executing.", self.title);
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
        println!("Status of Task {} updated to {}", self.title, self.status);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
        println!("Comment added to Task {}: {}", self.title, comment);
    }

    pub fn log_activity(&mut self, activity: String) {
        self.activities.push(activity);
        println!("Activity logged for Task {}: {}", self.title, activity);
    }
}
