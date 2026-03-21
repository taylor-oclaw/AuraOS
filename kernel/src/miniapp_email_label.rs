extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct EmailLabel {
    labels: Vec<String>,
}

impl EmailLabel {
    pub fn new() -> Self {
        EmailLabel {
            labels: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: &str) {
        if !self.labels.contains(&label.to_string()) {
            self.labels.push(label.to_string());
        }
    }

    pub fn remove_label(&mut self, label: &str) {
        self.labels.retain(|l| l != label);
    }

    pub fn has_label(&self, label: &str) -> bool {
        self.labels.contains(&label.to_string())
    }

    pub fn list_labels(&self) -> Vec<String> {
        self.labels.clone()
    }

    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_label() {
        let mut email_label = EmailLabel::new();

        assert_eq!(email_label.list_labels(), Vec::<String>::new());

        email_label.add_label("Work");
        email_label.add_label("Personal");

        assert_eq!(email_label.has_label("Work"), true);
        assert_eq!(email_label.has_label("Personal"), true);
        assert_eq!(email_label.has_label("Other"), false);

        let labels = email_label.list_labels();
        assert_eq!(labels.len(), 2);
        assert!(labels.contains(&"Work".to_string()));
        assert!(labels.contains(&"Personal".to_string()));

        email_label.remove_label("Work");
        assert_eq!(email_label.has_label("Work"), false);

        email_label.clear_labels();
        assert_eq!(email_label.list_labels(), Vec::<String>::new());
    }
}
