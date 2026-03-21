extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut calculator = Calculator::new();
}

pub struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.history.push(String::from("info"));
        result
    }

    pub fn subtract(&mut self, a: i32, b: i32) -> i32 {
        let result = a - b;
        self.history.push(String::from("info"));
        result
    }

    pub fn multiply(&mut self, a: i32, b: i32) -> i32 {
        let result = a * b;
        self.history.push(String::from("info"));
        result
    }

    pub fn divide(&mut self, a: i32, b: i32) -> i32 {
        if b == 0 {
            self.history.push(String::from("Division by zero error"));
            return 0;
        }
        let result = a / b;
        self.history.push(String::from("info"));
        result
    }

    pub fn history(&self) -> &Vec<String> {
        &self.history
    }
}
