extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut calculator = Calculator::new();
    println!("2 + 3 = {}", calculator.add(2, 3));
    println!("5 - 1 = {}", calculator.subtract(5, 1));
    println!("4 * 6 = {}", calculator.multiply(4, 6));
    println!("8 / 2 = {}", calculator.divide(8, 2));
    println!("History: {:?}", calculator.history());
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
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }

    pub fn subtract(&mut self, a: i32, b: i32) -> i32 {
        let result = a - b;
        self.history.push(format!("{} - {} = {}", a, b, result));
        result
    }

    pub fn multiply(&mut self, a: i32, b: i32) -> i32 {
        let result = a * b;
        self.history.push(format!("{} * {} = {}", a, b, result));
        result
    }

    pub fn divide(&mut self, a: i32, b: i32) -> i32 {
        if b == 0 {
            self.history.push(String::from("Division by zero error"));
            return 0;
        }
        let result = a / b;
        self.history.push(format!("{} / {} = {}", a, b, result));
        result
    }

    pub fn history(&self) -> &Vec<String> {
        &self.history
    }
}
