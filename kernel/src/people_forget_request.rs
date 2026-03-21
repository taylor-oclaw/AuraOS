extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn people_forget_request_init() {
    // Initialization code if needed
}

pub extern "C" fn people_forget_request_exit() {
    // Cleanup code if needed
}

pub struct PeopleForgetRequest {
    requests: Vec<String>,
}

impl PeopleForgetRequest {
    pub fn new() -> Self {
        PeopleForgetRequest {
            requests: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: String) {
        self.requests.push(request);
    }

    pub fn remove_request(&mut self, index: usize) -> Option<String> {
        if index < self.requests.len() {
            Some(self.requests.remove(index))
        } else {
            None
        }
    }

    pub fn get_request(&self, index: usize) -> Option<&String> {
        self.requests.get(index)
    }

    pub fn list_requests(&self) -> &[String] {
        &self.requests
    }

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_forget_request() {
        let mut pfr = PeopleForgetRequest::new();

        assert_eq!(pfr.list_requests().len(), 0);

        pfr.add_request(String::from("request1"));
        pfr.add_request(String::from("request2"));

        assert_eq!(pfr.list_requests().len(), 2);
        assert_eq!(pfr.get_request(0), Some(&String::from("request1")));
        assert_eq!(pfr.get_request(1), Some(&String::from("request2")));

        let removed = pfr.remove_request(0);
        assert_eq!(removed, Some(String::from("request1")));
        assert_eq!(pfr.list_requests().len(), 1);

        pfr.clear_requests();
        assert_eq!(pfr.list_requests().len(), 0);
    }
}
