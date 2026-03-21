extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_elderly_adapt_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_elderly_adapt_exit() {
    // Cleanup logic for the module
}

pub struct SpeechElderlyAdapt {
    commands: Vec<String>,
    responses: Vec<String>,
}

impl SpeechElderlyAdapt {
    pub fn new() -> Self {
        SpeechElderlyAdapt {
            commands: Vec::new(),
            responses: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn get_responses(&self) -> &Vec<String> {
        &self.responses
    }

    pub fn find_response(&self, command: &str) -> Option<&String> {
        self.commands.iter().position(|c| c == command).map(|index| &self.responses[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_elderly_adapt() {
        let mut adapter = SpeechElderlyAdapt::new();
        adapter.add_command(String::from("hello"));
        adapter.add_response(String::from("Hi there!"));

        assert_eq!(adapter.get_commands().len(), 1);
        assert_eq!(adapter.get_responses().len(), 1);

        let response = adapter.find_response("hello");
        assert_eq!(response, Some(&String::from("Hi there!")));
    }
}
