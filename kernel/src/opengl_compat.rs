extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn opengl_compat_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn opengl_compat_exit() {
}

pub struct OpenGLContext {
    version: String,
    extensions: Vec<String>,
    width: u32,
    height: u32,
    color_depth: u8,
}

impl OpenGLContext {
    pub fn new(version: &str, width: u32, height: u32, color_depth: u8) -> Self {
        OpenGLContext {
            version: String::from(version),
            extensions: Vec::new(),
            width,
            height,
            color_depth,
        }
    }

    pub fn add_extension(&mut self, extension: &str) {
        self.extensions.push(String::from(extension));
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[no_mangle]
pub extern "C" fn opengl_context_new(version: *const u8, width: u32, height: u32, color_depth: u8) -> *mut OpenGLContext {
    let version_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(version, 100)) };
    Box::into_raw(Box::new(OpenGLContext::new(version_str, width, height, color_depth)))
}

#[no_mangle]
pub extern "C" fn opengl_context_add_extension(ctx: *mut OpenGLContext, extension: *const u8) {
    let extension_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(extension, 100)) };
    unsafe { (*ctx).add_extension(extension_str); }
}

#[no_mangle]
pub extern "C" fn opengl_context_get_version(ctx: *const OpenGLContext) -> *const u8 {
    let version = unsafe { &(*ctx).version };
    version.as_ptr()
}

#[no_mangle]
pub extern "C" fn opengl_context_get_extensions(ctx: *const OpenGLContext, extensions: *mut *const u8, count: *mut usize) {
    let exts = unsafe { (*ctx).get_extensions() };
    for (i, ext) in exts.iter().enumerate() {
        unsafe { *extensions.offset(i as isize) = ext.as_ptr(); }
    }
    unsafe { *count = exts.len(); }
}

#[no_mangle]
pub extern "C" fn opengl_context_set_resolution(ctx: *mut OpenGLContext, width: u32, height: u32) {
    unsafe { (*ctx).set_resolution(width, height); }
}

#[no_mangle]
pub extern "C" fn opengl_context_get_resolution(ctx: *const OpenGLContext, width: *mut u32, height: *mut u32) {
    let (w, h) = unsafe { (*ctx).get_resolution() };
    unsafe { *width = w; *height = h; }
}

#[no_mangle]
pub extern "C" fn opengl_context_free(ctx: *mut OpenGLContext) {
    unsafe { drop(Box::from_raw(ctx)); }
}
