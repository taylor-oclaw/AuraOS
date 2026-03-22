#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct JacobiDecode {
    data: Vec<u8>,
}

impl JacobiDecode {
    pub fn new(data: Vec<u8>) -> Self {
        JacobiDecode { data }
    }

    pub fn decode(&self) -> String {
        let mut decoded = String::new();
        for byte in &self.data {
            decoded.push(*byte as char);
        }
        decoded
    }

    pub fn encode(&self, input: &str) -> Vec<u8> {
        input.chars().map(|c| c as u8).collect()
    }

    pub fn reverse(&self) -> Vec<u8> {
        self.data.iter().rev().cloned().collect()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = vec![74, 97, 99, 105, 111, 110];
        let jacobi = JacobiDecode::new(data);
        assert_eq!(jacobi.data, vec![74, 97, 99, 105, 111, 110]);
    }

    #[test]
    fn test_decode() {
        let data = vec![74, 97, 99, 105, 111, 110];
        let jacobi = JacobiDecode::new(data);
        assert_eq!(jacobi.decode(), "Jacino");
    }

    #[test]
    fn test_encode() {
        let jacobi = JacobiDecode::new(vec![]);
        assert_eq!(jacobi.encode("Jacino"), vec![74, 97, 99, 105, 111, 110]);
    }

    #[test]
    fn test_reverse() {
        let data = vec![74, 97, 99, 105, 111, 110];
        let jacobi = JacobiDecode::new(data);
        assert_eq!(jacobi.reverse(), vec![110, 111, 111, 105, 97, 74]);
    }

    #[test]
    fn test_length() {
        let data = vec![74, 97, 99, 105, 111, 110];
        let jacobi = JacobiDecode::new(data);
        assert_eq!(jacobi.length(), 6);
    }
}