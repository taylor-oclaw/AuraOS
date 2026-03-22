extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BrowserAutoTabClose {
    tab_close_delay: u64,
}

impl BrowserAutoTabClose {
    pub fn new(tab_close_delay: u64) -> Self {
        BrowserAutoTabClose { tab_close_delay }
    }

    pub fn get_tab_close_delay(&self) -> u64 {
        self.tab_close_delay
    }

    pub fn set_tab_close_delay(&mut self, delay: u64) {
        self.tab_close_delay = delay;
    }

    pub fn close_tabs(&self, tabs: Vec<String>) {
        for tab in tabs.iter() {
            println!("Closing tab: {}", tab);
        }
    }

    pub fn get_active_tab(&self, tabs: Vec<String>) -> Option<&String> {
        if let Some(tab) = tabs.last() {
            Some(tab)
        } else {
            None
        }
    }

    pub fn add_new_tab(&mut self, new_tab: String, tabs: &mut Vec<String>) {
        tabs.push(new_tab);
    }
}

pub fn main() {
    let mut browser = BrowserAutoTabClose::new(30); // 30 seconds delay

    let mut tabs = Vec::new();
    tabs.push(String::from("Tab1"));
    tabs.push(String::from("Tab2"));

    println!("Active tab: {:?}", browser.get_active_tab(tabs.clone()));

    browser.close_tabs(tabs.clone());

    browser.add_new_tab(String::from("New Tab"), &mut tabs);

    println!("Updated active tab: {:?}", browser.get_active_tab(tabs));
}