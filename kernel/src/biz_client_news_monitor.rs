extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct NewsMonitor {
    headlines: Vec<String>,
    sources: Vec<String>,
    keywords: Vec<String>,
}

impl NewsMonitor {
    pub fn new() -> Self {
        NewsMonitor {
            headlines: Vec::new(),
            sources: Vec::new(),
            keywords: Vec::new(),
        }
    }

    pub fn add_headline(&mut self, headline: String) {
        self.headlines.push(headline);
    }

    pub fn add_source(&mut self, source: String) {
        self.sources.push(source);
    }

    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.push(keyword);
    }

    pub fn get_headlines(&self) -> &Vec<String> {
        &self.headlines
    }

    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    pub fn filter_headlines_by_keyword(&self, keyword: &str) -> Vec<&String> {
        self.headlines.iter().filter(|h| h.contains(keyword)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_monitor() {
        let mut monitor = NewsMonitor::new();
        monitor.add_headline(String::from("AI Advances in Healthcare"));
        monitor.add_source(String::from("TechCrunch"));
        monitor.add_keyword(String::from("AI"));

        assert_eq!(monitor.get_headlines().len(), 1);
        assert_eq!(monitor.get_sources().len(), 1);
        assert_eq!(monitor.get_keywords().len(), 1);

        let filtered = monitor.filter_headlines_by_keyword("AI");
        assert_eq!(filtered.len(), 1);
        assert_eq!(*filtered[0], "AI Advances in Healthcare");
    }
}
