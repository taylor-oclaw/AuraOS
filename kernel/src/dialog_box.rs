extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DialogBox {
    title: String,
    content: String,
    buttons: Vec<String>,
    selected_button: usize,
}

impl DialogBox {
    pub fn new(title: &str, content: &str) -> Self {
        DialogBox {
            title: String::from(title),
            content: String::from(content),
            buttons: Vec::new(),
            selected_button: 0,
        }
    }

    pub fn add_button(&mut self, button_text: &str) {
        self.buttons.push(String::from(button_text));
        if self.selected_button >= self.buttons.len() {
            self.selected_button = self.buttons.len() - 1;
        }
    }

    pub fn select_next_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = (self.selected_button + 1) % self.buttons.len();
        }
    }

    pub fn select_previous_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = if self.selected_button == 0 {
                self.buttons.len() - 1
            } else {
                self.selected_button - 1
            };
        }
    }

    pub fn get_selected_button_text(&self) -> Option<&str> {
        self.buttons.get(self.selected_button).map(|s| s.as_str())
    }

    pub fn display(&self) -> String {
        let mut display = String::from("info");
        for (i, button) in self.buttons.iter().enumerate() {
            if i == self.selected_button {
                display.push_str(&String::from("info"));
            } else {
                display.push_str(&String::from("info"));
            }
        }
        display
    }
}
