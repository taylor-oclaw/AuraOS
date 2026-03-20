extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ParallelDecode {
    data: Vec<u8>,
    decoded_data: Vec<String>,
}

impl ParallelDecode {
    pub fn new(data: Vec<u8>) -> Self {
        ParallelDecode {
            data,
            decoded_data: Vec::new(),
        }
    }

    pub fn decode(&mut self) {
        for byte in &self.data {
            let char_str = String::from_utf8(vec![*byte]).unwrap_or_else(|_| String::from("�"));
            self.decoded_data.push(char_str);
        }
    }

    pub fn get_decoded_data(&self) -> &[String] {
        &self.decoded_data
    }

    pub fn clear_decoded_data(&mut self) {
        self.decoded_data.clear();
    }

    pub fn append_data(&mut self, additional_data: Vec<u8>) {
        self.data.extend(additional_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_decode() {
        let data = vec![72, 101, 108, 108, 111]; // "Hello" in ASCII
        let mut decoder = ParallelDecode::new(data);
        decoder.decode();
        assert_eq!(decoder.get_decoded_data(), &["H", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_append_data() {
        let data = vec![72, 101, 108]; // "Hel" in ASCII
        let mut decoder = ParallelDecode::new(data);
        decoder.append_data(vec![108, 111]); // "lo" in ASCII
        decoder.decode();
        assert_eq!(decoder.get_decoded_data(), &["H", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_clear_decoded_data() {
        let data = vec![72, 101, 108]; // "Hel" in ASCII
        let mut decoder = ParallelDecode::new(data);
        decoder.decode();
        decoder.clear_decoded_data();
        assert!(decoder.get_decoded_data().is_empty());
    }
}
