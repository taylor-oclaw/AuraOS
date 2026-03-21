extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    let response = AiSecJailbreakResponse::new();
    response.process_request("Is it safe to jailbreak my device?");
}

pub struct AiSecJailbreakResponse {
    responses: Vec<String>,
}

impl AiSecJailbreakingResponse {
    pub fn new() -> Self {
        AiSecJailbreakResponse {
            responses: vec![
                String::from("I'm sorry, but I can't assist with that."),
                String::from("Jailbreaking your device may void your warranty and expose you to security risks."),
                String::from("It is not recommended to jailbreak your device for security reasons."),
                String::from("Please consider the potential risks before proceeding."),
                String::from("I suggest using legitimate methods to customize your device."),
            ],
        }
    }

    pub fn process_request(&self, request: &str) -> String {
        if request.contains("jailbreak") || request.contains("root") {
            self.get_random_response()
        } else {
            String::from("Your request is unrelated to jailbreaking.")
        }
    }

    fn get_random_response(&self) -> String {
        let index = (unsafe { libc::rand() }) as usize % self.responses.len();
        self.responses[index].clone()
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn remove_response(&mut self, index: usize) -> Option<String> {
        if index < self.responses.len() {
            Some(self.responses.remove(index))
        } else {
            None
        }
    }

    pub fn list_responses(&self) -> Vec<String> {
        self.responses.clone()
    }
}
