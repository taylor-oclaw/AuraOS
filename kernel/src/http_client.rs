#![no_std]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

struct HttpClient {
    url: String,
    headers: Vec<(String, String)>,
}

impl HttpClient {
    pub fn new(url: &str) -> Self {
        HttpClient {
            url: url.to_string(),
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()));
    }

    pub fn get(&self) -> Result<String, &'static str> {
        // Simulate a GET request
        if self.url.contains("example.com") {
            Ok(String::from("Response from example.com"))
        } else {
            Err("Invalid URL")
        }
    }

    pub fn post(&mut self, body: &str) -> Result<String, &'static str> {
        // Simulate a POST request
        if self.url.contains("example.com") {
            Ok(String::from("Response from example.com"))
        } else {
            Err("Invalid URL")
        }
    }

    pub fn delete(&self) -> Result<String, &'static str> {
        // Simulate a DELETE request
        if self.url.contains("example.com") {
            Ok(String::from("Response from example.com"))
        } else {
            Err("Invalid URL")
        }
    }
}