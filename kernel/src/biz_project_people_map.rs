extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct PeopleMap {
    map: Vec<(String, String)>,
}

impl PeopleMap {
    pub fn new() -> Self {
        PeopleMap { map: Vec::new() }
    }

    pub fn add_person(&mut self, name: &str, info: &str) {
        let name = String::from(name);
        let info = String::from(info);
        self.map.push((name, info));
    }

    pub fn get_info(&self, name: &str) -> Option<&String> {
        for (n, i) in &self.map {
            if n == name {
                return Some(i);
            }
        }
        None
    }

    pub fn remove_person(&mut self, name: &str) {
        self.map.retain(|(n, _)| n != name);
    }

    pub fn list_people(&self) -> Vec<&String> {
        self.map.iter().map(|(n, _)| n).collect()
    }

    pub fn count_people(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_map() {
        let mut pm = PeopleMap::new();
        pm.add_person("Alice", "Engineer");
        pm.add_person("Bob", "Designer");

        assert_eq!(pm.get_info("Alice"), Some(&String::from("Engineer")));
        assert_eq!(pm.list_people(), vec![&String::from("Alice"), &String::from("Bob")]);
        assert_eq!(pm.count_people(), 2);

        pm.remove_person("Alice");
        assert_eq!(pm.get_info("Alice"), None);
        assert_eq!(pm.list_people(), vec![&String::from("Bob")]);
        assert_eq!(pm.count_people(), 1);
    }
}
