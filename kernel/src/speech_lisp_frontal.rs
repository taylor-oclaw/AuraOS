extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_lisp_frontal_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_lisp_frontal_exit() {
    // Cleanup logic for the module
}

pub struct SpeechLispFrontal {
    commands: Vec<String>,
    history: Vec<String>,
    max_history_size: usize,
}

impl SpeechLispFrontal {
    pub fn new(max_history_size: usize) -> Self {
        SpeechLispFrontal {
            commands: Vec::new(),
            history: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_command(&mut self, command: String) {
        if self.commands.len() >= self.max_history_size {
            self.commands.remove(0);
        }
        self.commands.push(command.clone());
        self.history.push(command);
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn execute_command(&self, command: &str) -> String {
        // Placeholder for command execution logic
        format!("Executing: {}", command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_lisp_frontal() {
        let mut slf = SpeechLispFrontal::new(3);
        slf.add_command(String::from("echo hello"));
        slf.add_command(String::from("ls -l"));
        slf.add_command(String::from("cat file.txt"));

        assert_eq!(slf.get_commands().len(), 3);
        assert_eq!(slf.get_history().len(), 3);

        slf.add_command(String::from("pwd"));
        assert_eq!(slf.get_commands().len(), 3);
        assert_eq!(slf.get_commands()[0], "ls -l");
        assert_eq!(slf.get_commands()[1], "cat file.txt");
        assert_eq!(slf.get_commands()[2], "pwd");

        slf.clear_history();
        assert_eq!(slf.get_history().len(), 0);
    }
}
