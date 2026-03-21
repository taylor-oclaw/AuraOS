extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let detector = CommandDetector::new();
    detector.process_command("echo Hello, World!");
    detector.process_command("ls -l");
    detector.process_command("cat /etc/passwd");
    detector.process_command("rm -rf /");
    detector.process_command("uname -a");
}

pub struct CommandDetector {
    history: Vec<String>,
    suspicious_commands: Vec<String>,
}

impl CommandDetector {
    pub fn new() -> Self {
        CommandDetector {
            history: Vec::new(),
            suspicious_commands: vec![
                String::from("rm"),
                String::from("dd"),
                String::from("mkfs"),
                String::from("chroot"),
                String::from("mount"),
            ],
        }
    }

    pub fn process_command(&mut self, command: &str) {
        let cmd = String::from(command);
        self.history.push(cmd.clone());
        if self.is_suspicious(&cmd) {
            self.handle_suspicious_command(&cmd);
        } else {
        }
    }

    pub fn is_suspicious(&self, command: &str) -> bool {
        for suspicious in &self.suspicious_commands {
            if command.contains(suspicious) {
                return true;
            }
        }
        false
    }

    pub fn handle_suspicious_command(&self, command: &str) {
        // Add logic to log the suspicious command or take other actions
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
