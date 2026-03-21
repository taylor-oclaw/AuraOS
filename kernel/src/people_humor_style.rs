extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod people_humor_style {
    use super::*;

    pub struct HumorStyle {
        styles: Vec<String>,
    }

    impl HumorStyle {
        pub fn new() -> Self {
            HumorStyle {
                styles: vec![
                    String::from("self-deprecating"),
                    String::from("dark"),
                    String::from("sarcasm"),
                    String::from("nerdy"),
                    String::from("wordplay"),
                ],
            }
        }

        pub fn add_style(&mut self, style: &str) {
            self.styles.push(String::from(style));
        }

        pub fn remove_style(&mut self, style: &str) -> bool {
            if let Some(index) = self.styles.iter().position(|s| s == style) {
                self.styles.remove(index);
                true
            } else {
                false
            }
        }

        pub fn get_styles(&self) -> Vec<String> {
            self.styles.clone()
        }

        pub fn has_style(&self, style: &str) -> bool {
            self.styles.contains(&String::from(style))
        }

        pub fn count_styles(&self) -> usize {
            self.styles.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humor_style() {
        let mut humor = people_humor_style::HumorStyle::new();
        assert_eq!(humor.count_styles(), 5);
        assert!(humor.has_style("self-deprecating"));
        assert!(!humor.has_style("improv"));

        humor.add_style("improv");
        assert_eq!(humor.count_styles(), 6);
        assert!(humor.has_style("improv"));

        assert!(humor.remove_style("dark"));
        assert_eq!(humor.count_styles(), 5);
        assert!(!humor.has_style("dark"));

        let styles = humor.get_styles();
        assert_eq!(styles.len(), 5);
        assert!(!styles.contains(&String::from("dark")));
    }
}
