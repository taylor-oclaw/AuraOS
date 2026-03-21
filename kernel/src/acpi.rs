//! ACPI Power Management
//! Handles shutdown and reboot.

/// Reboot via keyboard controller
pub fn reboot() {
    unsafe {
        // Send reset command to keyboard controller
        let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
        port.write(0xFE);
    }
    // If that didn't work, triple fault
    loop { x86_64::instructions::hlt(); }
}

/// Shutdown via QEMU-specific port (for testing)
/// On real hardware, needs full ACPI table parsing
pub fn shutdown() {
    unsafe {
        // QEMU shutdown port
        let mut port = x86_64::instructions::port::Port::<u16>::new(0x604);
        port.write(0x2000);
    }
    // Bochs/old QEMU
    unsafe {
        let mut port = x86_64::instructions::port::Port::<u16>::new(0xB004);
        port.write(0x2000);
    }
    // If nothing works, halt
    loop { x86_64::instructions::hlt(); }
}
