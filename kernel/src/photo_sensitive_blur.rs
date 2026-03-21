extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoSensitiveBlur {
    image_data: Vec<u8>,
    width: usize,
    height: usize,
    blur_radius: usize,
}

impl PhotoSensitiveBlur {
    pub fn new(image_data: Vec<u8>, width: usize, height: usize, blur_radius: usize) -> Self {
        PhotoSensitiveBlur {
            image_data,
            width,
            height,
            blur_radius,
        }
    }

    pub fn get_image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    pub fn set_blur_radius(&mut self, radius: usize) {
        self.blur_radius = radius;
    }

    pub fn apply_gaussian_blur(&mut self) {
        let kernel_size = 2 * self.blur_radius + 1;
        let mut blurred_image = vec![0u8; self.image_data.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;
                let mut count = 0;

                for ky in -self.blur_radius as isize..=self.blur_radius as isize {
                    for kx in -self.blur_radius as isize..=self.blur_radius as isize {
                        let nx = x as isize + kx;
                        let ny = y as isize + ky;

                        if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                            let index = (ny * self.width as isize + nx) as usize * 3;
                            sum_r += self.image_data[index] as i32;
                            sum_g += self.image_data[index + 1] as i32;
                            sum_b += self.image_data[index + 2] as i32;
                            count += 1;
                        }
                    }
                }

                let index = (y * self.width + x) * 3;
                blurred_image[index] = (sum_r / count) as u8;
                blurred_image[index + 1] = (sum_g / count) as u8;
                blurred_image[index + 2] = (sum_b / count) as u8;
            }
        }

        self.image_data = blurred_image;
    }

    pub fn convert_to_grayscale(&mut self) {
        for i in (0..self.image_data.len()).step_by(3) {
            let r = self.image_data[i] as f32;
            let g = self.image_data[i + 1] as f32;
            let b = self.image_data[i + 2] as f32;

            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            self.image_data[i] = gray;
            self.image_data[i + 1] = gray;
            self.image_data[i + 2] = gray;
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let mut resized_image = vec![0u8; new_width * new_height * 3];

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 / new_width as f32) * self.width as f32;
                let src_y = (y as f32 / new_height as f32) * self.height as f32;

                let src_x_floor = src_x.floor() as usize;
                let src_x_ceil = src_x.ceil() as usize;
                let src_y_floor = src_y.floor() as usize;
                let src_y_ceil = src_y.ceil() as usize;

                let x_weight = src_x - src_x_floor as f32;
                let y_weight = src_y - src_y_floor as f32;

                let index = (y * new_width + x) * 3;
                if src_x_floor < self.width && src_y_floor < self.height {
                    let top_left_index = (src_y_floor * self.width + src_x_floor) * 3;
                    resized_image[index] = self.image_data[top_left_index];
                    resized_image[index + 1] = self.image_data[top_left_index + 1];
                    resized_image[index + 2] = self.image_data[top_left_index + 2];
                }

                if src_x_ceil < self.width && src_y_floor < self.height {
                    let top_right_index = (src_y_floor * self.width + src_x_ceil) * 3;
                    resized_image[index] += ((self.image_data[top_right_index] as f32 - self.image_data[top_left_index] as f32) * x_weight) as u8;
                    resized_image[index + 1] += ((self.image_data[top_right_index + 1] as f32 - self.image_data[top_left_index + 1] as f32) * x_weight) as u8;
                    resized_image[index + 2] += ((self.image_data[top_right_index + 2] as f32 - self.image_data[top_left_index + 2] as f32) * x_weight) as u8;
                }

                if src_x_floor < self.width && src_y_ceil < self.height {
                    let bottom_left_index = (src_y_ceil * self.width + src_x_floor) * 3;
                    resized_image[index] += ((self.image_data[bottom_left_index] as f32 - self.image_data[top_left_index] as f32) * y_weight) as u8;
                    resized_image[index + 1] += ((self.image_data[bottom_left_index + 1] as f32 - self.image_data[top_left_index + 1] as f32) * y_weight) as u8;
                    resized_image[index + 2] += ((self.image_data[bottom_left_index + 2] as f32 - self.image_data[top_left_index + 2] as f32) * y_weight) as u8;
                }

                if src_x_ceil < self.width && src_y_ceil < self.height {
                    let bottom_right_index = (src_y_ceil * self.width + src_x_ceil) * 3;
                    resized_image[index] += ((self.image_data[bottom_right_index] as f32 - self.image_data[top_left_index] as f32) * x_weight * y_weight) as u8;
                    resized_image[index + 1] += ((self.image_data[bottom_right_index + 1] as f32 - self.image_data[top_left_index + 1] as f32) * x_weight * y_weight) as u8;
                    resized_image[index + 2] += ((self.image_data[bottom_right_index + 2] as f32 - self.image_data[top_left_index + 2] as f32) * x_weight * y_weight) as u8;
                }
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.image_data = resized_image;
    }
}
