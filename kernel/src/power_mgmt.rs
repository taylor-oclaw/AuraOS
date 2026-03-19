#![no_std]

extern crate alloc;

use x86_64::instructions::{hlt, port::Port};

const KEYBOARD_CONTROLLER_PORT: u16 = 0x64;
const KEYBOARD_RESET_COMMAND: u8 = 0xFE;

const QEMU_DEBUG_EXIT_PORT: u16 = 0x604;
const QEMU_DEBUG_EXIT_VALUE: u16 = 0x2000;

const ACPI_PM1A_CNT_PORT: u16 = 0x604;
const ACPI_SLP_TYP_A_S5: u16 = 5 << 10;
const ACPI_SLP_EN: u16 = 1 << 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Running,
    Sleep,
    Hibernate,
    ShuttingDown,
    Rebooting,
}

pub struct PowerManager {
    current_state: PowerState,
    uptime_ticks: u64,
}

impl PowerManager {
    pub const fn new() -> Self {
        Self {
            current_state: PowerState::Running,
            uptime_ticks: 0,
        }
    }

    pub fn tick(&mut self) {
        self.uptime_ticks = self.uptime_ticks.wrapping_add(1);
    }

    pub fn request_reboot(&mut self) {
        self.current_state = PowerState::Rebooting;
    }

    pub fn request_shutdown(&mut self) {
        self.current_state = PowerState::ShuttingDown;
    }

    pub const fn state(&self) -> PowerState {
        self.current_state
    }

    pub const fn uptime(&self) -> u64 {
        self.uptime_ticks
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn reboot() -> ! {
    unsafe {
        let mut port = Port::<u8>::new(KEYBOARD_CONTROLLER_PORT);
        port.write(KEYBOARD_RESET_COMMAND);
    }

    loop {
        hlt();
    }
}

pub fn shutdown_qemu() -> ! {
    unsafe {
        let mut port = Port::<u16>::new(QEMU_DEBUG_EXIT_PORT);
        port.write(QEMU_DEBUG_EXIT_VALUE);
    }

    loop {
        hlt();
    }
}

pub fn shutdown_acpi() -> ! {
    unsafe {
        let mut port = Port::<u16>::new(ACPI_PM1A_CNT_PORT);
        port.write(ACPI_SLP_TYP_A_S5 | ACPI_SLP_EN);
    }

    loop {
        hlt();
    }
}
