extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneContextSwitch {
    context_stack: Vec<String>,
}

impl ToneContextSwitch {
    pub fn new() -> Self {
        ToneContextSwitch {
            context_stack: Vec::new(),
        }
    }

    pub fn push_context(&mut self, context: String) {
        self.context_stack.push(context);
    }

    pub fn pop_context(&mut self) -> Option<String> {
        self.context_stack.pop()
    }

    pub fn current_context(&self) -> Option<&String> {
        self.context_stack.last()
    }

    pub fn clear_contexts(&mut self) {
        self.context_stack.clear();
    }

    pub fn context_count(&self) -> usize {
        self.context_stack.len()
    }
}
