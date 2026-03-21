extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut cursor_highlight = AccessCursorHighlight::new();
    cursor_highlight.initialize();
    loop {
        // Main loop of the module
        cursor_highlight.update_cursor_position(10, 20);
        if cursor_highlight.is_within_bounds(15, 15) {
            cursor_highlight.highlight(true);
        } else {
            cursor_highlight.highlight(false);
        }
        cursor_highlight.log_status();
    }
}

pub struct AccessCursorHighlight {
    cursor_x: usize,
    cursor_y: usize,
    bounds_x: usize,
    bounds_y: usize,
    is_highlighted: bool,
    log_messages: Vec<String>,
}

impl AccessCursorHighlight {
    pub fn new() -> Self {
        AccessCursorHighlight {
            cursor_x: 0,
            cursor_y: 0,
            bounds_x: 100,
            bounds_y: 100,
            is_highlighted: false,
            log_messages: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.log("Module initialized.");
    }

    pub fn update_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.log(String::from("info").", x, y);
    }

    pub fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.bounds_x && y < self.bounds_y
    }

    pub fn highlight(&mut self, enable: bool) {
        self.is_highlighted = enable;
        if enable {
            self.log("Cursor highlighted.");
        } else {
            self.log("Cursor unhighlighted.");
        }
    }

    pub fn log_status(&self) {
        let status = String::from("info"), Highlighted: {}, Bounds: ({}, {}).",
            self.cursor_x, self.cursor_y, self.is_highlighted, self.bounds_x, self.bounds_y
        ;
        self.log(status);
    }

    fn log(&mut self, message: String) {
        self.log_messages.push(message);
    }
}
