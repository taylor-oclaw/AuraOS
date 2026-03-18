#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod serial;
mod vga;
mod gdt;
mod interrupts;
mod memory;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Initialize hardware
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // Clear screen and show banner
    vga::clear_screen();
    vga::print_banner();

    crate::serial_println!("[kernel] AuraOS v0.1.0 booting...");
    crate::serial_println!("[kernel] GDT initialized");
    crate::serial_println!("[kernel] IDT initialized");
    crate::serial_println!("[kernel] PIC initialized, interrupts enabled");

    // Memory info
    let mem_regions = boot_info.memory_regions.len();
    let usable_mem = memory::total_usable_memory(&boot_info.memory_regions);
    crate::serial_println!("[kernel] Memory regions: {}", mem_regions);
    crate::serial_println!("[kernel] Usable memory: {} MB", usable_mem / (1024 * 1024));

    crate::println!("");
    vga::set_color(vga::Color::LightGreen, vga::Color::Black);
    crate::println!("  AuraOS v0.1.0 - The Ambient Intelligence OS");
    vga::set_color(vga::Color::LightCyan, vga::Color::Black);
    crate::println!("");
    crate::println!("  [ok] GDT initialized");
    crate::println!("  [ok] IDT initialized");
    crate::println!("  [ok] PIC initialized");
    crate::println!("  [ok] Interrupts enabled");
    crate::println!("  [ok] Memory: {} MB usable ({} regions)", usable_mem / (1024 * 1024), mem_regions);
    crate::println!("");
    vga::set_color(vga::Color::White, vga::Color::Black);
    crate::println!("  Type something... (keyboard input active)");
    crate::println!("");
    vga::set_color(vga::Color::LightCyan, vga::Color::Black);
    crate::print!("  aura> ");

    // Main loop — halt until interrupt
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::serial_println!("KERNEL PANIC: {}", info);
    vga::set_color(vga::Color::LightRed, vga::Color::Black);
    crate::println!("\n  !! KERNEL PANIC !!");
    crate::println!("  {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
