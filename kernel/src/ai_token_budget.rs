extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AiTokenBudget {
    budget: u32,
    used: u32,
}

impl AiTokenBudget {
    pub fn new(budget: u32) -> Self {
        AiTokenBudget { budget, used: 0 }
    }

    pub fn get_budget(&self) -> u32 {
        self.budget
    }

    pub fn get_used(&self) -> u32 {
        self.used
    }

    pub fn get_remaining(&self) -> u32 {
        self.budget - self.used
    }

    pub fn allocate_tokens(&mut self, tokens: u32) -> Result<(), String> {
        if tokens > self.get_remaining() {
            Err(String::from("Insufficient tokens"))
        } else {
            self.used += tokens;
            Ok(())
        }
    }

    pub fn release_tokens(&mut self, tokens: u32) -> Result<(), String> {
        if tokens > self.used {
            Err(String::from("Cannot release more tokens than used"))
        } else {
            self.used -= tokens;
            Ok(())
        }
    }
}
