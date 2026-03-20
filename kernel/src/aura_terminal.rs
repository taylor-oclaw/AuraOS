extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_ffi_example() {
    // Example FFI function to demonstrate interaction with C code
}

struct AuraTerminal {
    buffer: Vec<String>,
    history: Vec<String>,
    max_history_size: usize,
}

impl AuraTerminal {
    pub fn new(max_history_size: usize) -> Self {
        AuraTerminal {
            buffer: Vec::new(),
            history: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_to_buffer(&mut self, line: String) {
        self.buffer.push(line);
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn get_buffer_content(&self) -> Vec<String> {
        self.buffer.clone()
    }

    pub fn add_to_history(&mut self, command: String) {
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
        }
        self.history.push(command);
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_terminal() {
        let mut terminal = AuraTerminal::new(3);

        terminal.add_to_buffer(String::from("echo Hello"));
        assert_eq!(terminal.get_buffer_content(), vec![String::from("echo Hello")]);

        terminal.clear_buffer();
        assert_eq!(terminal.get_buffer_content().len(), 0);

        terminal.add_to_history(String::from("ls -l"));
        terminal.add_to_history(String::from("cd /home"));
        terminal.add_to_history(String::from("pwd"));
        terminal.add_to_history(String::from("whoami"));

        let history = terminal.get_history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], String::from("cd /home"));
        assert_eq!(history[1], String::from("pwd"));
        assert_eq!(history[2], String::from("whoami"));
    }
}
