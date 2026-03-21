extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailAttachmentPreview {
    file_name: String,
    content_type: String,
    preview_data: Vec<u8>,
    is_image: bool,
    size_in_bytes: usize,
}

impl EmailAttachmentPreview {
    pub fn new(file_name: &str, content_type: &str, preview_data: Vec<u8>, is_image: bool, size_in_bytes: usize) -> Self {
        EmailAttachmentPreview {
            file_name: String::from(file_name),
            content_type: String::from(content_type),
            preview_data,
            is_image,
            size_in_bytes,
        }
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_content_type(&self) -> &str {
        &self.content_type
    }

    pub fn get_preview_data(&self) -> &[u8] {
        &self.preview_data
    }

    pub fn is_image(&self) -> bool {
        self.is_image
    }

    pub fn get_size_in_bytes(&self) -> usize {
        self.size_in_bytes
    }
}
