extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_sec_dos_query_complex_limit_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn ai_sec_dos_query_complex_limit_exit() {
    // Cleanup logic for the module
}

pub struct DosQueryComplexLimit {
    limit: u32,
    queries: Vec<String>,
}

impl DosQueryComplexLimit {
    pub fn new(limit: u32) -> Self {
        DosQueryComplexLimit {
            limit,
            queries: Vec::new(),
        }
    }

    pub fn add_query(&mut self, query: String) -> bool {
        if self.queries.len() < self.limit as usize {
            self.queries.push(query);
            true
        } else {
            false
        }
    }

    pub fn remove_query(&mut self, index: usize) -> Option<String> {
        if index < self.queries.len() {
            Some(self.queries.remove(index))
        } else {
            None
        }
    }

    pub fn get_queries(&self) -> &[String] {
        &self.queries
    }

    pub fn is_limit_reached(&self) -> bool {
        self.queries.len() >= self.limit as usize
    }

    pub fn clear_queries(&mut self) {
        self.queries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dos_query_complex_limit() {
        let mut limit = DosQueryComplexLimit::new(3);

        assert_eq!(limit.add_query(String::from("query1")), true);
        assert_eq!(limit.add_query(String::from("query2")), true);
        assert_eq!(limit.add_query(String::from("query3")), true);
        assert_eq!(limit.add_query(String::from("query4")), false);

        assert_eq!(limit.get_queries().len(), 3);
        assert_eq!(limit.is_limit_reached(), true);

        assert_eq!(limit.remove_query(1), Some(String::from("query2")));
        assert_eq!(limit.get_queries().len(), 2);
        assert_eq!(limit.is_limit_reached(), false);

        limit.clear_queries();
        assert_eq!(limit.get_queries().len(), 0);
    }
}
