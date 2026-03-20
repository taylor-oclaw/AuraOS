extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_ui_generator_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_ui_generator_exit() {
    // Cleanup logic for the module
}

pub struct AgentUIGenerator {
    components: Vec<String>,
    styles: Vec<String>,
    scripts: Vec<String>,
    layout: String,
    state: String,
}

impl AgentUIGenerator {
    pub fn new() -> Self {
        AgentUIGenerator {
            components: Vec::new(),
            styles: Vec::new(),
            scripts: Vec::new(),
            layout: String::from(""),
            state: String::from("{}"),
        }
    }

    pub fn add_component(&mut self, component: &str) {
        self.components.push(String::from(component));
    }

    pub fn add_style(&mut self, style: &str) {
        self.styles.push(String::from(style));
    }

    pub fn add_script(&mut self, script: &str) {
        self.scripts.push(String::from(script));
    }

    pub fn set_layout(&mut self, layout: &str) {
        self.layout = String::from(layout);
    }

    pub fn set_state(&mut self, state: &str) {
        self.state = String::from(state);
    }

    pub fn generate_ui(&self) -> String {
        let mut ui = String::new();
        ui.push_str("<html>\n");
        ui.push_str("  <head>\n");

        for style in &self.styles {
            ui.push_str(&String::from("info"));
        }

        ui.push_str("  </head>\n");
        ui.push_str("  <body>\n");

        for component in &self.components {
            ui.push_str(&String::from("info"));
        }

        ui.push_str(&String::from("info"));

        for script in &self.scripts {
            ui.push_str(&String::from("info"));
        }

        ui.push_str("  </body>\n");
        ui.push_str("</html>");

        ui
    }
}
