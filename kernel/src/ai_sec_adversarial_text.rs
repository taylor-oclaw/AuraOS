extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

pub struct AIAdversarialText {
    text: String,
    transformations: Vec<String>,
}

impl AIAdversarialText {
    pub fn new(text: &str) -> Self {
        AIAdversarialText {
            text: String::from(text),
            transformations: Vec::new(),
        }
    }

    pub fn add_transformation(&mut self, transformation: &str) {
        self.transformations.push(String::from(transformation));
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn apply_transformation(&mut self, index: usize) -> Result<&str, &'static str> {
        if let Some(transformation) = self.transformations.get(index) {
            self.text.push_str(transformation);
            Ok(&self.text)
        } else {
            Err("Transformation index out of bounds")
        }
    }

    pub fn remove_transformation(&mut self, index: usize) -> Result<String, &'static str> {
        if let Some(transformation) = self.transformations.remove(index) {
            Ok(transformation)
        } else {
            Err("Transformation index out of bounds")
        }
    }

    pub fn list_transformations(&self) -> &Vec<String> {
        &self.transformations
    }
}
