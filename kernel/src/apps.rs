extern crate alloc;
use alloc::boxed::Box;
use alloc::format;

use alloc::vec::Vec;
use alloc::string::String;


pub trait AuraApp {
    fn name(&self) -> &str;
    fn on_create(&mut self);
    fn on_key(&mut self, key: char);
    fn on_mouse(&mut self, x: i32, y: i32, button: u8);
    fn on_render(&self, buf: &mut Vec<String>);
    fn on_destroy(&mut self);
}

pub struct AppManager {
    apps: Vec<Box<dyn AuraApp>>,
    active_app: Option<usize>,
}

impl AppManager {
    pub fn new() -> Self {
        AppManager {
            apps: Vec::new(),
            active_app: None,
        }
    }

    pub fn register(&mut self, app: Box<dyn AuraApp>) {
        self.apps.push(app);
    }

    pub fn launch(&mut self, name: &str) {
        if let Some(index) = self.apps.iter().position(|app| app.name() == name) {
            self.active_app = Some(index);
            self.apps[index].on_create();
        }
    }

    pub fn close_active(&mut self) {
        if let Some(index) = self.active_app {
            self.apps[index].on_destroy();
            self.active_app = None;
        }
    }

    pub fn route_key(&mut self, key: char) {
        if let Some(index) = self.active_app {
            self.apps[index].on_key(key);
        }
    }

    pub fn route_mouse(&mut self, x: i32, y: i32, btn: u8) {
        if let Some(index) = self.active_app {
            self.apps[index].on_mouse(x, y, btn);
        }
    }

    pub fn render_active(&self) -> Vec<String> {
        let mut buf = Vec::new();
        if let Some(index) = self.active_app {
            self.apps[index].on_render(&mut buf);
        }
        buf
    }
}

struct Calculator {
    input: String,
    result: Option<f64>,
}

impl AuraApp for Calculator {
    fn name(&self) -> &str { "Calculator" }

    fn on_create(&mut self) {}

    fn on_key(&mut self, key: char) {
        match key {
            'c' => self.input.clear(),
            '=' => self.result = self.input.parse::<f64>().ok().and_then(|_| eval(&self.input)),
            _ if key.is_digit(10) || key == '+' || key == '-' || key == '*' || key == '/' => self.input.push(key),
            _ => (),
        }
    }

    fn on_mouse(&mut self, _: i32, _: i32, _: u8) {}

    fn on_render(&self, buf: &mut Vec<String>) {
        buf.push(format!("Input: {}", self.input));
        if let Some(result) = self.result {
            buf.push(format!("Result: {}", result));
        }
    }

    fn on_destroy(&mut self) {}
}

struct Notes {
    content: String,
}

impl AuraApp for Notes {
    fn name(&self) -> &str { "Notes" }

    fn on_create(&mut self) {}

    fn on_key(&mut self, key: char) {
        if key == '\x08' && !self.content.is_empty() {
            self.content.pop();
        } else if !key.is_control() {
            self.content.push(key);
        }
    }

    fn on_mouse(&mut self, _: i32, _: i32, _: u8) {}

    fn on_render(&self, buf: &mut Vec<String>) {
        buf.push(String::from("Notes:"));
        for line in self.content.split('\n') {
            buf.push(String::from(line));
        }
    }

    fn on_destroy(&mut self) {}
}

struct Clock;

impl AuraApp for Clock {
    fn name(&self) -> &str { "Clock" }

    fn on_create(&mut self) {}

    fn on_key(&mut self, _: char) {}

    fn on_mouse(&mut self, _: i32, _: i32, _: u8) {}

    fn on_render(&self, buf: &mut Vec<String>) {
        let now = core::time::Duration::from_secs(1609459200 + (unsafe { core::arch::x86_64::_rdtsc() } / 3_000_000) as u64); // Example timestamp
        buf.push(format!("Time: {:02}:{:02}:{:02}", now.as_secs() % 86400 / 3600, (now.as_secs() % 3600) / 60, now.as_secs() % 60));
    }

    fn on_destroy(&mut self) {}
}

fn eval(expr: &str) -> Option<f64> {
    let mut chars = expr.chars().peekable();
    parse_expr(&mut chars)
}

fn parse_expr(chars: &mut core::iter::Peekable<core::str::Chars>) -> Option<f64> {
    let mut lhs = parse_term(chars)?;
    while let Some(op) = chars.next_if(|&c| c == '+' || c == '-') {
        let rhs = parse_term(chars)?;
        lhs = match op {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            _ => unreachable!(),
        };
    }
    Some(lhs)
}

fn parse_term(chars: &mut core::iter::Peekable<core::str::Chars>) -> Option<f64> {
    let mut lhs = parse_factor(chars)?;
    while let Some(op) = chars.next_if(|&c| c == '*' || c == '/') {
        let rhs = parse_factor(chars)?;
        lhs = match op {
            '*' => lhs * rhs,
            '/' => lhs / rhs,
            _ => unreachable!(),
        };
    }
    Some(lhs)
}

fn parse_factor(chars: &mut core::iter::Peekable<core::str::Chars>) -> Option<f64> {
    if let Some('(') = chars.next_if(|&c| c == '(') {
        let expr = parse_expr(chars)?;
        chars.next_if_eq(&')');
        Some(expr)
    } else {
        let mut num_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                chars.next();
            } else {
                break;
            }
        }
        num_str.parse().ok()
    }
}