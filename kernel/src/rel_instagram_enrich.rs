extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct InstagramEnricher {
    posts: Vec<String>,
    users: Vec<String>,
}

impl InstagramEnricher {
    pub fn new() -> Self {
        InstagramEnricher {
            posts: Vec::new(),
            users: Vec::new(),
        }
    }

    pub fn add_post(&mut self, post: String) {
        self.posts.push(post);
    }

    pub fn add_user(&mut self, user: String) {
        self.users.push(user);
    }

    pub fn get_posts_count(&self) -> usize {
        self.posts.len()
    }

    pub fn get_users_count(&self) -> usize {
        self.users.len()
    }

    pub fn list_posts(&self) -> Vec<String> {
        self.posts.clone()
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instagram_enricher() {
        let mut enricher = InstagramEnricher::new();
        assert_eq!(enricher.get_posts_count(), 0);
        assert_eq!(enricher.get_users_count(), 0);

        enricher.add_post(String::from("Hello, world!"));
        enricher.add_user(String::from("Alice"));

        assert_eq!(enricher.get_posts_count(), 1);
        assert_eq!(enricher.get_users_count(), 1);

        let posts = enricher.list_posts();
        let users = enricher.list_users();

        assert_eq!(posts, vec![String::from("Hello, world!")]);
        assert_eq!(users, vec![String::from("Alice")]);
    }
}
