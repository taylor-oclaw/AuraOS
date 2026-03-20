extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct AuraCodeEditor {
    content: String,
    cursor_position: usize,
    history: Vec<String>,
}

impl AuraCodeEditor {
    pub fn new() -> Self {
        AuraCodeEditor {
            content: String::new(),
            cursor_position: 0,
            history: Vec::new(),
        }
    }

    pub fn insert(&mut self, text: &str) {
        let (left, right) = self.content.split_at(self.cursor_position);
        self.content = String::from("info");
        self.cursor_position += text.len();
    }

    pub fn delete(&mut self) {
        if self.cursor_position > 0 {
            self.content.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

pub extern "C" fn aura_code_editor_new() -> *mut AuraCodeEditor {
    Box::into_raw(Box::new(AuraCodeEditor::new()))
}

pub extern "C" fn aura_code_editor_insert(editor: *mut AuraCodeEditor, text: *const u8, len: usize) {
    unsafe {
        let editor = &mut *editor;
        let slice = core::slice::from_raw_parts(text, len);
        if let Ok(s) = core::str::from_utf8(slice) {
            editor.insert(s);
        }
    }
}

pub extern "C" fn aura_code_editor_delete(editor: *mut AuraCodeEditor) {
    unsafe {
        (*editor).delete();
    }
}

pub extern "C" fn aura_code_editor_move_cursor_left(editor: *mut AuraCodeEditor) {
    unsafe {
        (*editor).move_cursor_left();
    }
}

pub extern "C" fn aura_code_editor_move_cursor_right(editor: *mut AuraCodeEditor) {
    unsafe {
        (*editor).move_cursor_right();
    }
}

pub extern "C" fn aura_code_editor_get_content(editor: *const AuraCodeEditor, buffer: *mut u8, len: usize) -> usize {
    unsafe {
        let editor = &*editor;
        let content = editor.get_content();
        let bytes_to_copy = core::cmp::min(content.len(), len);
        core::ptr::copy_nonoverlapping(content.as_ptr(), buffer, bytes_to_copy);
        bytes_to_copy
    }
}

pub extern "C" fn aura_code_editor_free(editor: *mut AuraCodeEditor) {
    unsafe {
        drop(Box::from_raw(editor));
    }
}
