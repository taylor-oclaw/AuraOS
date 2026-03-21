extern crate alloc;
use alloc::string::String;

pub enum PowerState {
    Running,
    Idle,
    Sleeping,
    Hibernating,
    ShuttingDown,
}

pub struct PowerManager {
    pub state: PowerState,
    pub uptime_ticks: u64,
}

impl PowerManager {
    pub fn new() -> Self {
        PowerManager {
            state: PowerState::Running,
            uptime_ticks: 0,
        }
    }

    pub fn idle(&mut self) {
        self.state = PowerState::Idle;
    }

    pub fn sleep(&mut self) {
        self.state = PowerState::Sleeping;
    }

    pub fn wake(&mut self) {
        self.state = PowerState::Running;
    }

    pub fn shutdown(&mut self) {
        self.state = PowerState::ShuttingDown;
    }

    pub fn tick(&mut self) {
        self.uptime_ticks += 1;
    }

    pub fn current_state(&self) -> &PowerState {
        &self.state
    }

    pub unsafe fn acpi_shutdown() {
        core::arch::asm!("out dx, al", in("dx") 0x604u16, in("al") 0x00u8);
    }
}
