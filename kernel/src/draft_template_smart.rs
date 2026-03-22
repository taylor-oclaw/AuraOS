extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DraftTemplateSmart {
    template_name: String,
    template_content: Vec<u8>,
}

impl DraftTemplateSmart {
    pub fn new(template_name: &str, template_content: &[u8]) -> Self {
        DraftTemplateSmart {
            template_name: String::from(template_name),
            template_content: Vec::from(template_content),
        }
    }

    pub fn get_template_name(&self) -> &str {
        self.template_name.as_str()
    }

    pub fn set_template_name(&mut self, new_name: &str) {
        self.template_name = String::from(new_name);
    }

    pub fn get_template_content(&self) -> &[u8] {
        &self.template_content
    }

    pub fn update_template_content(&mut self, new_content: &[u8]) {
        self.template_content.clear();
        self.template_content.extend_from_slice(new_content);
    }

    pub fn render_template(&self, data: &[u8]) -> Vec<u8> {
        let mut rendered_template = Vec::new();

        for (i, byte) in self.template_content.iter().enumerate() {
            if *byte == b'{' && i < self.template_content.len() - 1 {
                let end_brace_index = self.template_content.iter().position(|&b| b == b'}').unwrap();
                let placeholder = &self.template_content[i + 1..end_brace_index];
                let value = data.get(placeholder.len()).and_then(|v| v.get(..placeholder.len())).unwrap_or(b"");
                rendered_template.extend_from_slice(value);
            } else {
                rendered_template.push(*byte);
            }
        }

        rendered_template
    }
}