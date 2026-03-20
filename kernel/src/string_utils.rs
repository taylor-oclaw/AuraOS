extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

pub fn pad_left(s: &str, width: usize, ch: char) -> String {
    if s.len() >= width {
        return String::from(s);
    }
    let padding: String = core::iter::repeat(ch).take(width - s.len()).collect();
    let mut result = padding;
    result.push_str(s);
    result
}

pub fn pad_right(s: &str, width: usize, ch: char) -> String {
    let mut result = String::from(s);
    while result.len() < width {
        result.push(ch);
    }
    result
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        String::from(s)
    } else {
        let mut r = String::from(&s[..max_len.min(s.len())]);
        r.push_str("...");
        r
    }
}

pub fn to_uppercase_char(c: char) -> char {
    if c >= 97 as char && c <= 122 as char {
        (c as u8 - 32) as char
    } else {
        c
    }
}

pub fn to_lowercase_char(c: char) -> char {
    if c >= 65 as char && c <= 90 as char {
        (c as u8 + 32) as char
    } else {
        c
    }
}

pub fn format_size_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        String::from("error")
    } else if bytes < 1024 * 1024 {
        String::from("error")
    } else if bytes < 1024 * 1024 * 1024 {
        String::from("error")
    } else {
        String::from("error")
    }
}
