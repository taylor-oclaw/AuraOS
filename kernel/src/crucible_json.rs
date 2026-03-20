extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_json_init() {
    // Initialization code for the module
}

pub extern "C" fn crucible_json_exit() {
    // Cleanup code for the module
}

pub struct JsonValue {
    value: String,
}

impl JsonValue {
    pub fn new(value: &str) -> Self {
        JsonValue {
            value: String::from(value),
        }
    }

    pub fn is_null(&self) -> bool {
        self.value == "null"
    }

    pub fn is_string(&self) -> bool {
        self.value.starts_with('"') && self.value.ends_with('"')
    }

    pub fn as_str(&self) -> &str {
        if self.is_string() {
            &self.value[1..self.value.len() - 1]
        } else {
            ""
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    pub fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        for _ in 0..indent {
            result.push_str("  ");
        }
        result.push_str(&self.value);
        result.push('\n');
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_new() {
        let json = JsonValue::new("null");
        assert_eq!(json.value, "null");
    }

    #[test]
    fn test_json_value_is_null() {
        let json = JsonValue::new("null");
        assert!(json.is_null());
    }

    #[test]
    fn test_json_value_is_string() {
        let json = JsonValue::new("\"hello\"");
        assert!(json.is_string());
    }

    #[test]
    fn test_json_value_as_str() {
        let json = JsonValue::new("\"world\"");
        assert_eq!(json.as_str(), "world");
    }

    #[test]
    fn test_json_value_to_vec() {
        let json = JsonValue::new("true");
        let vec = json.to_vec();
        assert_eq!(vec, b"true"[..]);
    }

    #[test]
    fn test_json_value_pretty_print() {
        let json = JsonValue::new("\"pretty\"");
        let result = json.pretty_print(2);
        assert_eq!(result, "  \"pretty\"\n");
    }
}
