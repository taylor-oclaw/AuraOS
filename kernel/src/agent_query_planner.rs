extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AgentQueryPlanner {
    queries: Vec<String>,
    responses: Vec<String>,
}

impl AgentQueryPlanner {
    pub fn new() -> Self {
        AgentQueryPlanner {
            queries: Vec::new(),
            responses: Vec::new(),
        }
    }

    pub fn add_query(&mut self, query: String) {
        self.queries.push(query);
    }

    pub fn get_queries(&self) -> &Vec<String> {
        &self.queries
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn get_responses(&self) -> &Vec<String> {
        &self.responses
    }

    pub fn clear_queries(&mut self) {
        self.queries.clear();
    }

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_query_planner() {
        let mut planner = AgentQueryPlanner::new();

        assert!(planner.get_queries().is_empty());
        assert!(planner.get_responses().is_empty());

        planner.add_query(String::from("What is the weather?"));
        planner.add_response(String::from("Sunny"));

        assert_eq!(planner.get_queries().len(), 1);
        assert_eq!(planner.get_responses().len(), 1);

        planner.clear_queries();
        planner.clear_responses();

        assert!(planner.get_queries().is_empty());
        assert!(planner.get_responses().is_empty());
    }
}
