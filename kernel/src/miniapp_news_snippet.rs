extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod kernel_module {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct MiniAppNewsSnippet {
        title: String,
        content: String,
        author: String,
        date: String,
        tags: Vec<String>,
    }

    impl MiniAppNewsSnippet {
        pub fn new(title: &str, content: &str, author: &str, date: &str) -> Self {
            MiniAppNewsSnippet {
                title: String::from(title),
                content: String::from(content),
                author: String::from(author),
                date: String::from(date),
                tags: Vec::new(),
            }
        }

        pub fn add_tag(&mut self, tag: &str) {
            self.tags.push(String::from(tag));
        }

        pub fn get_title(&self) -> &str {
            &self.title
        }

        pub fn get_content(&self) -> &str {
            &self.content
        }

        pub fn get_author(&self) -> &str {
            &self.author
        }

        pub fn get_date(&self) -> &str {
            &self.date
        }

        pub fn get_tags(&self) -> &[String] {
            &self.tags
        }
    }
}

#[cfg(test)]
mod tests {
    use super::kernel_module::MiniAppNewsSnippet;

    #[test]
    fn test_miniapp_news_snippet() {
        let mut snippet = MiniAppNewsSnippet::new(
            "AI Takes Over the World",
            "In a surprising turn of events, AI has become the dominant force in global affairs.",
            "John Doe",
            "2023-10-01",
        ;

        assert_eq!(snippet.get_title(), "AI Takes Over the World");
        assert_eq!(snippet.get_content(), "In a surprising turn of events, AI has become the dominant force in global affairs.");
        assert_eq!(snippet.get_author(), "John Doe");
        assert_eq!(snippet.get_date(), "2023-10-01");
        assert_eq!(snippet.get_tags().len(), 0);

        snippet.add_tag("AI");
        snippet.add_tag("Global Affairs");

        assert_eq!(snippet.get_tags().len(), 2);
        assert_eq!(snippet.get_tags()[0], "AI");
        assert_eq!(snippet.get_tags()[1], "Global Affairs");
    }
}
