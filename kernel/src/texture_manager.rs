extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TextureManager {
    textures: Vec<Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture: Texture) -> Result<(), String> {
        if self.textures.iter().any(|t| t.id == texture.id) {
            Err(String::from("Texture with this ID already exists"))
        } else {
            self.textures.push(texture);
            Ok(())
        }
    }

    pub fn remove_texture(&mut self, id: u32) -> Result<Texture, String> {
        if let Some(index) = self.textures.iter().position(|t| t.id == id) {
            Ok(self.textures.remove(index))
        } else {
            Err(String::from("Texture not found"))
        }
    }

    pub fn get_texture(&self, id: u32) -> Option<&Texture> {
        self.textures.iter().find(|&t| t.id == id)
    }

    pub fn list_textures(&self) -> Vec<u32> {
        self.textures.iter().map(|t| t.id).collect()
    }

    pub fn update_texture(&mut self, texture: Texture) -> Result<(), String> {
        if let Some(index) = self.textures.iter().position(|t| t.id == texture.id) {
            self.textures[index] = texture;
            Ok(())
        } else {
            Err(String::from("Texture not found"))
        }
    }
}

pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}
