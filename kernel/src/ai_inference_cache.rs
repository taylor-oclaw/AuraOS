extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod ai_inference_cache {
    use super::*;

    pub struct InferenceCache {
        cache: Vec<(String, Vec<u8>)>,
    }

    impl InferenceCache {
        pub fn new() -> Self {
            InferenceCache { cache: Vec::new() }
        }

        pub fn add(&mut self, key: String, value: Vec<u8>) {
            self.cache.push((key, value));
        }

        pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
            self.cache.iter().find(|&&(ref k, _)| k == key).map(|(_, v)| v)
        }

        pub fn remove(&mut self, key: &str) {
            self.cache.retain(|(k, _)| k != key);
        }

        pub fn contains_key(&self, key: &str) -> bool {
            self.cache.iter().any(|(k, _)| k == key)
        }

        pub fn clear(&mut self) {
            self.cache.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_inference_cache::*;

    #[test]
    fn test_add_get_remove() {
        let mut cache = InferenceCache::new();
        let key = String::from("model1");
        let value: Vec<u8> = vec![0, 1, 2];

        cache.add(key.clone(), value.clone());
        assert_eq!(cache.get(&key), Some(&value));

        cache.remove(&key);
        assert_eq!(cache.get(&key), None);
    }

    #[test]
    fn test_contains_key() {
        let mut cache = InferenceCache::new();
        let key = String::from("model2");
        let value: Vec<u8> = vec![3, 4, 5];

        assert!(!cache.contains_key(&key));
        cache.add(key.clone(), value);
        assert!(cache.contains_key(&key));
    }

    #[test]
    fn test_clear() {
        let mut cache = InferenceCache::new();
        let key1 = String::from("model3");
        let key2 = String::from("model4");
        let value1: Vec<u8> = vec![6, 7, 8];
        let value2: Vec<u8> = vec![9, 10, 11];

        cache.add(key1.clone(), value1);
        cache.add(key2.clone(), value2);
        assert!(cache.contains_key(&key1));
        assert!(cache.contains_key(&key2));

        cache.clear();
        assert!(!cache.contains_key(&key1));
        assert!(!cache.contains_key(&key2));
    }
}
