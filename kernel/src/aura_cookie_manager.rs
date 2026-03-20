extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AuraCookieManager {
    cookies: Vec<String>,
}

impl AuraCookieManager {
    pub fn new() -> Self {
        AuraCookieManager {
            cookies: Vec::new(),
        }
    }

    pub fn add_cookie(&mut self, cookie: String) {
        if !self.cookies.contains(&cookie) {
            self.cookies.push(cookie);
        }
    }

    pub fn remove_cookie(&mut self, cookie: &str) -> bool {
        let pos = self.cookies.iter().position(|c| c == cookie);
        if let Some(index) = pos {
            self.cookies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_cookie(&self, cookie: &str) -> bool {
        self.cookies.contains(&String::from(cookie))
    }

    pub fn list_cookies(&self) -> Vec<String> {
        self.cookies.clone()
    }

    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_manager() {
        let mut manager = AuraCookieManager::new();

        assert_eq!(manager.list_cookies(), Vec::<String>::new());

        manager.add_cookie(String::from("cookie1"));
        manager.add_cookie(String::from("cookie2"));

        assert_eq!(manager.has_cookie("cookie1"), true);
        assert_eq!(manager.has_cookie("cookie3"), false);

        assert_eq!(manager.list_cookies().len(), 2);

        assert_eq!(manager.remove_cookie("cookie1"), true);
        assert_eq!(manager.remove_cookie("cookie4"), false);

        assert_eq!(manager.list_cookies().len(), 1);

        manager.clear_cookies();
        assert_eq!(manager.list_cookies().len(), 0);
    }
}
