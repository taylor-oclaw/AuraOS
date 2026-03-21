extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceContextStack {
    stack: Vec<String>,
}

impl VoiceContextStack {
    pub fn new() -> Self {
        VoiceContextStack {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, context: String) {
        self.stack.push(context);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&String> {
        self.stack.last()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}
