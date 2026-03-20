extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut debugger = AgentCodeDebugger::new();
    debugger.set_breakpoint(1);
    debugger.run();
    if debugger.is_stopped_at_breakpoint() {
    }
    let code = debugger.get_code_at_line(1);
}

pub struct AgentCodeDebugger {
    code_lines: Vec<String>,
    breakpoints: Vec<usize>,
    current_line: usize,
}

impl AgentCodeDebugger {
    pub fn new() -> Self {
        AgentCodeDebugger {
            code_lines: Vec::new(),
            breakpoints: Vec::new(),
            current_line: 0,
        }
    }

    pub fn add_code(&mut self, code: &str) {
        self.code_lines.push(String::from(code));
    }

    pub fn set_breakpoint(&mut self, line_number: usize) {
        if line_number < self.code_lines.len() {
            self.breakpoints.push(line_number);
        }
    }

    pub fn run(&mut self) {
        for (i, _) in self.code_lines.iter().enumerate() {
            self.current_line = i;
            if self.breakpoints.contains(&self.current_line) {
                break;
            }
        }
    }

    pub fn is_stopped_at_breakpoint(&self) -> bool {
        self.breakpoints.contains(&self.current_line)
    }

    pub fn get_code_at_line(&self, line_number: usize) -> Option<&str> {
        if line_number < self.code_lines.len() {
            Some(&self.code_lines[line_number])
        } else {
            None
        }
    }
}
