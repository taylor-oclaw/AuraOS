extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MiniAppLinkPreview {
    links: Vec<String>,
}

impl MiniAppLinkPreview {
    pub fn new() -> Self {
        MiniAppLinkPreview { links: Vec::new() }
    }

    pub fn add_link(&mut self, link: String) {
        self.links.push(link);
    }

    pub fn remove_link(&mut self, index: usize) -> Option<String> {
        if index < self.links.len() {
            Some(self.links.remove(index))
        } else {
            None
        }
    }

    pub fn get_link(&self, index: usize) -> Option<&String> {
        self.links.get(index)
    }

    pub fn list_links(&self) -> &[String] {
        &self.links
    }

    pub fn clear_links(&mut self) {
        self.links.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miniapp_link_preview() {
        let mut preview = MiniAppLinkPreview::new();
        assert_eq!(preview.list_links().len(), 0);

        preview.add_link(String::from("https://example.com"));
        assert_eq!(preview.list_links().len(), 1);
        assert_eq!(preview.get_link(0), Some(&String::from("https://example.com")));

        let removed = preview.remove_link(0);
        assert_eq!(removed, Some(String::from("https://example.com")));
        assert_eq!(preview.list_links().len(), 0);

        preview.add_link(String::from("https://example1.com"));
        preview.add_link(String::from("https://example2.com"));
        preview.clear_links();
        assert_eq!(preview.list_links().len(), 0);
    }
}
