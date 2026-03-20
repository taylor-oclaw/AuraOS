//! Input Router — routes keyboard/mouse to the right surface

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Keyboard input state for the active terminal
pub struct InputState {
    pub buffer: [u8; 256],
    pub cursor: usize,
    pub history: Vec<String>,
    pub output_lines: Vec<String>,
    pub max_output: usize,
}

impl InputState {
    pub fn new() -> Self {
        let mut state = InputState {
            buffer: [0u8; 256],
            cursor: 0,
            history: Vec::new(),
            output_lines: Vec::new(),
            max_output: 20,
        };
        // Initial greeting
        state.output_lines.push(String::from("AuraOS Terminal v0.1"));
        state.output_lines.push(String::from(""));
        state
    }

    pub fn handle_char(&mut self, c: char) {
        match c {
            '\n' | '\r' => {
                // Execute command
                let cmd_len = self.cursor;
                let cmd = core::str::from_utf8(&self.buffer[..cmd_len])
                    .unwrap_or("")
                    .trim();
                
                if !cmd.is_empty() {
                    // Add to output
                    let mut line = String::from("you> ");
                    line.push_str(cmd);
                    self.output_lines.push(line);
                    
                    // Process through NLP
                    let response = process_input(cmd);
                    for resp_line in response {
                        self.output_lines.push(resp_line);
                    }
                    self.output_lines.push(String::new());
                    
                    // Trim output if too long
                    while self.output_lines.len() > self.max_output {
                        self.output_lines.remove(0);
                    }
                }
                
                self.cursor = 0;
            }
            '\u{8}' => {
                // Backspace
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            c if c.is_ascii() && c as u32 >= 0x20 => {
                if self.cursor < 255 {
                    self.buffer[self.cursor] = c as u8;
                    self.cursor += 1;
                }
            }
            _ => {}
        }
    }

    /// Get the current input line
    pub fn current_input(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.cursor]).unwrap_or("")
    }

    /// Get display lines (output + current input)
    pub fn display_lines(&self) -> Vec<String> {
        let mut lines = self.output_lines.clone();
        let mut input_line = String::from("you> ");
        input_line.push_str(self.current_input());
        input_line.push('_');
        lines.push(input_line);
        lines
    }
}

fn process_input(input: &str) -> Vec<String> {
    let intent = crate::nlp::parse_intent(input);
    let mut lines = Vec::new();
    
    match intent {
        crate::nlp::Intent::Greeting => {
            lines.push(String::from("Hey! I'm Aura, your OS companion."));
            lines.push(String::from("What can I help you with?"));
        }
        crate::nlp::Intent::WhoAreYou => {
            lines.push(String::from("I'm Aura - the intelligence inside AuraOS."));
            lines.push(String::from("I'm not an app. I AM the operating system."));
        }
        crate::nlp::Intent::WhoMadeYou => {
            lines.push(String::from("Created by Venkat Yarlagadda"));
            lines.push(String::from("Built by Taylor Oclaw"));
            lines.push(String::from("Patent pending - Suvayar LLC / RedSky LLC"));
        }
        crate::nlp::Intent::TimeQuery => {
            let dt = crate::rtc::read_local_time();
            let tz = crate::rtc::timezone_name();
            let (h12, ampm) = if dt.hour == 0 { (12, "AM") }
                else if dt.hour < 12 { (dt.hour, "AM") }
                else if dt.hour == 12 { (12, "PM") }
                else { (dt.hour - 12, "PM") };
            lines.push(alloc::format!("It's {}:{:02} {} {} on {} {} {}, {}",
                h12, dt.minute, ampm, tz,
                dt.weekday_name(), dt.month_name(), dt.day, dt.year));
        }
        crate::nlp::Intent::SystemInfo => {
            lines.push(String::from("CPU: x86_64 (AuthenticAMD in QEMU)"));
            lines.push(String::from("RAM: 506 MB usable"));
            lines.push(String::from("Display: 1280x720 BGR framebuffer"));
            lines.push(String::from("Keyboard: PS/2 (active)"));
        }
        crate::nlp::Intent::Help => {
            lines.push(String::from("Just talk to me naturally!"));
            lines.push(String::from("Try: what time is it, who are you"));
            lines.push(String::from("     tell me a joke, what computer"));
        }
        crate::nlp::Intent::Joke => {
            use core::sync::atomic::{AtomicU8, Ordering};
            static IDX: AtomicU8 = AtomicU8::new(0);
            let i = IDX.fetch_add(1, Ordering::Relaxed) % 4;
            match i {
                0 => lines.push(String::from("Why do programmers prefer dark mode? Light attracts bugs.")),
                1 => lines.push(String::from("I'd tell you a UDP joke... but you might not get it.")),
                2 => lines.push(String::from("Why did the OS break up with the app? Too many unresolved dependencies.")),
                _ => lines.push(String::from("There are 10 types of people: those who understand binary, and those who don't.")),
            }
        }
        crate::nlp::Intent::ClearScreen => {
            // Will be handled by clearing output_lines
            lines.push(String::from("[screen cleared]"));
        }
        _ => {
            lines.push(String::from("I'm not sure what you mean."));
            lines.push(String::from("My LLM brain is loading soon!"));
        }
    }
    
    lines
}

/// Global input state
static INPUT: spin::Lazy<spin::Mutex<InputState>> = spin::Lazy::new(|| {
    spin::Mutex::new(InputState::new())
});

pub fn handle_key(c: char) {
    INPUT.lock().handle_char(c);
}

pub fn display_lines() -> Vec<String> {
    INPUT.lock().display_lines()
}
