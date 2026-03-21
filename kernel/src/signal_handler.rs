extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SignalHandler {
    signals: Vec<String>,
}

impl SignalHandler {
    pub fn new() -> Self {
        SignalHandler {
            signals: Vec::new(),
        }
    }

    pub fn register_signal(&mut self, signal: String) {
        self.signals.push(signal);
    }

    pub fn unregister_signal(&mut self, signal: &str) {
        if let Some(index) = self.signals.iter().position(|s| s == signal) {
            self.signals.remove(index);
        }
    }

    pub fn is_registered(&self, signal: &str) -> bool {
        self.signals.contains(&String::from(signal))
    }

    pub fn list_signals(&self) -> Vec<String> {
        self.signals.clone()
    }

    pub fn handle_signal(&mut self, signal: &str) {
        if self.is_registered(signal) {
            // Simulate handling the signal
        } else {
        }
    }
}

