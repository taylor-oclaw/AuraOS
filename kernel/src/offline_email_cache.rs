extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod offline_email_cache {
    use core::cmp::Ordering;
    use alloc::collections::BTreeMap;

    pub struct EmailCache {
        cache: BTreeMap<String, String>,
    }

    impl EmailCache {
        pub fn new() -> Self {
            EmailCache {
                cache: BTreeMap::new(),
            }
        }

        pub fn add_email(&mut self, address: &str, content: &str) {
            let addr = String::from(address);
            let cont = String::from(content);
            self.cache.insert(addr, cont);
        }

        pub fn get_email(&self, address: &str) -> Option<&String> {
            self.cache.get(address)
        }

        pub fn remove_email(&mut self, address: &str) -> bool {
            self.cache.remove(address).is_some()
        }

        pub fn list_emails(&self) -> Vec<&String> {
            self.cache.keys().collect()
        }

        pub fn count_emails(&self) -> usize {
            self.cache.len()
        }
    }
}
