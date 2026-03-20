extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod bidi_text {
    use super::*;

    pub struct BidiText {
        text: String,
        direction: Direction,
    }

    impl BidiText {
        pub fn new(text: &str) -> Self {
            BidiText {
                text: String::from(text),
                direction: Direction::LTR,
            }
        }

        pub fn set_text(&mut self, text: &str) {
            self.text = String::from(text);
        }

        pub fn get_text(&self) -> &str {
            &self.text
        }

        pub fn set_direction(&mut self, direction: Direction) {
            self.direction = direction;
        }

        pub fn get_direction(&self) -> Direction {
            self.direction
        }

        pub fn reverse_text(&mut self) {
            let reversed: String = self.text.chars().rev().collect();
            self.text = reversed;
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Direction {
        LTR,
        RTL,
    }
}

#[cfg(test)]
mod tests {
    use super::bidi_text::*;

    #[test]
    fn test_new() {
        let text = BidiText::new("Hello");
        assert_eq!(text.get_text(), "Hello");
        assert_eq!(text.get_direction(), Direction::LTR);
    }

    #[test]
    fn test_set_text() {
        let mut text = BidiText::new("Hello");
        text.set_text("World");
        assert_eq!(text.get_text(), "World");
    }

    #[test]
    fn test_get_text() {
        let text = BidiText::new("Hello");
        assert_eq!(text.get_text(), "Hello");
    }

    #[test]
    fn test_set_direction() {
        let mut text = BidiText::new("Hello");
        text.set_direction(Direction::RTL);
        assert_eq!(text.get_direction(), Direction::RTL);
    }

    #[test]
    fn test_get_direction() {
        let text = BidiText::new("Hello");
        assert_eq!(text.get_direction(), Direction::LTR);
    }

    #[test]
    fn test_reverse_text() {
        let mut text = BidiText::new("Hello");
        text.reverse_text();
        assert_eq!(text.get_text(), "olleH");
    }
}
