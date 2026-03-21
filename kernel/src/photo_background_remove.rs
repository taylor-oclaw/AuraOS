extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoBackgroundRemove {
    image_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl PhotoBackgroundRemove {
    pub fn new(image_data: Vec<u8>, width: usize, height: usize) -> Self {
        PhotoBackgroundRemove {
            image_data,
            width,
            height,
        }
    }

    pub fn get_image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    pub fn set_image_data(&mut self, image_data: Vec<u8>) {
        self.image_data = image_data;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn remove_background(&mut self) -> Result<(), String> {
        if self.image_data.len() != self.width * self.height * 3 {
            return Err(String::from("Invalid image data size"));
        }

        // Simple background removal logic (for demonstration purposes)
        for i in (0..self.image_data.len()).step_by(3) {
            let r = self.image_data[i];
            let g = self.image_data[i + 1];
            let b = self.image_data[i + 2];

            // Assuming white background, change it to transparent
            if r > 200 && g > 200 && b > 200 {
                self.image_data[i] = 0;
                self.image_data[i + 1] = 0;
                self.image_data[i + 2] = 0;
            }
        }

        Ok(())
    }
}
