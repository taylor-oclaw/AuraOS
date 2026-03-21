extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneParagraphStyle {
    tone: String,
    paragraph_style: String,
}

impl ToneParagraphStyle {
    pub fn new(tone: &str, paragraph_style: &str) -> Self {
        ToneParagraphStyle {
            tone: String::from(tone),
            paragraph_style: String::from(paragraph_style),
        }
    }

    pub fn set_tone(&mut self, tone: &str) {
        self.tone = String::from(tone);
    }

    pub fn get_tone(&self) -> &String {
        &self.tone
    }

    pub fn set_paragraph_style(&mut self, paragraph_style: &str) {
        self.paragraph_style = String::from(paragraph_style);
    }

    pub fn get_paragraph_style(&self) -> &String {
        &self.paragraph_style
    }

    pub fn apply_to_text(&self, text: &str) -> String {
        let mut styled_text = String::new();
        for line in text.lines() {
            styled_text.push_str(&format!("{} {}\n", self.tone, line));
        }
        styled_text.push_str(&format!("\n{}", self.paragraph_style));
        styled_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tps = ToneParagraphStyle::new("Serious", "Indented");
        assert_eq!(tps.get_tone(), "Serious");
        assert_eq!(tps.get_paragraph_style(), "Indented");
    }

    #[test]
    fn test_set_get_tone() {
        let mut tps = ToneParagraphStyle::new("", "");
        tps.set_tone("Friendly");
        assert_eq!(tps.get_tone(), "Friendly");
    }

    #[test]
    fn test_set_get_paragraph_style() {
        let mut tps = ToneParagraphStyle::new("", "");
        tps.set_paragraph_style("Centered");
        assert_eq!(tps.get_paragraph_style(), "Centered");
    }

    #[test]
    fn test_apply_to_text() {
        let tps = ToneParagraphStyle::new("Excited", "Bold");
        let text = "Hello\nWorld";
        let styled_text = tps.apply_to_text(text);
        assert_eq!(styled_text, "Excited Hello\nExcited World\n\nBold");
    }
}
