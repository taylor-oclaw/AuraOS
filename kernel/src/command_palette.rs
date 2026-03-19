extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PaletteCommand {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub shortcut: Option<String>,
    pub category: String,
    pub usage_count: u64
}

pub struct CommandPalette {
    pub commands: Vec<PaletteCommand>,
    pub visible: bool,
    pub query: String,
    pub selected: usize,
    pub next_id: u64
}

impl CommandPalette {
    pub fn new() -> Self {
        let mut p = Self {
            commands: Vec::new(),
            visible: false,
            query: String::new(),
            selected: 0,
            next_id: 1
        };
        p.register("Open File", "Open a file or folder", Some("Cmd+O"), "file");
        p.register("Save", "Save current document", Some("Cmd+S"), "file");
        p.register("Search", "Search everything", Some("Cmd+F"), "search");
        p.register("Settings", "Open settings", Some("Cmd+,"), "system");
        p.register("Terminal", "Open raw terminal", Some("Cmd+T"), "system");
        p.register("Lock Screen", "Lock the screen", Some("Cmd+L"), "security");
        p.register("Screenshot", "Take screenshot", Some("Cmd+Shift+3"), "capture");
        p.register("New Agent", "Spawn a new agent", None, "agent");
        p.register("Agent Status", "View running agents", None, "agent");
        p
    }

    fn register(&mut self, name: &str, desc: &str, shortcut: Option<&str>, cat: &str) {
        let id = self.next_id;
        self.next_id += 1;
        self.commands.push(PaletteCommand {
            id,
            name: String::from(name),
            description: String::from(desc),
            shortcut: shortcut.map(String::from),
            category: String::from(cat),
            usage_count: 0
        });
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.query = String::new();
        self.selected = 0;
    }

    pub fn search(&self, query: &str) -> Vec<&PaletteCommand> {
        let q = query.to_lowercase();
        self.commands.iter().filter(|c| c.name.to_lowercase().contains(&q) || c.description.to_lowercase().contains(&q)).collect()
    }

    pub fn execute(&mut self, id: u64) {
        if let Some(cmd) = self.commands.iter_mut().find(|c| c.id == id) {
            cmd.usage_count += 1;
        }
        self.visible = false;
    }
}
