extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct StatusItem {
    pub id: u64,
    pub icon: String,
    pub label: String,
    pub value: String,
    pub position: u8,
    pub visible: bool,
    pub clickable: bool,
}

pub struct StatusBar {
    pub items: Vec<StatusItem>,
    pub next_id: u64,
    pub height: u32,
    pub position_top: bool,
    pub clock_format_24h: bool,
}

impl StatusBar {
    pub fn new() -> Self {
        let mut bar = Self {
            items: Vec::new(),
            next_id: 1,
            height: 28,
            position_top: true,
            clock_format_24h: false,
        };
        bar.add_item("clock", "🕐", "12:00", 0, true);
        bar.add_item("wifi", "📶", "Connected", 1, true);
        bar.add_item("battery", "🔋", "100%", 2, true);
        bar.add_item("volume", "🔊", "75%", 3, true);
        bar.add_item("agents", "🤖", "3 active", 4, true);
        bar
    }

    pub fn add_item(&mut self, icon: &str, label: &str, value: &str, pos: u8, clickable: bool) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(StatusItem {
            id,
            icon: String::from(icon),
            label: String::from(label),
            value: String::from(value),
            position: pos,
            visible: true,
            clickable,
        });
        id
    }

    pub fn update_value(&mut self, id: u64, value: &str) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.value = String::from(value);
        }
    }

    pub fn visible_items(&self) -> Vec<&StatusItem> {
        let mut items: Vec<&StatusItem> = self.items.iter().filter(|i| i.visible).collect();
        items.sort_by_key(|i| i.position);
        items
    }
}
