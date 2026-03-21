extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SignalHandler {
    handlers: Vec<Option<fn(i32)>>,
}

impl SignalHandler {
    pub fn new() -> Self {
        SignalHandler {
            handlers: vec![None; 64], // Assuming 64 possible signals
        }
    }

    pub fn register_handler(&mut self, signal: i32, handler: fn(i32)) -> Result<(), &'static str> {
        if (0..self.handlers.len() as i32).contains(&signal) {
            self.handlers[signal as usize] = Some(handler);
            Ok(())
        } else {
            Err("Invalid signal number")
        }
    }

    pub fn unregister_handler(&mut self, signal: i32) -> Result<(), &'static str> {
        if (0..self.handlers.len() as i32).contains(&signal) {
            self.handlers[signal as usize] = None;
            Ok(())
        } else {
            Err("Invalid signal number")
        }
    }

    pub fn handle_signal(&mut self, signal: i32) -> Result<(), &'static str> {
        if (0..self.handlers.len() as i32).contains(&signal) {
            if let Some(handler) = self.handlers[signal as usize] {
                handler(signal);
                Ok(())
            } else {
                Err("No handler registered for this signal")
            }
        } else {
            Err("Invalid signal number")
        }
    }

    pub fn list_handlers(&self) -> Vec<(i32, Option<String>)> {
        self.handlers
            .iter()
            .enumerate()
            .map(|(index, handler)| {
                let name = match index {
                    1 => Some(String::from("SIGHUP")),
                    2 => Some(String::from("SIGINT")),
                    3 => Some(String::from("SIGQUIT")),
                    4 => Some(String::from("SIGILL")),
                    5 => Some(String::from("SIGTRAP")),
                    6 => Some(String::from("SIGABRT")),
                    7 => Some(String::from("SIGBUS")),
                    8 => Some(String::from("SIGFPE")),
                    9 => Some(String::from("SIGKILL")),
                    10 => Some(String::from("SIGUSR1")),
                    11 => Some(String::from("SIGSEGV")),
                    12 => Some(String::from("SIGUSR2")),
                    13 => Some(String::from("SIGPIPE")),
                    14 => Some(String::from("SIGALRM")),
                    15 => Some(String::from("SIGTERM")),
                    16 => Some(String::from("SIGSTKFLT")),
                    17 => Some(String::from("SIGCHLD")),
                    18 => Some(String::from("SIGCONT")),
                    19 => Some(String::from("SIGSTOP")),
                    20 => Some(String::from("SIGTSTP")),
                    21 => Some(String::from("SIGTTIN")),
                    22 => Some(String::from("SIGTTOU")),
                    23 => Some(String::from("SIGURG")),
                    24 => Some(String::from("SIGXCPU")),
                    25 => Some(String::from("SIGXFSZ")),
                    26 => Some(String::from("SIGVTALRM")),
                    27 => Some(String::from("SIGPROF")),
                    28 => Some(String::from("SIGWINCH")),
                    29 => Some(String::from("SIGIO")),
                    30 => Some(String::from("SIGPWR")),
                    31 => Some(String::from("SIGSYS")),
                    _ => None,
                };
                (index as i32, name)
            })
            .collect()
    }
}
