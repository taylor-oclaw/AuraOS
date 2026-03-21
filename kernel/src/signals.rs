extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Clone, Copy, PartialEq)]
enum Signal {
    SIGTERM,
    SIGKILL,
    SIGSTOP,
    SIGCONT,
    SIGUSR1,
    SIGUSR2,
    SIGALRM,
    SIGCHLD,
}

#[derive(Clone, Copy, PartialEq)]
enum SignalAction {
    Ignore,
    Default,
    Custom(u64),
}

struct SignalHandler {
    pid: u32,
    handlers: Vec<(Signal, SignalAction)>,
}

struct SignalManager {
    handlers: Vec<SignalHandler>,
    pending: Vec<(u32, Signal)>,
}

impl SignalManager {
    fn new() -> Self {
        SignalManager {
            handlers: Vec::new(),
            pending: Vec::new(),
        }
    }

    fn register_handler(&mut self, pid: u32, signal: Signal, action: SignalAction) {
        if let Some(handler) = self.handlers.iter_mut().find(|h| h.pid == pid) {
            handler.handlers.push((signal, action));
        } else {
            self.handlers.push(SignalHandler {
                pid,
                handlers: vec![(signal, action)],
            });
        }
    }

    fn send_signal(&mut self, pid: u32, signal: Signal) -> bool {
        if self.handlers.iter().any(|h| h.pid == pid) {
            self.pending.push((pid, signal));
            true
        } else {
            false
        }
    }

    fn pending_signals(&self, pid: u32) -> Vec<Signal> {
        self.pending.iter()
            .filter_map(|&(p, s)| if p == pid { Some(s) } else { None })
            .collect()
    }

    fn clear_pending(&mut self, pid: u32) {
        self.pending.retain(|&(p, _)| p != pid);
    }
}