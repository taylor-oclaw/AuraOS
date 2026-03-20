extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RenderPipeline {
    shaders: Vec<String>,
    buffers: Vec<u8>,
    textures: Vec<u32>,
    framebuffers: Vec<u64>,
    render_targets: Vec<u32>,
}

impl RenderPipeline {
    pub fn new() -> Self {
        RenderPipeline {
            shaders: Vec::new(),
            buffers: Vec::new(),
            textures: Vec::new(),
            framebuffers: Vec::new(),
            render_targets: Vec::new(),
        }
    }

    pub fn add_shader(&mut self, shader_code: &str) {
        let shader = String::from(shader_code);
        self.shaders.push(shader);
    }

    pub fn create_buffer(&mut self, size: usize) -> u64 {
        let buffer_id = self.buffers.len() as u64;
        self.buffers.resize(self.buffers.len() + size, 0);
        buffer_id
    }

    pub fn upload_texture(&mut self, texture_data: &[u8]) -> u32 {
        let texture_id = self.textures.len() as u32;
        self.textures.extend_from_slice(texture_data);
        texture_id
    }

    pub fn create_framebuffer(&mut self) -> u64 {
        let framebuffer_id = self.framebuffers.len() as u64;
        self.framebuffers.push(framebuffer_id);
        framebuffer_id
    }

    pub fn set_render_target(&mut self, target: u32) {
        if self.render_targets.contains(&target) {
            return;
        }
        self.render_targets.push(target);
    }
}
