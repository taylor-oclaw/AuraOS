extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod tone_emoji_style {
    use super::*;

    pub struct ToneEmojiStyle {
        styles: Vec<String>,
    }

    impl ToneEmojiStyle {
        pub fn new() -> Self {
            let mut styles = Vec::new();
            styles.push(String::from("neutral"));
            styles.push(String::from("happy"));
            styles.push(String::from("sad"));
            styles.push(String::from("angry"));
            styles.push(String::from("surprised"));
            ToneEmojiStyle { styles }
        }

        pub fn add_style(&mut self, style: String) {
            self.styles.push(style);
        }

        pub fn remove_style(&mut self, index: usize) -> Option<String> {
            if index < self.styles.len() {
                Some(self.styles.remove(index))
            } else {
                None
            }
        }

        pub fn get_style(&self, index: usize) -> Option<&String> {
            self.styles.get(index)
        }

        pub fn list_styles(&self) -> &Vec<String> {
            &self.styles
        }

        pub fn count_styles(&self) -> usize {
            self.styles.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tone_emoji_style::*;

    #[test]
    fn test_tone_emoji_style() {
        let mut style = ToneEmojiStyle::new();
        assert_eq!(style.count_styles(), 5);

        style.add_style(String::from("excited"));
        assert_eq!(style.count_styles(), 6);
        assert_eq!(style.get_style(5), Some(&String::from("excited")));

        assert_eq!(style.remove_style(0), Some(String::from("neutral")));
        assert_eq!(style.count_styles(), 5);

        assert_eq!(style.list_styles(), &vec![
            String::from("happy"),
            String::from("sad"),
            String::from("angry"),
            String::from("surprised"),
            String::from("excited")
        ];
    }
}
