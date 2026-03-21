extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileUIWidgetsAdjust {
    widgets: Vec<String>,
    selected_widget_index: usize,
}

impl ProfileUIWidgetsAdjust {
    pub fn new(widgets: Vec<String>) -> Self {
        ProfileUIWidgetsAdjust {
            widgets,
            selected_widget_index: 0,
        }
    }

    pub fn add_widget(&mut self, widget_name: String) {
        self.widgets.push(widget_name);
    }

    pub fn remove_widget(&mut self, index: usize) {
        if index < self.widgets.len() {
            self.widgets.remove(index);
            if self.selected_widget_index >= self.widgets.len() {
                self.selected_widget_index = 0;
            }
        }
    }

    pub fn select_next_widget(&mut self) {
        if !self.widgets.is_empty() {
            self.selected_widget_index = (self.selected_widget_index + 1) % self.widgets.len();
        }
    }

    pub fn select_previous_widget(&mut self) {
        if !self.widgets.is_empty() {
            self.selected_widget_index = if self.selected_widget_index == 0 {
                self.widgets.len() - 1
            } else {
                self.selected_widget_index - 1
            };
        }
    }

    pub fn get_selected_widget_name(&self) -> Option<&String> {
        self.widgets.get(self.selected_widget_index)
    }
}
