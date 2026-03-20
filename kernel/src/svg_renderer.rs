extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SvgRenderer {
    width: u32,
    height: u32,
    elements: Vec<String>,
}

impl SvgRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        SvgRenderer {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn add_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: &str) {
        let rect = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
            x, y, width, height, color
        );
        self.elements.push(rect);
    }

    pub fn add_circle(&mut self, cx: u32, cy: u32, r: u32, color: &str) {
        let circle = format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />",
            cx, cy, r, color
        );
        self.elements.push(circle);
    }

    pub fn add_text(&mut self, x: u32, y: u32, text: &str, font_size: u32, color: &str) {
        let text_element = format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>",
            x, y, font_size, color, text
        );
        self.elements.push(text_element);
    }

    pub fn render(&self) -> String {
        let mut svg_content = format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        );

        for element in &self.elements {
            svg_content.push_str(element);
        }

        svg_content.push_str("</svg>");
        svg_content
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}
