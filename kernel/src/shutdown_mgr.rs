extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ShutdownAction {
    Shutdown,
    Restart,
    Sleep,
    Hibernate,
    Logout,
}

pub enum ShutdownState {
    Running,
    Preparing(ShutdownAction),
    SavingState,
    ClosingApps(usize),
    Finalizing,
    Done,
}

pub struct ShutdownManager {
    pub state: ShutdownState,
    pub pending_saves: Vec<String>,
    pub force: bool,
    pub countdown_secs: u8,
}

impl ShutdownManager {
    pub fn new() -> Self {
        Self {
            state: ShutdownState::Running,
            pending_saves: Vec::new(),
            force: false,
            countdown_secs: 0,
        }
    }

    pub fn initiate(&mut self, action: ShutdownAction) {
        self.state = ShutdownState::Preparing(action);
        self.countdown_secs = 5;
    }

    pub fn add_pending_save(&mut self, app: &str) {
        self.pending_saves.push(String::from(app));
    }

    pub fn save_complete(&mut self, app: &str) {
        self.pending_saves.retain(|a| a != app);
        if self.pending_saves.is_empty() {
            self.state = ShutdownState::Finalizing;
        }
    }

    pub fn force_shutdown(&mut self) {
        self.force = true;
        self.pending_saves.clear();
        self.state = ShutdownState::Finalizing;
    }

    pub fn tick(&mut self) {
        if self.countdown_secs > 0 {
            self.countdown_secs -= 1;
        }
        if self.countdown_secs == 0 {
            match &self.state {
                ShutdownState::Preparing(_) => self.state = ShutdownState::SavingState,
                ShutdownState::SavingState => {
                    if self.pending_saves.is_empty() {
                        self.state = ShutdownState::Finalizing;
                    }
                }
                ShutdownState::Finalizing => self.state = ShutdownState::Done,
                _ => {}
            }
        }
    }

    pub fn is_shutting_down(&self) -> bool {
        !matches!(self.state, ShutdownState::Running)
    }

    pub fn is_done(&self) -> bool {
        matches!(self.state, ShutdownState::Done)
    }
}
