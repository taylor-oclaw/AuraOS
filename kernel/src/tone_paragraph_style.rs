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
            if !line.is_empty() {
                styled_text.push_str(&format!("{} {}\n", self.tone, line));
            } else {
                styled_text.push('\n');
            }
        }
        styled_text
    }

    pub fn list_styles(&self) -> Vec<String> {
        vec![self.tone.clone(), self.paragraph_style.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tps = ToneParagraphStyle::new("Formal", "Indented");
        assert_eq!(tps.get_tone(), "Formal");
        assert_eq!(tps.get_paragraph_style(), "Indented");
    }

    #[test]
    fn test_set_get_tone() {
        let mut tps = ToneParagraphStyle::new("", "");
        tps.set_tone("Casual");
        assert_eq!(tps.get_tone(), "Casual");
    }

    #[test]
    fn test_set_get_paragraph_style() {
        let mut tps = ToneParagraphStyle::new("", "");
        tps.set_paragraph_style("Bullet Points");
        assert_eq!(tps.get_paragraph_style(), "Bullet Points");
    }

    #[test]
    fn test_apply_to_text() {
        let tps = ToneParagraphStyle::new("Friendly", "Left Aligned");
        let text = "Hello\nWorld";
        let styled_text = tps.apply_to_text(text);
        assert_eq!(styled_text, "Friendly Hello\nFriendly World\n");
    }

    #[test]
    fn test_list_styles() {
        let tps = ToneParagraphStyle::new("Professional", "Center Aligned");
        let styles = tps.list_styles();
        assert_eq!(styles, vec![String::from("Professional"), String::from("Center Aligned")]);
    }
}
